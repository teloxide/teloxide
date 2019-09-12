use crate::core::requests::{ChatId, RequestContext};

//TODO:: need implementation
///Use this method to send information about a venue. On success, the sent
/// Message is returned.
#[derive(Debug, Clone, Serialize)]
struct SendVenue<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///    Integer or String 	Yes 	Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    ///    Float number 	Yes 	Latitude of the venue
    latitude: f64,
    ///Float number 	Yes 	Longitude of the venue
    longitude: f64,
    ///    Yes 	Name of the venue
    title: String,
    ///String 	Yes 	Address of the venue
    address: String,
    ///    String 	Optional 	Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    ///    String 	Optional 	Foursquare type of the venue, if known. (For
    /// example, “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    ///    Boolean 	Optional 	Sends the message silently. Users will receive a
    /// notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    ///    Integer 	Optional 	If the message is a reply, ID of the original
    /// message
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i32>,
    ///	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or
    /// ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<()>, //TODO: need concrete type
}
