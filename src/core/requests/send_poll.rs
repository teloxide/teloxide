use crate::core::requests::{RequestContext, ChatId};


///Use this method to send a native poll. A native poll can't be sent to a private chat. On success, the sent Message is returned.
#[derive(Debug, Clone, Serialize)]
struct SendPoll<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    chat_id 	:ChatId,	Yes 	Unique identifier for the target chat or username of the target channel (in the format @channelusername). A native poll can't be sent to a private chat.
    question :String	String 	Yes 	Poll question, 1-255 characters
options Vec	Array of String 	Yes 	List of answer options, 2-10 strings 1-100 characters each
disable_notification 	Boolean 	Optional 	Sends the message silently. Users will receive a notification with no sound.
reply_to_message_id 	Integer 	Optional 	If the message is a reply, ID of the original message
reply_markup 	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply 	Optional 	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
}
