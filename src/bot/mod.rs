use reqwest::Client;

mod api;
mod download;
mod execute;

/// A Telegram bot used to build requests.
#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
    client: Client,
}

impl Bot {
    pub fn new<S>(token: S) -> Self
    where
        S: Into<String>,
    {
        Bot {
            token: token.into(),
            client: Client::new(),
        }
    }

    pub fn with_client<S>(token: S, client: Client) -> Self
    where
        S: Into<String>,
    {
        Bot {
            token: token.into(),
            client,
        }
    }
}

impl Bot {
    #[inline]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[inline]
    pub fn client(&self) -> &Client {
        &self.client
    }
}
