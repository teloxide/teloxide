use crate::types::ParseMode;
use reqwest::{
    header::{HeaderMap, CONNECTION},
    Client, ClientBuilder,
};
use std::{sync::Arc, time::Duration};

mod api;
mod download;

pub(crate) const TELOXIDE_TOKEN: &str = "TELOXIDE_TOKEN";
pub(crate) const TELOXIDE_PROXY: &str = "TELOXIDE_PROXY";

/// A requests sender.
///
/// No need to put it into [`Arc`], because it's already in.
///
/// [`Arc`]: std::sync::Arc
#[derive(Debug, Clone)]
pub struct Bot {
    token: Arc<str>,
    client: Client,
    parse_mode: Option<ParseMode>,
}

impl Bot {
    /// Creates a new `Bot` with the `TELOXIDE_TOKEN` & `TELOXIDE_PROXY`
    /// environmental variables (a bot's token & a proxy) and the default
    /// [`reqwest::Client`].
    ///
    /// This function passes the value of `TELOXIDE_PROXY` into
    /// [`reqwest::Proxy::all`], if it exists, otherwise returns the default
    /// client.
    ///
    /// # Panics
    ///  - If cannot get the `TELOXIDE_TOKEN`  environmental variable.
    ///  - If it cannot create [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    /// [`reqwest::Proxy::all`]: https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html#method.all
    #[must_use]
    pub fn from_env() -> Self {
        BotBuilder::new().build()
    }

    /// Creates a new `Bot` with the `TELOXIDE_TOKEN` environmental variable (a
    /// bot's token) and your [`reqwest::Client`].
    ///
    /// # Panics
    /// If cannot get the `TELOXIDE_TOKEN` environmental variable.
    ///
    /// # Caution
    /// Your custom client might not be configured correctly to be able to work
    /// in long time durations, see [issue 223].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    /// [issue 223]: https://github.com/teloxide/teloxide/issues/223
    #[deprecated]
    #[allow(deprecated)]
    pub fn from_env_with_client(client: Client) -> Self {
        Self::with_client(&get_env(TELOXIDE_TOKEN), client)
    }

    /// Creates a new `Bot` with the specified token and the default
    /// [`reqwest::Client`].
    ///
    /// # Panics
    /// If it cannot create [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/latest/reqwest/struct.Client.html
    #[deprecated]
    #[allow(deprecated)]
    pub fn new<S>(token: S) -> Self
    where
        S: Into<String>,
    {
        Self::with_client(token, build_sound_bot())
    }

    /// Creates a new `Bot` with the specified token and your
    /// [`reqwest::Client`].
    ///
    /// # Caution
    /// Your custom client might not be configured correctly to be able to work
    /// in long time durations, see [issue 223].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/latest/reqwest/struct.Client.html
    /// [issue 223]: https://github.com/teloxide/teloxide/issues/223
    #[deprecated]
    #[allow(deprecated)]
    pub fn with_client<S>(token: S, client: Client) -> Self
    where
        S: Into<String>,
    {
        Self {
            token: Into::<Arc<str>>::into(Into::<String>::into(token)),
            client,
            parse_mode: None,
        }
    }
}

/// Returns a builder with safe settings.
///
/// By "safe settings" I mean that a client will be able to work in long time
/// durations, see the [issue 223].
///
/// [issue 223]: https://github.com/teloxide/teloxide/issues/223
pub(crate) fn sound_bot() -> ClientBuilder {
    let mut headers = HeaderMap::new();
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());

    let connect_timeout = Duration::from_secs(5);
    let timeout = 10;

    ClientBuilder::new()
        .connect_timeout(connect_timeout)
        .timeout(Duration::from_secs(connect_timeout.as_secs() + timeout + 2))
        .tcp_nodelay_(true)
        .default_headers(headers)
}

pub(crate) fn build_sound_bot() -> Client {
    sound_bot().build().expect("creating reqwest::Client")
}

fn get_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
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

/// A builder of [`Bot`], supporting some extra settings.
///
/// [`Bot`] crate::Bot
#[derive(Debug, Default)]
pub struct BotBuilder {
    token: Option<String>,
    client: Option<Client>,
    parse_mode: Option<ParseMode>,
}

impl BotBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Specifies a custom HTTPS client. Otherwise, the default will be used.
    ///
    /// # Caution
    ///  - Your custom client might not be configured correctly to be able to
    ///    work
    /// in long time durations, see [issue 223].
    ///
    ///  - If this method is used, the `TELOXIDE_PROXY` environmental variable
    ///    won't be extracted in [`BotBuilder::build`].
    ///
    /// [issue 223]: https://github.com/teloxide/teloxide/issues/223
    /// [`BotBuilder::build`]: crate::BotBuilder::build
    #[must_use]
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Specified a custom token.
    ///
    /// Otherwise, a token will be extracted from the `TELOXIDE_TOKEN`
    /// environmental variable.
    #[must_use]
    pub fn token<S>(mut self, token: S) -> Self
    where
        S: Into<String>,
    {
        self.token = Some(token.into());
        self
    }

    /// Specifies [`ParseMode`], which will be used during all calls to:
    ///
    ///  - [`send_message`]
    ///  - [`send_photo`]
    ///  - [`send_video`]
    ///  - [`send_audio`]
    ///  - [`send_document`]
    ///  - [`send_animation`]
    ///  - [`send_voice`]
    ///  - [`send_poll`]
    ///  - [`edit_message_text`]
    ///  - [`edit_message_caption`]
    ///
    /// [`send_message`]: crate::Bot::send_message
    /// [`send_photo`]: crate::Bot::send_photo
    /// [`send_video`]: crate::Bot::send_video
    /// [`send_audio`]: crate::Bot::send_audio
    /// [`send_document`]: crate::Bot::send_document
    /// [`send_animation`]: crate::Bot::send_animation
    /// [`send_voice`]: crate::Bot::send_voice
    /// [`send_poll`]: crate::Bot::send_poll
    /// [`edit_message_text`]: crate::Bot::edit_message_text
    /// [`edit_message_caption`]: crate::Bot::edit_message_caption
    #[must_use]
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Builds [`Bot`].
    ///
    /// This method will attempt to build a new client with a proxy, specified
    /// in the `TELOXIDE_PROXY` (passed into [`reqwest::Proxy::all`])
    /// environmental variable, if a client haven't been specified. If
    /// `TELOXIDE_PROXY` is unspecified, it'll use no proxy.
    ///
    /// # Panics
    ///  - If cannot get the `TELOXIDE_TOKEN` environmental variable.
    ///  - If it cannot create [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/0.10.1/reqwest/struct.Client.html
    ///
    /// [`Bot`]: crate::Bot
    /// [`reqwest::Proxy::all`]: https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html#method.all
    #[must_use]
    pub fn build(self) -> Bot {
        Bot {
            client: self.client.unwrap_or_else(crate::utils::client_from_env),
            token: self.token.unwrap_or_else(|| get_env(TELOXIDE_TOKEN)).into(),
            parse_mode: self.parse_mode,
        }
    }
}
