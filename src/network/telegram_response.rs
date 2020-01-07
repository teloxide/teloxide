use reqwest::StatusCode;

use crate::{
    requests::ResponseResult,
    types::{False, ResponseParameters, True},
    ApiErrorKind, RequestError,
};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TelegramResponse<R> {
    Ok {
        /// A dummy field. Used only for deserialization.
        #[allow(dead_code)]
        ok: True,

        result: R,
    },
    Err {
        /// A dummy field. Used only for deserialization.
        #[allow(dead_code)]
        ok: False,

        #[serde(rename = "description")]
        kind: ApiErrorKind,
        error_code: u16,
        response_parameters: Option<ResponseParameters>,
    },
}

impl<R> Into<ResponseResult<R>> for TelegramResponse<R> {
    fn into(self) -> Result<R, RequestError> {
        match self {
            TelegramResponse::Ok { result, .. } => Ok(result),
            TelegramResponse::Err {
                kind,
                error_code,
                response_parameters,
                ..
            } => {
                if let Some(params) = response_parameters {
                    match params {
                        ResponseParameters::RetryAfter(i) => {
                            Err(RequestError::RetryAfter(i))
                        }
                        ResponseParameters::MigrateToChatId(to) => {
                            Err(RequestError::MigrateToChatId(to))
                        }
                    }
                } else {
                    Err(RequestError::ApiError {
                        kind,
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
    use crate::types::Update;

    #[test]
    fn terminated_by_other_get_updates() {
        let expected = ApiErrorKind::TerminatedByOtherGetUpdates;
        if let TelegramResponse::Err{ ok, kind, error_code, response_parameters } = serde_json::from_str::<TelegramResponse<Update>>(r#"{"ok":false,"error_code":409,"description":"Conflict: terminated by other getUpdates request; make sure that only one bot instance is running"}"#).unwrap() {
            assert_eq!(expected, kind);
        }
        else {
            panic!("Этой херни здесь не должно быть");
        }
    }
}
