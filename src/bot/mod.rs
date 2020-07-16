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
    /// Creates a new `Bot` with the `TELOXIDE_TOKEN` environmental variable (a
    /// bot's token) and the default [`reqwest::Client`].
    ///
    /// # Panics
    /// If cannot get the `TELOXIDE_TOKEN` environmental variable.
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    pub fn from_env() -> Arc<Self> {
        Self::from_env_with_client(Client::new())
    }

    /// Creates a new `Bot` with the `TELOXIDE_TOKEN` environmental variable (a
    /// bot's token) and your [`reqwest::Client`].
    ///
    /// # Panics
    /// If cannot get the `TELOXIDE_TOKEN` environmental variable.
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    #[deprecated]
    pub fn from_env_with_client(client: Client) -> Arc<Self> {
        Self::with_client(
            &std::env::var("TELOXIDE_TOKEN")
                .expect("Cannot get the TELOXIDE_TOKEN env variable"),
            client,
        )
    }

    /// Creates a new `Bot` with the specified token and the default
    /// [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    #[deprecated]
    pub fn new<S>(token: S) -> Arc<Self>
    where
        S: Into<String>,
    {
        Self::with_client(token, Client::new())
    }

    /// Creates a new `Bot` with the specified token and your
    /// [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    #[deprecated]
    pub fn with_client<S>(token: S, client: Client) -> Arc<Self>
    where
        S: Into<String>,
    {
        Arc::new(Self { token: token.into(), client })
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

#[derive(Debug)]
struct BotBuilder {
    token: Option<String>,
    client: Option<Client>,
}

impl BotBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self { token: None, client: None }
    }

    #[must_use]
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    #[must_use]
    pub fn token(mut self, token: S) -> Self
    where
        S: Into<String>,
    {
        self.token = Some(token.into());
        self
    }

    #[must_use]
    pub fn build(self) -> Bot {
        Bot {
            client: self.client.unwrap_or(Client::new()),
            token: self.token.unwrap_or(
                std::env::var("TELOXIDE_TOKEN")
                    .expect("Cannot get the TELOXIDE_TOKEN env variable"),
            ),
        }
    }
}
