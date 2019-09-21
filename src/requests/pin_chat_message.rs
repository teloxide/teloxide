use crate::core::requests::{ChatId, RequestContext, RequestFuture, ResponseResult, Request};
use crate::core::network;

/// Use this method to get up to date information about the chat 
/// (current name of the user for one-on-one conversations, 
/// current username of a user, group or channel, etc.). 
/// Returns a Chat object on success.
#[derive(Debug, Clone, Serialize)]
pub struct PinChatMessage<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier for the target chat or username 
    /// of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
    message_id: i32,
    disable_notification: bool
}

impl<'a> Request<'a> for PinChatMessage<'a> {
    type ReturnValue = bool;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "pinChatMessage",
                &self,
            ).await
        })
    }
}
