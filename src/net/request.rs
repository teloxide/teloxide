use std::time::Duration;

use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    net::{TelegramResponse, TELEGRAM_API_URL},
    requests::ResponseResult,
    serde_multipart::to_form,
    RequestError,
};

const DELAY_ON_SERVER_ERROR: Duration = Duration::from_secs(10);

pub async fn request_multipart<P, R>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: &P, // I'll regret this
) -> ResponseResult<R>
where
    P: Serialize,
    R: DeserializeOwned,
{
    use crate::serde_multipart::Error;
    let form = match to_form(params).await {
        Ok(x) => x,
        Err(Error::Io(ioerr)) => return Err(RequestError::Io(ioerr)),
        Err(_) => unreachable!(
            "we don't create requests those fail to serialize (if you see this, open an issue :|)"
        ),
    };

    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .multipart(form)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub async fn request_json<P, R>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: &P,
) -> ResponseResult<R>
where
    P: Serialize,
    R: DeserializeOwned,
{
    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .json(params)
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
        tokio::time::delay_for(DELAY_ON_SERVER_ERROR).await;
    }

    serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?
    .into()
}
