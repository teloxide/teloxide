use dptree::{di::DependencyMap, Handler};
use teloxide_core::types::{Message, Update, UpdateKind};

macro_rules! define_ext {
    ($ext_name:ident, $for_ty:ty => $( ($func:ident, $arg_ty:ty, $proj_fn:expr) ,)*) => {
        pub trait $ext_name<Out> {
            $( define_ext!(@sig $func, $arg_ty); )*
        }

        impl<Out> $ext_name<Out> for $for_ty
        where
            Out: Send + Sync + 'static,
        {
            $( define_ext!(@impl $for_ty, $func, $arg_ty, $proj_fn); )*
        }
    };

    (@sig $func:ident, $arg_ty:ty) => {
        fn $func() -> Handler<'static, DependencyMap, Out>;
    };

    (@impl $for_ty:ty, $func:ident, $arg_ty:ty, $proj_fn:expr) => {
        fn $func() -> Handler<'static, DependencyMap, Out> {
            dptree::filter_map(move |input: $for_ty| {
                async move { $proj_fn(input) }
            })
        }
    };
}

macro_rules! to_owned_fn {
    ($fn_name:expr) => {
        |x| $fn_name(&x).map(ToOwned::to_owned)
    };
}

// May be expanded in the future.
define_ext! {
    MessageFilterExt, Message =>
    (filter_from, types::User, to_owned_fn![Message::from]),
    (filter_animation, types::Animation, to_owned_fn![Message::animation]),
    (filter_audio, types::Audio, to_owned_fn![Message::audio]),
    (filter_contact, types::Contact, to_owned_fn![Message::contact]),
    (filter_document, types::Document, to_owned_fn![Message::document]),
    (filter_location, types::Location, to_owned_fn![Message::location]),
    (filter_photo, [types::PhotoSize], to_owned_fn![Message::photo]),
    (filter_poll, types::Poll, to_owned_fn![Message::poll]),
    (filter_sticker, types::Sticker, to_owned_fn![Message::sticker]),
    (filter_text, str, to_owned_fn![Message::text]),
    (filter_reply_to_message, Message, to_owned_fn![Message::reply_to_message]),
    (filter_forward_from, types::ForwardedFrom, to_owned_fn![Message::forward_from]),
    (filter_new_chat_members, [types::User], to_owned_fn![Message::new_chat_members]),
    (filter_left_chat_member, types::User, to_owned_fn![Message::left_chat_member]),
    (filter_pinned, Message, to_owned_fn![Message::pinned_message]),
    (filter_dice, types::Dice, to_owned_fn![Message::dice]),
}

macro_rules! kind {
    ($kind:ident) => {
        |update: Update| match update.kind {
            UpdateKind::$kind(x) => Some(x),
            _ => None,
        }
    };
}

define_ext! {
    UpdateFilterExt, Update =>
    (filter_message, types::Message, kind![Message]),
    (filter_edited_message, types::EditedMessage, kind![EditedMessage]),
    (filter_channel_post, types::ChannelPost, kind![ChannelPost]),
    (filter_edited_channel_post, types::EditedChannelPost, kind![EditedChannelPost]),
    (filter_inline_query, types::InlineQuery, kind![InlineQuery]),
    (filter_chosen_inline_result, types::ChosenInlineResult, kind![ChosenInlineResult]),
    (filter_callback_query, types::CallbackQuery, kind![CallbackQuery]),
    (filter_shipping_query, types::ShippingQuery, kind![ShippingQuery]),
    (filter_pre_checkout_query, types::PreCheckoutQuery, kind![PreCheckoutQuery]),
    (filter_poll, types::Poll, kind![Poll]),
    (filter_poll_answer, types::PollAnswer, kind![PollAnswer]),
    (filter_my_chat_member, types::MyChatMember, kind![MyChatMember]),
    (filter_chat_member, types::ChatMember, kind![ChatMember]),
}
