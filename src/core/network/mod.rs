use crate::core::{
    requests::{RequestError, ResponseResult},
    types::ResponseParameters,
};

use apply::Apply;
use reqwest::{
    r#async::{multipart::Form, Client},
    StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

/// Creates URL for making HTTPS requests. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#making-requests
fn method_url(base: &str, token: &str, method_name: &str) -> String {
    format!(
        "{url}/bot{token}/{method}",
        url = base,
        token = token,
        method = method_name,
    )
}

/// Creates URL for downloading a file. See the [Telegram documentation].
///
/// [Telegram documentation] (https://core.telegram.org/bots/api#file)
fn file_url(base: &str, token: &str, file_path: &str) -> String {
    format!(
        "{url}/file/bot{token}/{file}",
        url = base,
        token = token,
        file = file_path,
    )
}

pub async fn request_multipart<T: DeserializeOwned>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: Option<Form>,
) -> ResponseResult<T> {
    let mut response = client
        .post(&method_url(TELEGRAM_API_URL, token, method_name))
        .apply(|request_builder| match params {
            Some(params) => request_builder.multipart(params),
            None => request_builder,
        })
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    let response = serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?;

    match response {
        TelegramResponse::Ok { result, .. } => Ok(result),
        TelegramResponse::Err {
            description,
            error_code,
            response_parameters: _,
            ..
        } => Err(RequestError::ApiError {
            description,
            status_code: StatusCode::from_u16(error_code).unwrap(),
        }),
    }
}

pub async fn request_json<T: DeserializeOwned, P: Serialize>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: &P,
) -> ResponseResult<T> {
    let mut response = client
        .post(&method_url(TELEGRAM_API_URL, token, method_name))
        .json(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    let response = serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?;

    match response {
        TelegramResponse::Ok { result, .. } => Ok(result),
        TelegramResponse::Err {
            description,
            error_code,
            response_parameters: _,
            ..
        } => Err(RequestError::ApiError {
            description,
            status_code: StatusCode::from_u16(error_code).unwrap(),
        }),
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TelegramResponse<R> {
    Ok {
        /// Dummy field. Used for deserialization.
        #[allow(dead_code)]
        ok: bool, // TODO: True type

        result: R,
    },
    Err {
        /// Dummy field. Used for deserialization.
        #[allow(dead_code)]
        ok: bool, // TODO: False type

        description: String,
        error_code: u16,
        response_parameters: Option<ResponseParameters>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_url_test() {
        let url = method_url(
            TELEGRAM_API_URL,
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "methodName",
        );

        assert_eq!(
            url,
            "https://api.telegram.org/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/methodName"
        );
    }

    #[test]
    fn file_url_test() {
        let url = file_url(
            TELEGRAM_API_URL,
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ",
        );

        assert_eq!(
            url,
            "https://api.telegram.org/file/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ"
        );
    }
}
