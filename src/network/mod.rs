use futures::StreamExt;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    stream::Stream,
    io::{AsyncWrite, AsyncWriteExt},
};
use reqwest::{
    StatusCode,
    r#async::{multipart::Form, Client, Response, Chunk},
};
use bytes::Buf;
use apply::Apply;

use crate::{
    DownloadError, RequestError,
    requests::ResponseResult, types::ResponseParameters,
};


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

pub async fn download_file<D>(
    client: &Client,
    token: &str,
    path: &str,
    destination: &mut D,
) -> Result<(), DownloadError>
where
    D: AsyncWrite + Unpin,
{
    let mut stream = download_file_stream(client, token, path).await?;

    while let Some(chunk) = stream.next().await {
        destination.write_all(chunk?.bytes()).await?;
    }

    Ok(())
}

pub(crate) async fn download_file_stream(
    client: &Client,
    token: &str,
    path: &str,
) -> Result<impl Stream<Item = Result<Chunk, reqwest::Error>>, reqwest::Error> {
    let url = file_url(TELEGRAM_API_URL, token, path);
    let resp = client.get(&url).send().await?.error_for_status()?;
    Ok(resp.into_body())
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
