use reqwest::StatusCode;

use crate::{
    requests::ResponseResult, types::ResponseParameters, RequestError,
};

#[derive(Deserialize)]
#[serde(untagged)]
enum TelegramResponse<R> {
    Ok {
        /// A dummy field. Used only for deserialization.
        #[allow(dead_code)]
        ok: bool, // TODO: True type

        result: R,
    },
    Err {
        /// A dummy field. Used only for deserialization.
        #[allow(dead_code)]
        ok: bool, // TODO: False type

        description: String,
        error_code: u16,
        response_parameters: Option<ResponseParameters>,
    },
}

impl<R> Into<ResponseResult<R>> for TelegramResponse<R> {
    fn into(self) -> Result<R, RequestError> {
        match self {
            TelegramResponse::Ok { result, .. } => Ok(result),
            TelegramResponse::Err {
                description,
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
                        description,
                        status_code: StatusCode::from_u16(error_code).unwrap(),
                    })
                }
            }
        }
    }
}
