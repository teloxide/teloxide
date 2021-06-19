use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    requests::ResponseResult,
    types::{False, ResponseParameters, True},
    ApiError, RequestError,
};

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum TelegramResponse<R> {
    Ok {
        /// A dummy field. Used only for deserialization.
        #[allow(dead_code)]
        ok: True,
        #[serde(rename = "result")]
        response: R,
    },
    Err {
        /// A dummy field. Used only for deserialization.
        #[allow(dead_code)]
        ok: False,

        #[serde(rename = "description")]
        error: ApiError,
        error_code: u16,
        #[serde(rename = "parameters")]
        response_parameters: Option<ResponseParameters>,
    },
}

impl<R> From<TelegramResponse<R>> for ResponseResult<R> {
    fn from(this: TelegramResponse<R>) -> ResponseResult<R> {
        match this {
            TelegramResponse::Ok { response, .. } => Ok(response),
            TelegramResponse::Err {
                error,
                error_code,
                response_parameters,
                ..
            } => {
                if let Some(params) = response_parameters {
                    match params {
                        ResponseParameters::RetryAfter(i) => Err(RequestError::RetryAfter(i)),
                        ResponseParameters::MigrateToChatId(to) => {
                            Err(RequestError::MigrateToChatId(to))
                        }
                    }
                } else {
                    Err(RequestError::ApiError {
                        kind: error,
                        status_code: StatusCode::from_u16(error_code).unwrap(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{errors::ApiError, types::Update};

    #[test]
    fn parse_terminated_by_other_get_updates() {
        let s = r#"{"ok":false,"error_code":409,"description":"Conflict: terminated by other getUpdates request; make sure that only one bot instance is running"}"#;
        let val = serde_json::from_str::<TelegramResponse<Update>>(s).unwrap();

        assert!(matches!(
            val,
            TelegramResponse::Err {
                error: ApiError::TerminatedByOtherGetUpdates,
                ..
            }
        ));
    }

    #[test]
    fn parse_unknown() {
        let s = r#"{"ok":false,"error_code":111,"description":"Unknown description that won't match anything"}"#;
        let val = serde_json::from_str::<TelegramResponse<Update>>(s).unwrap();

        assert!(
            matches!(val, TelegramResponse::Err { error: ApiError::Unknown(s), .. } if s == "Unknown description that won't match anything")
        );
    }
}
