use self::not_implemented_types::*;


pub type Integer = i32;
pub type UnsignedInteger = u32;


pub use self::{
    answer_pre_checkout_query::AnswerPreCheckoutQuery,
    answer_shipping_query::AnswerShippingQuery,
    audio::Audio,
    chat::Chat,
    chat_permissions::ChatPermissions,
    chat_photo::ChatPhoto,
    chat_member::ChatMember,
    document::Document,
    invoice::Invoice,
    label_price::LabeledPrice,
    message::Message,
    message_entity::MessageEntity,
    order_info::OrderInfo,
    photo_size::PhotoSize,
    pre_checkout_query::PreCheckoutQuery,
    send_invoice::SendInvoice,
    shipping_address::ShippingAddress,
    shipping_option::ShippingOption,
    shipping_query::ShippingQuery,
    sticker::Sticker,
    successful_payment::SuccessfulPayment,
    user::User,
    input_file::InputFile,
    input_media::InputMedia,
    parse_mode::ParseMode,
};

mod answer_pre_checkout_query;
mod answer_shipping_query;
mod audio;
mod chat;
mod chat_permissions;
mod chat_photo;
mod chat_member;
mod document;
mod invoice;
mod label_price;
mod message;
mod message_entity;
mod not_implemented_types;
mod order_info;
mod photo_size;
mod pre_checkout_query;
mod send_invoice;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod successful_payment;
mod user;
mod input_file;
mod input_media;
mod parse_mode;
