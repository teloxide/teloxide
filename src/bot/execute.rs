use serde::de::DeserializeOwned;

use crate::{
    Bot,
    requests::{dynamic, json, multipart, ResponseResult},
    network::{request_dynamic, request_json, request_multipart},
};

impl Bot {
    /// Execute dyn-request
    ///
    /// ## Example
    /// ```no_run
    /// # use telebofr::{Bot, requests::payloads::SendMessage};
    /// # #[tokio::main] async fn main() {
    /// let bot = Bot::new("TOKEN");
    /// let payload = SendMessage::new(123456, "text");
    /// bot.execute_dyn(&payload).await;
    /// # }
    /// ```
    ///
    /// **NOTES**:
    /// 1. we recommend to use `bot.send_message(id, "text").send().await`
    ///   instead
    /// 2. this is _dynamic_ version of execute, so it has a _little_ overhead,
    ///   prefer using [`execute_json`] or [`execute_multipart`] depending on
    ///   type of payload when possible.
    ///
    /// [`execute_json`]: self::Bot::execute_json
    /// [`execute_multipart`]: self::Bot::execute_multipart
    pub async fn execute_dyn<O>(
        &self,
        payload: &dyn dynamic::Payload<Output = O>
    ) -> ResponseResult<O>
    where
        O: DeserializeOwned,
    {
        request_dynamic(
            self.client(),
            self.token(),
            payload.name(),
            payload.kind()
        ).await
    }

    /// Execute json-request
    ///
    /// ## Example
    /// ```no_run
    /// # use telebofr::{Bot, requests::payloads::SendMessage};
    /// # #[tokio::main] async fn main() {
    /// let bot = Bot::new("TOKEN");
    /// let payload = SendMessage::new(123456, "text");
    /// bot.execute_json(&payload).await;
    /// # }
    /// ```
    ///
    /// **NOTE**: we recommend to use
    ///   `bot.send_message(id, "text").send().await` instead
    pub async fn execute_json<P>(&self, payload: &P) -> ResponseResult<P::Output>
    where
        P: json::Payload,
        P::Output: DeserializeOwned,
    {
        request_json(self.client(), self.token(), P::NAME, payload).await
    }

    /// Execute multipart-request
    ///
    /// ## Example
    /// ```no_run
    /// # use telebofr::{Bot, requests::payloads::SendAnimation, types::InputFile};
    /// # #[tokio::main] async fn main() {
    /// let bot = Bot::new("TOKEN");
    /// let payload = SendAnimation::new(
    ///     123456,
    ///     InputFile::Url(String::from("https://example.com"))
    /// );
    /// bot.execute_multipart(&payload).await;
    /// # }
    /// ```
    ///
    /// **NOTE**: we recommend to use
    ///   `bot.send_animation(id, InputFile::...).send().await` instead
    pub async fn execute_multipart<P>(&self, payload: &P) -> ResponseResult<P::Output>
    where
        P: multipart::Payload,
        P::Output: DeserializeOwned,
    {
        request_multipart(
            self.client(),
            self.token(),
            P::NAME,
            payload.payload()
        ).await
    }
}
