//! Telegram API types.

pub use allowed_update::*;
pub use animation::*;
pub use audio::*;
pub use bot_command::*;
pub use bot_command_scope::*;
pub use bot_description::*;
pub use bot_name::*;
pub use bot_short_description::*;
pub use callback_game::*;
pub use callback_query::*;
pub use chat::*;
pub use chat_action::*;
pub use chat_administrator_rights::*;
pub use chat_full_info::*;
pub use chat_invite_link::*;
pub use chat_join_request::*;
pub use chat_location::*;
pub use chat_member::*;
pub use chat_member_updated::*;
pub use chat_permissions::*;
pub use chat_photo::*;
pub use chat_shared::*;
pub use chat_type::*;
pub use chosen_inline_result::*;
pub use contact::*;
pub use dice::*;
pub use dice_emoji::*;
pub use document::*;
pub use encrypted_credentials::*;
pub use encrypted_passport_element::*;
pub use file::*;
pub use force_reply::*;
pub use forum_topic::*;
pub use forum_topic_closed::*;
pub use forum_topic_created::*;
pub use forum_topic_edited::*;
pub use forum_topic_reopened::*;
pub use game::*;
pub use game_high_score::*;
pub use general_forum_topic_hidden::*;
pub use general_forum_topic_unhidden::*;
pub use inaccessible_message::*;
pub use inline_keyboard_button::*;
pub use inline_keyboard_markup::*;
pub use inline_query::*;
pub use inline_query_result::*;
pub use inline_query_result_article::*;
pub use inline_query_result_audio::*;
pub use inline_query_result_cached_audio::*;
pub use inline_query_result_cached_document::*;
pub use inline_query_result_cached_gif::*;
pub use inline_query_result_cached_mpeg4_gif::*;
pub use inline_query_result_cached_photo::*;
pub use inline_query_result_cached_sticker::*;
pub use inline_query_result_cached_video::*;
pub use inline_query_result_cached_voice::*;
pub use inline_query_result_contact::*;
pub use inline_query_result_document::*;
pub use inline_query_result_game::*;
pub use inline_query_result_gif::*;
pub use inline_query_result_location::*;
pub use inline_query_result_mpeg4_gif::*;
pub use inline_query_result_photo::*;
pub use inline_query_result_venue::*;
pub use inline_query_result_video::*;
pub use inline_query_result_voice::*;
pub use inline_query_results_button::*;
pub use input_file::*;
pub use input_media::*;
pub use input_message_content::*;
pub use input_sticker::*;
pub use invoice::*;
pub use keyboard_button::*;
pub use keyboard_button_poll_type::*;
pub use keyboard_button_request_chat::*;
pub use keyboard_button_request_user::*;
pub use label_price::*;
pub use location::*;
pub use login_url::*;
pub use mask_position::*;
pub use maybe_inaccessible_message::*;
pub use me::*;
pub use menu_button::*;
pub use message::*;
pub use message_auto_delete_timer_changed::*;
pub use message_entity::*;
pub use message_id::*;
pub use message_origin::*;
pub use order_info::*;
pub use parse_mode::*;
pub use passport_data::*;
pub use passport_element_error::*;
pub use passport_file::*;
pub use photo_size::*;
pub use poll::*;
pub use poll_answer::*;
pub use poll_type::*;
pub use pre_checkout_query::*;
pub use proximity_alert_triggered::*;
pub use reply_keyboard_markup::*;
pub use reply_keyboard_remove::*;
pub use reply_markup::*;
pub use reply_parameters::*;
pub use response_parameters::*;
pub use sent_web_app_message::*;
pub use shipping_address::*;
pub use shipping_option::*;
pub use shipping_query::*;
pub use sticker::*;
pub use sticker_set::*;
pub use story::*;
pub use successful_payment::*;
pub use switch_inline_query_chosen_chat::*;
pub use target_message::*;
pub use text_quote::*;
pub use thread_id::*;
pub use unit_false::*;
pub use unit_true::*;
pub use update::*;
pub use user::*;
pub use user_profile_photos::*;
pub use users_shared::*;
pub use venue::*;
pub use video::*;
pub use video_chat_ended::*;
pub use video_chat_participants_invited::*;
pub use video_chat_scheduled::*;
pub use video_chat_started::*;
pub use video_note::*;
pub use voice::*;
pub use web_app_data::*;
pub use web_app_info::*;
pub use webhook_info::*;
pub use write_access_allowed::*;

mod allowed_update;
mod animation;
mod audio;
mod bot_command;
mod bot_command_scope;
mod bot_description;
mod bot_name;
mod bot_short_description;
mod callback_game;
mod callback_query;
mod chat;
mod chat_action;
mod chat_administrator_rights;
mod chat_full_info;
mod chat_invite_link;
mod chat_join_request;
mod chat_location;
mod chat_member;
mod chat_member_updated;
mod chat_permissions;
mod chat_photo;
mod chat_shared;
mod chat_type;
mod chosen_inline_result;
mod contact;
mod dice;
mod dice_emoji;
mod document;
mod file;
mod force_reply;
mod forum_topic;
mod forum_topic_closed;
mod forum_topic_created;
mod forum_topic_edited;
mod forum_topic_reopened;
mod game;
mod game_high_score;
mod general_forum_topic_hidden;
mod general_forum_topic_unhidden;
mod inaccessible_message;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod inline_query_results_button;
mod input_file;
mod input_media;
mod input_message_content;
mod input_sticker;
mod invoice;
mod keyboard_button;
mod keyboard_button_poll_type;
mod keyboard_button_request_chat;
mod keyboard_button_request_user;
mod label_price;
mod location;
mod login_url;
mod mask_position;
mod maybe_inaccessible_message;
mod me;
mod menu_button;
mod message;
mod message_auto_delete_timer_changed;
mod message_entity;
mod message_id;
mod message_origin;
mod order_info;
mod parse_mode;
mod photo_size;
mod poll;
mod poll_answer;
mod poll_type;
mod pre_checkout_query;
mod proximity_alert_triggered;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod reply_markup;
mod reply_parameters;
mod response_parameters;
mod sent_web_app_message;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod sticker_set;
mod story;
mod successful_payment;
mod switch_inline_query_chosen_chat;
mod target_message;
mod text_quote;
mod thread_id;
mod unit_false;
mod unit_true;
mod update;
mod user;
mod user_profile_photos;
mod users_shared;
mod venue;
mod video;
mod video_chat_ended;
mod video_chat_participants_invited;
mod video_chat_scheduled;
mod video_chat_started;
mod video_note;
mod voice;
mod web_app_data;
mod web_app_info;
mod webhook_info;
mod write_access_allowed;

mod inline_query;
mod inline_query_result;
mod inline_query_result_article;
mod inline_query_result_audio;
mod inline_query_result_cached_audio;
mod inline_query_result_cached_document;
mod inline_query_result_cached_gif;
mod inline_query_result_cached_mpeg4_gif;
mod inline_query_result_cached_photo;
mod inline_query_result_cached_sticker;
mod inline_query_result_cached_video;
mod inline_query_result_cached_voice;
mod inline_query_result_contact;
mod inline_query_result_document;
mod inline_query_result_game;
mod inline_query_result_gif;
mod inline_query_result_location;
mod inline_query_result_mpeg4_gif;
mod inline_query_result_photo;
mod inline_query_result_venue;
mod inline_query_result_video;
mod inline_query_result_voice;

mod encrypted_credentials;
mod encrypted_passport_element;
mod passport_data;
mod passport_element_error;
mod passport_file;

pub use non_telegram_types::{country_code::*, currency::*, until_date::*};
mod non_telegram_types {
    pub(super) mod country_code;
    pub(super) mod currency;
    pub(crate) mod mime;
    pub(super) mod until_date;
}

mod chat_id;
mod recipient;
mod seconds;
mod user_id;

pub use chat_id::*;
pub use recipient::*;
pub use seconds::*;
pub use user_id::*;

/// Converts an `i64` timestamp to a `choro::DateTime`, producing serde error
/// for invalid timestamps
pub(crate) fn serde_timestamp<E: serde::de::Error>(
    timestamp: i64,
) -> Result<chrono::DateTime<chrono::Utc>, E> {
    chrono::DateTime::from_timestamp(timestamp, 0).ok_or_else(|| E::custom("invalid timestump"))
}

pub(crate) mod serde_opt_date_from_unix_timestamp {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::types::serde_timestamp;

    pub(crate) fn serialize<S>(
        this: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        this.map(|dt| dt.timestamp()).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<i64>::deserialize(deserializer)?.map(serde_timestamp).transpose()
    }

    #[test]
    fn test() {
        #[derive(Serialize, Deserialize)]
        struct Struct {
            #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
            date: Option<DateTime<Utc>>,
        }

        {
            let json = r#"{"date":1}"#;
            let expected = DateTime::from_timestamp(1, 0).unwrap();

            let Struct { date } = serde_json::from_str(json).unwrap();
            assert_eq!(date, Some(expected));
        }

        {
            let json = "{}";

            let Struct { date } = serde_json::from_str(json).unwrap();
            assert_eq!(date, None);
        }
    }
}

pub(crate) mod serde_date_from_unix_timestamp {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::types::serde_timestamp;

    pub(crate) fn serialize<S>(this: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        this.timestamp().serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_timestamp(i64::deserialize(deserializer)?)
    }
}

pub(crate) mod option_url_from_string {
    use reqwest::Url;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S>(this: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match this {
            Some(url) => url.serialize(serializer),
            None => "".serialize(serializer),
        }
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(reqwest::Url::deserialize(deserializer).ok())
    }

    #[test]
    fn test() {
        use std::str::FromStr;
        #[derive(Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::types::option_url_from_string")]
            url: Option<Url>,
        }

        {
            let json = r#"{"url":""}"#;
            let url: Struct = serde_json::from_str(json).unwrap();
            assert_eq!(url.url, None);
            assert_eq!(serde_json::to_string(&url).unwrap(), json.to_owned());

            let json = r#"{"url":"https://github.com/token"}"#;
            let url: Struct = serde_json::from_str(json).unwrap();
            assert_eq!(url.url, Some(Url::from_str("https://github.com/token").unwrap()));
            assert_eq!(serde_json::to_string(&url).unwrap(), json.to_owned());
        }
    }
}

#[allow(dead_code)]
pub(crate) mod option_msg_id_as_int {
    use crate::types::MessageId;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S>(this: &Option<MessageId>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        this.map(|MessageId(id)| id).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<MessageId>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<i32>::deserialize(deserializer).map(|r| r.map(MessageId))
    }

    #[test]
    fn test() {
        #[derive(Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::types::option_msg_id_as_int")]
            id: Option<MessageId>,
        }

        {
            let json = r#"{"id":123}"#;
            let id: Struct = serde_json::from_str(json).unwrap();
            assert_eq!(id.id, Some(MessageId(123)));
            assert_eq!(serde_json::to_string(&id).unwrap(), json.to_owned());
        }
    }
}

pub(crate) mod serde_rgb {
    use serde::{de::Visitor, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(&this: &[u8; 3], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u32(to_u32(this))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 3], D::Error> {
        struct V;

        impl Visitor<'_> for V {
            type Value = [u8; 3];

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer represeting an RGB color")
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(from_u32(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v.try_into().map_err(|_| E::custom("rgb value doesn't fit u32"))?)
            }
        }
        d.deserialize_u32(V)
    }

    fn to_u32([r, g, b]: [u8; 3]) -> u32 {
        u32::from_be_bytes([0, r, g, b])
    }

    fn from_u32(rgb: u32) -> [u8; 3] {
        let [_, r, g, b] = rgb.to_be_bytes();
        [r, g, b]
    }

    #[test]
    fn bytes() {
        assert_eq!(to_u32([0xAA, 0xBB, 0xCC]), 0x00AABBCC);
        assert_eq!(from_u32(0x00AABBCC), [0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn json() {
        #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        struct Struct {
            #[serde(with = "self")]
            color: [u8; 3],
        }

        let json = format!(r#"{{"color":{}}}"#, 0x00AABBCC);
        let Struct { color } = serde_json::from_str(&json).unwrap();

        assert_eq!(color, [0xAA, 0xBB, 0xCC])
    }
}
