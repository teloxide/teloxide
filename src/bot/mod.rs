use reqwest::Client;
use std::sync::Arc;

mod api;
mod download;

/// A Telegram bot used to send requests.
#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
    client: Client,
}

impl Bot {
    pub fn new<S>(token: S) -> Arc<Self>
    where
        S: Into<String>,
    {
        Self::with_client(token, Client::new())
    }

    pub fn with_client<S>(token: S, client: Client) -> Arc<Self>
    where
        S: Into<String>,
    {
        Arc::new(Bot {
            token: token.into(),
            client,
        })
    }
}

impl Bot {
    // TODO: const fn
    pub fn token(&self) -> &str {
        &self.token
    }

    // TODO: const fn
    pub fn client(&self) -> &Client {
        &self.client
    }
}
