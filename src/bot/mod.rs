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
        match std::env::var_os("TELOXIDE_PROXY") {
            Some(proxy) => Self::from_env_with_client(
                Client::builder()
                    .proxy(
                        reqwest::Proxy::all(
                            &*proxy.to_string_lossy(),
                        )
                        .expect("creating reqwest::Proxy"),
                    )
                    .build()
                    .expect("creating reqwest::Client"),
            ),
            None => Self::from_env_with_client(Client::new()),
        }
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
            &*std::env::var_os("TELOXIDE_TOKEN")
                .expect("Cannot get the TELOXIDE_TOKEN env variable")
                .to_string_lossy(),
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
        Self::with_client(token, Client::new())
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
