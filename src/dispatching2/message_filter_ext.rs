use dptree::{di::DependencyMap, Handler};
use futures::Future;
use std::sync::Arc;
use teloxide_core::{types, types::Message};

macro_rules! define_ext {
    ($ext_name:ident, $for_ty:ty => $( ($func:ident, $arg_ty:ty, $get_func:expr) ,)*) => {
        pub trait $ext_name<Out> {
            $( define_ext!(@filter_sign $func, $arg_ty); )*
        }
        impl<Out> $ext_name<Out> for $for_ty
        where
            Out: Send + Sync + 'static,
        {
            $( define_ext!(@filter_impl $for_ty, $func, $arg_ty, $get_func); )*
        }
    };
    (@filter_sign $func:ident, $arg_ty:ty) => {
        fn $func<F, Fut>(f: F) -> Handler<'static, DependencyMap, Out>
        where
            F: Fn(&$arg_ty) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = bool> + Send,
        ;
    };
    (@filter_impl $for_ty:ty, $func:ident, $arg_ty:ty, $get_func:expr) => {
        fn $func<F, Fut>(f: F) -> Handler<'static, DependencyMap, Out> where
            F: Fn(&$arg_ty) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = bool> + Send,
        {
            let filter = Arc::new(f);
            dptree::filter(move |mes: $for_ty| {
                let filter = filter.clone();
                async move {
                    match $get_func(&mes) {
                        Some(user) => filter(user).await,
                        None => false,
                    }
                }
            })
        }
    };
}

// May be expanded in the future.
define_ext!(
    MessageFilterExt, Message =>
    (filter_from, types::User, Message::from),
    (filter_animation, types::Animation, Message::animation),
    (filter_audio, types::Audio, Message::audio),
    (filter_contact, types::Contact, Message::contact),
    (filter_document, types::Document, Message::document),
    (filter_location, types::Location, Message::location),
    (filter_photo, [types::PhotoSize], Message::photo),
    (filter_poll, types::Poll, Message::poll),
    (filter_sticker, types::Sticker, Message::sticker),
    (filter_text, str, Message::text),
    (filter_reply_to_message, Message, Message::reply_to_message),
    (filter_forward_from, types::ForwardedFrom, Message::forward_from),
    (filter_new_chat_members, [types::User], Message::new_chat_members),
    (filter_left_chat_member, types::User, Message::left_chat_member),
    (filter_pinned, Message, Message::pinned_message),
    (filter_dice, types::Dice, Message::dice),
);
