use crate::core::requests::{ChatId, RequestContext};
use crate::core::types::ReplyMarkup;

//TODO:: need implementation
#[derive(Debug, Clone, Serialize)]
struct SendContact<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///    Integer or String 	Yes 	Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    ///    String 	Yes 	Contact's phone number
    phone_number: String,
    ///    String 	Yes 	Contact's first name
    first_name: String,
    ///    String  	Optional 	Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    ///   	String Optional 	Additional data about the contact in the form of a
    /// vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
    ///    Boolean 	Optional 	Sends the message silently. Users will receive a
    /// notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    ///    Integer 	Optional 	If the message is a reply, ID of the original
    /// message
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i32>,
    ///    InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove
    /// or ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}
