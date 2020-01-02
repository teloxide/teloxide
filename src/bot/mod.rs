use reqwest::Client;

mod download;

/// A Telegram bot used to send requests.
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
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
