//! A Telegram bot.

use reqwest::Client;

mod api;
mod download;

#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
    client: Client,
}

/// Constructors
impl Bot {
    pub fn new(token: &str) -> Self {
        Bot {
            token: String::from(token),
            client: Client::new(),
        }
    }

    pub fn with_client(token: &str, client: Client) -> Self {
        Bot {
            token: String::from(token),
            client,
        }
    }
}

impl Bot {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
