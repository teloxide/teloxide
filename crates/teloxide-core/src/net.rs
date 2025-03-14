//! Network-specific API.

use std::time::Duration;

pub use self::download::{download_file, download_file_stream, Download};

pub(crate) use self::{
    request::{request_json, request_multipart},
    telegram_response::TelegramResponse,
};

mod download;
mod request;
mod telegram_response;

/// The default Telegram API URL.
pub const TELEGRAM_API_URL: &str = "https://api.telegram.org";

/// Constructs a network client from the `TELOXIDE_PROXY` environmental
/// variable.
///
/// This function passes the value of `TELOXIDE_PROXY` into
/// [`reqwest::Proxy::all`], if it exists, otherwise returns the default
/// client.
///
/// ## Note
///
/// The created client will have safe settings, meaning that it will be able to
/// work in long time durations, see the [issue 223].
///
/// [`reqwest::Proxy::all`]: https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html#method.all
/// [issue 223]: https://github.com/teloxide/teloxide/issues/223
///
/// ## Panics
///
/// If `TELOXIDE_PROXY` exists, but isn't correct url.
#[must_use]
pub fn client_from_env() -> reqwest::Client {
    use reqwest::Proxy;

    const TELOXIDE_PROXY: &str = "TELOXIDE_PROXY";

    let builder = default_reqwest_settings();

    match std::env::var(TELOXIDE_PROXY).ok() {
        Some(proxy) => builder.proxy(Proxy::all(proxy).expect("reqwest::Proxy creation failed")),
        None => builder,
    }
    .build()
    .expect("creating reqwest::Client")
}

/// Returns a reqwest client builder with default settings.
///
/// Client built from default settings is supposed to work over long time
/// durations, see the [issue 223].
///
/// The current settings are:
///  - A connection timeout of 5 seconds.
///  - A timeout of 17 seconds.
///  - `tcp_nodelay` is on.
///
/// ## Notes
///
/// 1. The settings may change in the future.
/// 2. If you are using the polling mechanism to get updates, the timeout
///    configured in the client should be bigger than the polling timeout.
/// 3. If you alter the current settings listed above, your bot will not be
///    guaranteed to work over long time durations.
///
/// [issue 223]: https://github.com/teloxide/teloxide/issues/223
pub fn default_reqwest_settings() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(17))
        .tcp_nodelay(true)
}

/// Creates URL for making HTTPS requests. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#making-requests
fn method_url(base: reqwest::Url, token: &str, method_name: &str) -> reqwest::Url {
    let mut url = base;
    {
        let mut segments = url.path_segments_mut().expect("base URL cannot be a cannot-be-a-base");
        segments.push(&format!("bot{}", token));
        segments.push(method_name);
    }
    url
}

/// Creates URL for downloading a file. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#file
fn file_url(base: reqwest::Url, token: &str, file_path: &str) -> reqwest::Url {
    let mut url = base;
    {
        let mut segments = url.path_segments_mut().expect("base URL cannot be a cannot-be-a-base");
        segments.push("file");
        segments.push(&format!("bot{}", token));
        segments.push(file_path);
    }
    url
}

#[cfg(test)]
mod tests {
    use crate::net::*;

    #[test]
    fn method_url_test() {
        let url = method_url(
            reqwest::Url::parse(TELEGRAM_API_URL).unwrap(),
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "methodName",
        );

        assert_eq!(
            url.as_str(),
            "https://api.telegram.org/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/methodName"
        );
    }

    #[test]
    fn method_url_with_custom_url_test() {
        let url = method_url(
            reqwest::Url::parse("https://example.com/telegram").unwrap(),
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "methodName",
        );

        assert_eq!(
	    url.as_str(),
	    "https://example.com/telegram/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/methodName"
	);
    }

    #[test]
    fn file_url_test() {
        let url = file_url(
            reqwest::Url::parse(TELEGRAM_API_URL).unwrap(),
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ",
        );

        assert_eq!(
            url.as_str(),
            "https://api.telegram.org/file/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ"
        );
    }

    #[test]
    fn file_url_with_custom_url_test() {
        let url = file_url(
            reqwest::Url::parse("https://example.com/telegram").unwrap(),
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ",
        );

        assert_eq!(
	    url.as_str(),
	    "https://example.com/telegram/file/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ"
	);
    }
}
