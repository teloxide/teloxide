use apply::Apply;
use reqwest::r#async::{multipart::Form, Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{requests::ResponseResult, RequestError};

use super::{TelegramResponse, TELEGRAM_API_URL};

pub async fn request_multipart<T>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: Option<Form>,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    process_response(
        client
            .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
            .apply(|request_builder| match params {
                Some(params) => request_builder.multipart(params),
                None => request_builder,
            })
            .send()
            .await
            .map_err(RequestError::NetworkError)?,
    )
    .await
}

pub async fn request_json<T: DeserializeOwned, P: Serialize>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: &P,
) -> ResponseResult<T> {
    process_response(
        client
            .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
            .json(params)
            .send()
            .await
            .map_err(RequestError::NetworkError)?,
    )
    .await
}

async fn process_response<T: DeserializeOwned>(
    mut response: Response,
) -> ResponseResult<T> {
    serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?
    .into()
}
