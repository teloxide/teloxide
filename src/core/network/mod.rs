use super::requests::Request;
use apply::Apply;
use futures::compat::Future01CompatExt;
use reqwest::r#async::Client;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

fn method_url(base: &str, token: &str, method_name: &str) -> String {
    format!(
        "{url}/bot{token}/{method}",
        url = base,
        token = token,
        method = method_name,
    )
}

fn file_url(base: &str, token: &str, file_path: &str) -> String {
    format!(
        "{url}/file/bot{token}/{file}",
        url = base,
        token = token,
        file = file_path,
    )
}

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

pub async fn request<T: DeserializeOwned, R: Request<ReturnValue = T>>(
    client: &Client,
    request: R,
) -> ResponseResult<T> {
    let mut response = client
        .post(&method_url(TELEGRAM_API_URL, request.token(), request.name()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_url_test() {
        let url = method_url(
            TELEGRAM_API_URL,
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "methodName"
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
            "AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ"
        );

        assert_eq!(
            url,
            "https://api.telegram.org/file/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ"
        );
    }
}
