use reqwest::StatusCode;
use reqwest::r#async::Client;
use serde_json::Value;
use futures::compat::Future01CompatExt;
use reqwest::r#async::multipart::Form;
use apply::Apply;


const TELEGRAM_URL_START: &str = "https://api.telegram.org/bot";

#[derive(Debug)]
pub enum Error {
    Api {
        status_code: StatusCode,
        description: Option<String>,
    },
    Send(reqwest::Error),
    InvalidJson(reqwest::Error),
}

pub type Response<T> = Result<T, Error>;

pub async fn request<T: serde::de::DeserializeOwned>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: Option<Form>,
) -> Response<T> {
    let mut response = client
        .post(&format!(
            "{}{token}/{method}",
            TELEGRAM_URL_START,
            token = token,
            method = method_name,
        ))
        .apply(|req| if let Some(params) = params {
            req.multipart(params)
        } else { req })
        .send()
        .compat()
        .await
        .map_err(Error::Send)?;

    let response_json = response
        .json::<Value>()
        .compat()
        .await
        .map_err(Error::InvalidJson)?;

    if response_json["ok"] == "false" {
        return Err(Error::Api {
            status_code: response.status(),
            description: match response_json.get("description") {
                None => None,
                Some(description) => Some(description.to_string()),
            },
        });
    }

    Ok(serde_json::from_value(response_json["result"].clone()).unwrap())
}