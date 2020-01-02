//! API requests.

#[macro_export]
macro_rules! impl_multipart {
    ($request:ty, $return_type:ty, $method_name:literal, $payload:expr) => {
        use crate::{
            network::request_multipart,
            requests::{Request, ResponseResult},
        };

        #[async_trait::async_trait]
        impl Request<$return_type> for $request {
            async fn send(
                &self,
                bot: &crate::Bot,
            ) -> ResponseResult<$return_type> {
                request_multipart(
                    bot.client(),
                    bot.token(),
                    $method_name,
                    $payload,
                )
                .await
            }
        }
    };
}

mod all;
mod form_builder;
mod utils;

pub use all::*;

use crate::Bot;

/// A type that is returned after making a request to Telegram.
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// Designates an API request.
#[async_trait::async_trait]
pub trait Request<T> {
    async fn send(&self, bot: &Bot) -> ResponseResult<T>;
}
