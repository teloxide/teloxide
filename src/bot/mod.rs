use std::{future::Future, sync::Arc, time::Duration};

use reqwest::{
    header::{HeaderMap, CONNECTION},
    Client, ClientBuilder,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bot::api_url::ApiUrl,
    net,
    requests::{Payload, ResponseResult},
    serde_multipart,
};

mod api;
mod api_url;
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
    api_url: ApiUrl,
    client: Client,
}

impl Bot {
    /// Creates a new `Bot` with the specified token and the default
    /// [http-client](reqwest::Client).
    ///
    /// # Panics
    ///
    /// If it cannot create [`reqwest::Client`].
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
    pub fn with_client<S>(token: S, client: Client) -> Self
    where
        S: Into<String>,
    {
        Self {
            token: Into::<Arc<str>>::into(Into::<String>::into(token)),
            api_url: ApiUrl::Default,
            client,
        }
    }

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
    pub fn from_env() -> Self {
        Self::from_env_with_client(crate::client_from_env())
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
    pub fn from_env_with_client(client: Client) -> Self {
        Self::with_client(&get_env(TELOXIDE_TOKEN), client)
    }

    /// Sets custom api url.
    ///
    /// For example you can run your own [Telegram bot API server][tbas] and set
    /// it's url using this method.
    ///
    /// [tbas]: https://github.com/tdlib/telegram-bot-api
    ///
    /// ## Examples
    ///
    /// ```
    /// use teloxide_core::{
    ///     requests::{Request, Requester},
    ///     Bot,
    /// };
    ///
    /// # async {
    /// // Setup bot
    /// let url = reqwest::Url::parse("https://localhost/tbas").unwrap();
    /// let bot = Bot::new("TOKEN").set_api_url(url);
    /// // All methods will use "https://localhost/tbas" as an api url
    /// bot.get_me().send().await
    /// # };
    /// ```
    pub fn set_api_url(mut self, url: reqwest::Url) -> Self {
        self.api_url = ApiUrl::Custom(Arc::new(url));
        self
    }

    /// Returns currently used token.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns currently used http-client.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Returns currently used token API url.
    pub fn api_url(&self) -> reqwest::Url {
        self.api_url.get()
    }
}

impl Bot {
    pub(crate) fn execute_json<P>(
        &self,
        payload: &P,
    ) -> impl Future<Output = ResponseResult<P::Output>> + 'static
    where
        P: Payload + Serialize,
        P::Output: DeserializeOwned,
    {
        let client = self.client.clone();
        let token = Arc::clone(&self.token);
        let api_url = self.api_url.clone();

        let params = serde_json::to_vec(payload)
            // this `expect` should be ok since we don't write request those may trigger error here
            .expect("serialization of request to be infallible");

        // async move to capture client&token&api_url&params
        async move { net::request_json(&client, token.as_ref(), api_url.get(), P::NAME, params).await }
    }

    pub(crate) fn execute_multipart<P>(
        &self,
        payload: &P,
    ) -> impl Future<Output = ResponseResult<P::Output>>
    where
        P: Payload + Serialize,
        P::Output: DeserializeOwned,
    {
        let client = self.client.clone();
        let token = Arc::clone(&self.token);
        let api_url = self.api_url.clone();

        let params = serde_multipart::to_form(payload);

        // async move to capture client&token&api_url&params
        async move {
            let params = params.await?;
            net::request_multipart(&client, token.as_ref(), api_url.get(), P::NAME, params).await
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
