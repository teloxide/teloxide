use futures::compat::Future01CompatExt;
use reqwest::r#async::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

lazy_static! {
    static ref REQWEST_CLIENT: Client = Client::new();
}

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

#[derive(Debug, Deserialize)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}

pub async fn get_me(bot_token: &str) -> Response<User> {
    let mut response = REQWEST_CLIENT
        .get(&format!(
            "{}{bot_token}/getMe",
            TELEGRAM_URL_START,
            bot_token = bot_token
        ))
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
