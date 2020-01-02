use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{InputFile, True},
    Bot,
};

/// Use this method to specify a url and receive incoming updates via an
/// outgoing webhook. Whenever there is an update for the bot, we will send an
/// HTTPS POST request to the specified url, containing a JSON-serialized
/// Update. In case of an unsuccessful request, we will give up after a
/// reasonable amount of attempts. Returns True on success. If you'd like to
/// make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else
/// knows your bot‘s token, you can be pretty sure it’s us.Notes1. You will not
/// be able to receive updates using getUpdates for as long as an outgoing
/// webhook is set up.2. To use a self-signed certificate, you need to upload
/// your public key certificate using certificate parameter. Please upload as
/// InputFile, sending a String will not work.3. Ports currently supported for
/// Webhooks: 443, 80, 88, 8443.NEW! If you're having any trouble setting up
/// webhooks, please check out this amazing guide to Webhooks.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetWebhook<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// HTTPS url to send updates to. Use an empty string to remove webhook
    /// integration
    url: String,
    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See our self-signed guide for details.
    certificate: Option<InputFile>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery, 1-100. Defaults to 40. Use lower values to limit
    /// the load on your bot‘s server, and higher values to increase your bot’s
    /// throughput.
    max_connections: Option<i32>,
    /// List the types of updates you want your bot to receive. For example,
    /// specify [“message”, “edited_channel_post”, “callback_query”] to only
    /// receive updates of these types. See Update for a complete list of
    /// available update types. Specify an empty list to receive all updates
    /// regardless of type (default). If not specified, the previous setting
    /// will be used.Please note that this parameter doesn't affect updates
    /// created before the call to the setWebhook, so unwanted updates may be
    /// received for a short period of time.
    allowed_updates: Option<Vec<String>>,
}

#[async_trait::async_trait]
impl Request<True> for SetWebhook<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setWebhook",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SetWebhook<'a> {
    pub(crate) fn new<U>(bot: &'a Bot, url: U) -> Self
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

    pub fn url<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.url = val.into();
        self
    }

    pub fn certificate(mut self, val: InputFile) -> Self {
        self.certificate = Some(val);
        self
    }

    pub fn max_connections(mut self, val: i32) -> Self {
        self.max_connections = Some(val);
        self
    }

    pub fn allowed_updates<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<String>>,
    {
        self.allowed_updates = Some(val.into());
        self
    }
}
