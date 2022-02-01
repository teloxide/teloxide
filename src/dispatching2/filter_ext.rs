#![allow(clippy::redundant_closure_call)]

use dptree::{di::DependencyMap, Handler};
use teloxide_core::types::{Message, Update, UpdateKind};

macro_rules! define_ext {
    ($ext_name:ident, $for_ty:ty => $( ($func:ident, $proj_fn:expr, $fn_doc:expr) ,)*) => {
        #[doc = concat!("Filter methods for [`", stringify!($for_ty), "`].")]
        pub trait $ext_name<Out> {
            $( define_ext!(@sig $func, $fn_doc); )*
        }

        impl<Out> $ext_name<Out> for $for_ty
        where
            Out: Send + Sync + 'static,
        {
            $( define_ext!(@impl $for_ty, $func, $proj_fn); )*
        }
    };

    (@sig $func:ident, $fn_doc:expr) => {
        #[doc = $fn_doc]
        fn $func() -> Handler<'static, DependencyMap, Out>;
    };

    (@impl $for_ty:ty, $func:ident, $proj_fn:expr) => {
        fn $func() -> Handler<'static, DependencyMap, Out> {
            dptree::filter_map(move |input: $for_ty| {
                async move { $proj_fn(input) }
            })
        }
    };
}

macro_rules! define_message_ext {
    ($( ($func:ident, $fn_name:path) ,)*) => {
        define_ext! {
            MessageFilterExt, crate::types::Message =>
            $((
                $func,
                (|x| $fn_name(&x).map(ToOwned::to_owned)),
                concat!("Applies the [`crate::types::", stringify!($fn_name), "`] filter.")
            ),)*
        }
    }
}

// May be expanded in the future.
define_message_ext! {
    (filter_from, Message::from),
    (filter_animation, Message::animation),
    (filter_audio, Message::audio),
    (filter_contact, Message::contact),
    (filter_document, Message::document),
    (filter_location, Message::location),
    (filter_photo, Message::photo),
    (filter_poll, Message::poll),
    (filter_sticker, Message::sticker),
    (filter_text, Message::text),
    (filter_reply_to_message, Message::reply_to_message),
    (filter_forward_from, Message::forward_from),
    (filter_new_chat_members, Message::new_chat_members),
    (filter_left_chat_member, Message::left_chat_member),
    (filter_pinned, Message::pinned_message),
    (filter_dice, Message::dice),
}

macro_rules! define_update_ext {
    ($( ($func:ident, $kind:path) ,)*) => {
        define_ext! {
            UpdateFilterExt, crate::types::Update =>
            $((
                $func,
                |update: Update| match update.kind {
                    $kind(x) => Some(x),
                    _ => None,
                },
                concat!("Filters out [`crate::types::", stringify!($kind), "`] objects.")
            ),)*
        }
    }
}

// May be expanded in the future.
define_update_ext! {
    (filter_message, UpdateKind::Message),
    (filter_edited_message, UpdateKind::EditedMessage),
    (filter_channel_post, UpdateKind::ChannelPost),
    (filter_edited_channel_post, UpdateKind::EditedChannelPost),
    (filter_inline_query, UpdateKind::InlineQuery),
    (filter_chosen_inline_result, UpdateKind::ChosenInlineResult),
    (filter_callback_query, UpdateKind::CallbackQuery),
    (filter_shipping_query, UpdateKind::ShippingQuery),
    (filter_pre_checkout_query, UpdateKind::PreCheckoutQuery),
    (filter_poll, UpdateKind::Poll),
    (filter_poll_answer, UpdateKind::PollAnswer),
    (filter_my_chat_member, UpdateKind::MyChatMember),
    (filter_chat_member, UpdateKind::ChatMember),
}
