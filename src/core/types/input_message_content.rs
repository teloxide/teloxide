use serde::Serialize;
use crate::core::parse_mode::ParseMode;

pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
}

#[derive(Debug, Serialize)]
pub struct InputTextMessageContent {
    pub message_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,
}

#[derive(Debug, Serialize)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32> // should be between 60 and 86400
}

#[derive(Debug, Serialize)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
