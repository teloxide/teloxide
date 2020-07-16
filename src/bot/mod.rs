use reqwest::{
    header::{HeaderMap, CONNECTION},
    Client, ClientBuilder,
};
use std::{sync::Arc, time::Duration};

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
        Self::from_env_with_client(sound_bot())
    }

    /// Creates a new `Bot` with the `TELOXIDE_TOKEN` environmental variable (a
    /// bot's token) and your [`reqwest::Client`].
    ///
    /// # Panics
    /// If cannot get the `TELOXIDE_TOKEN` environmental variable.
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
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
    pub fn new<S>(token: S) -> Arc<Self>
    where
        S: Into<String>,
    {
        Self::with_client(token, sound_bot())
    }

    /// Creates a new `Bot` with the specified token and your
    /// [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    pub fn with_client<S>(token: S, client: Client) -> Arc<Self>
    where
        S: Into<String>,
    {
        Arc::new(Self { token: token.into(), client })
    }
}

// See https://github.com/teloxide/teloxide/issues/223.
fn sound_bot() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());

    let connect_timeout = Duration::from_secs(5);
    let timeout = 10;

    ClientBuilder::new()
        .connect_timeout(connect_timeout)
        .timeout(Duration::from_secs(connect_timeout.as_secs() + timeout + 2))
        .tcp_nodelay_(true)
        .default_headers(headers)
        .build()
        .expect("Cannot build reqwest::Client")
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
