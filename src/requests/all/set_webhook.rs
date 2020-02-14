use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{AllowedUpdate, InputFile, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to specify a url and receive incoming updates via an
/// outgoing webhook.
///
/// Whenever there is an update for the bot, we will send an
/// HTTPS POST request to the specified url, containing a JSON-serialized
/// [`Update`]. In case of an unsuccessful request, we will give up after a
/// reasonable amount of attempts.
///
/// If you'd like to make sure that the Webhook request comes from Telegram,
/// we recommend using a secret path in the URL, e.g.
/// `https://www.example.com/<token>`. Since nobody else knows your bot‘s
/// token, you can be pretty sure it’s us.
///
/// [The official docs](https://core.telegram.org/bots/api#setwebhook).
///
/// [`Update`]: crate::types::Update
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetWebhook {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    url: String,
    certificate: Option<InputFile>,
    max_connections: Option<i32>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[async_trait::async_trait]
impl Request for SetWebhook {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "setWebhook",
            &self,
        )
        .await
    }
}

impl SetWebhook {
    pub(crate) fn new<U>(bot: Arc<Bot>, url: U) -> Self
    where
        U: Into<String>,
    {
        let url = url.into();
        Self {
            bot,
            url,
            certificate: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    /// HTTPS url to send updates to.
    ///
    /// Use an empty string to remove webhook integration.
    pub fn url<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.url = val.into();
        self
    }

    /// Upload your public key certificate so that the root certificate in use
    /// can be checked.
    ///
    /// See our [self-signed guide] for details.
    ///
    /// [self-signed guide]: https://core.telegram.org/bots/self-signed
    pub fn certificate(mut self, val: InputFile) -> Self {
        self.certificate = Some(val);
        self
    }

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery, 1-100.
    ///
    /// Defaults to 40. Use lower values to limit the load on your bot‘s server,
    /// and higher values to increase your bot’s throughput.
    pub fn max_connections(mut self, val: i32) -> Self {
        self.max_connections = Some(val);
        self
    }

    /// List the types of updates you want your bot to receive.
    ///
    /// For example, specify [`AllowedUpdate::Message`],
    /// [`AllowedUpdate::EditedChannelPost`], [`AllowedUpdate::CallbackQuery`]
    /// to only receive updates of these types. Specify an empty list to receive
    /// all updates regardless of type (default). If not specified, the
    /// previous setting will be used. See [`AllowedUpdate`] for a complete list
    /// of available update types.
    ///
    /// Please note that this parameter doesn't affect updates created before
    /// the call to the [`Bot::set_webhook`], so unwanted updates may be
    /// received for a short period of time.
    ///
    /// [`Bot::set_webhook`]: crate::Bot::set_webhook
    /// [`AllowedUpdate::Message`]: crate::types::AllowedUpdate::Message
    /// [`AllowedUpdate::EditedChannelPost`]:
    /// crate::types::AllowedUpdate::EditedChannelPost
    /// [`AllowedUpdate::CallbackQuery`]:
    /// crate::types::AllowedUpdate::CallbackQuery
    /// [`AllowedUpdate`]: crate::types::AllowedUpdate
    pub fn allowed_updates<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<AllowedUpdate>>,
    {
        self.allowed_updates = Some(val.into());
        self
    }
}
