use reqwest::r#async::Client;

use reqwest::StatusCode;

mod games;
mod getting_updates;
mod inline_mode;
mod other;
mod payments;
mod stickers;
mod telegram_passport;
mod updating_messages;

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
