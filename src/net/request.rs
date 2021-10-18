use std::time::Duration;

use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Client, Response,
};
use serde::de::DeserializeOwned;

use crate::{net::TelegramResponse, requests::ResponseResult, RequestError};

const DELAY_ON_SERVER_ERROR: Duration = Duration::from_secs(10);

pub async fn request_multipart<T>(
    client: &Client,
    token: &str,
    api_url: reqwest::Url,
    method_name: &str,
    params: reqwest::multipart::Form,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    // Workaround for [#460]
    //
    // Telegram has some methods that return either `Message` or `True` depending on
    // the used arguments we model this as `...` and `..._inline` pairs of methods.
    //
    // Currently inline versions have wrong Payload::NAME (ie with the "Inline"
    // sufix). This removes the sufix allowing to call the right telegram method.
    // Note that currently there are no normal telegram methods ending in "Inline",
    // so this is fine.
    //
    // [#460]: https://github.com/teloxide/teloxide/issues/460
    let method_name = method_name.trim_end_matches("Inline");

    let response = client
        .post(crate::net::method_url(api_url, token, method_name))
        .multipart(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub async fn request_json<T>(
    client: &Client,
    token: &str,
    api_url: reqwest::Url,
    method_name: &str,
    params: Vec<u8>,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    // Workaround for [#460]
    //
    // Telegram has some methods that return either `Message` or `True` depending on
    // the used arguments we model this as `...` and `..._inline` pairs of methods.
    //
    // Currently inline versions have wrong Payload::NAME (ie with the "Inline"
    // sufix). This removes the sufix allowing to call the right telegram method.
    // Note that currently there are no normal telegram methods ending in "Inline",
    // so this is fine.
    //
    // [#460]: https://github.com/teloxide/teloxide/issues/460
    let method_name = method_name.trim_end_matches("Inline");

    let response = client
        .post(crate::net::method_url(api_url, token, method_name))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

async fn process_response<T>(response: Response) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    if response.status().is_server_error() {
        tokio::time::sleep(DELAY_ON_SERVER_ERROR).await;
    }

    serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?
    .into()
}
