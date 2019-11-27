use reqwest::{multipart::Form, Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{requests::ResponseResult, RequestError};

use super::{TelegramResponse, TELEGRAM_API_URL};

pub async fn request_multipart<T>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: Form,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .multipart(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub async fn request_json<T, P>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: &P,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
    P: Serialize,
{
    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .json(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub async fn request_body<T>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: String,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .body(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub async fn request_dynamic<T>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: crate::requests::dynamic::Kind,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    use crate::requests::dynamic::Kind;

    match params {
        Kind::Json(str) => request_body(client, token, method_name, str).await,
        Kind::Multipart(form) => {
            request_multipart(client, token, method_name, form).await
        }
    }
}

async fn process_response<T>(response: Response) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?
    .into()
}
