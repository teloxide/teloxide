use apply::Apply;
use reqwest::r#async::{multipart::Form, Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    network::{method_url, TelegramResponse, TELEGRAM_API_URL},
    requests::ResponseResult,
    RequestError,
};

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
            .post(&method_url(TELEGRAM_API_URL, token, method_name))
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
            .post(&method_url(TELEGRAM_API_URL, token, method_name))
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
    let response = serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?;

    response.into()
}
