use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;
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
    /// Returns [`BotBuilder`] from the `TELOXIDE_TOKEN` environmental variable
    /// (a bot's token).
    ///
    /// # Panics
    /// If cannot get the `TELOXIDE_TOKEN` environmental variable.
    ///
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn from_env() -> BotBuilder {
        BotBuilder {
            token: std::env::var("TELOXIDE_TOKEN")
                .expect("Cannot get the TELOXIDE_TOKEN env variable"),
            client: None,
        }
    }

    /// Returns [`BotBuilder`] with the specified token.
    ///
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn new<S>(token: S) -> BotBuilder
    where
        S: Into<String>,
    {
        BotBuilder {
            token: token.into(),
            client: None,
        }
    }
}

/// Used to build [`Bot`].
///
/// [`Bot`]: crate::Bot
pub struct BotBuilder {
    token: String,
    client: Option<Client>,
}

impl BotBuilder {
    /// Sets your custom [`reqwest::Client`] (teloxide will make all requests
    /// using it).
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Enables logging through [pretty-env-logger].
    ///
    /// A logger will **only** print errors from teloxide and **all** logs from
    /// your program.
    ///
    /// [pretty-env-logger]: https://crates.io/crates/pretty_env_logger
    pub fn enable_logging(self, crate_name: &'static str) -> Self {
        Self::enable_logging_with_filter(self, crate_name, LevelFilter::Trace)
    }

    /// Enables logging through [pretty-env-logger].
    ///
    /// A logger will **only** print errors from teloxide and restrict logs from
    /// your program by the specified filter.
    ///
    /// [pretty-env-logger]: https://crates.io/crates/pretty_env_logger
    pub fn enable_logging_with_filter(
        self,
        crate_name: &'static str,
        filter: LevelFilter,
    ) -> Self {
        pretty_env_logger::formatted_builder()
            .write_style(WriteStyle::Auto)
            .filter(Some(crate_name), filter)
            .filter(Some("teloxide"), LevelFilter::Error)
            .init();
        self
    }

    /// Builds [`Bot`].
    ///
    /// Sets the default [`request::Client`] if you haven't specified yours.
    ///
    /// [`request::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    /// [`Bot`]: crate::Bot
    pub fn build(self) -> Arc<Bot> {
        Arc::new(Bot {
            token: self.token,
            client: self.client.unwrap_or(Client::new()),
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
