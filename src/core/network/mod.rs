use super::requests::Request;
use apply::Apply;
use futures::compat::Future01CompatExt;
use reqwest::r#async::Client;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

#[derive(Debug)]
pub enum RequestError {
    ApiError {
        status_code: StatusCode,
        description: String,
    },
    NetworkError(reqwest::Error),
    InvalidJson(serde_json::Error),
}

pub type ResponseResult<T> = Result<T, RequestError>;

pub async fn request<T: DeserializeOwned, R: Request<T>>(
    client: &Client,
    request: R,
) -> ResponseResult<T> {
    let mut response = client
        .post(&format!(
            "{url}/bot{token}/{method}",
            url = TELEGRAM_API_URL,
            token = request.token(),
            method = request.name(),
        ))
        .apply(|request_builder| {
            if let Some(params) = request.params() {
                request_builder.multipart(params)
            } else {
                request_builder
            }
        })
        .send()
        .compat()
        .await
        .map_err(RequestError::NetworkError)?;

    let response_json = serde_json::from_str::<Value>(
        &response
            .text()
            .compat()
            .await
            .map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?;

    if response_json["ok"] == "false" {
        Err(RequestError::ApiError {
            status_code: response.status(),
            description: response_json["description"].to_string(),
        })
    } else {
        Ok(serde_json::from_value(response_json["result"].clone()).unwrap())
    }
}
