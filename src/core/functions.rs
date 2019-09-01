use crate::core::types::User;
use crate::core::{Error, Response, TELEGRAM_URL_START, REQWEST_CLIENT};
use serde_json::Value;
use futures::compat::Future01CompatExt;

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
