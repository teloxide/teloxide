pub(crate) use self::{
    download::{download_file, download_file_stream},
    request::{request_json, request_json2, request_multipart, request_multipart2},
    telegram_response::TelegramResponse,
};

mod download;
mod request;
mod telegram_response;

pub(crate) const TELEGRAM_API_URL: &str = "https://api.telegram.org";

/// Creates URL for making HTTPS requests. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#making-requests
fn method_url(base: reqwest::Url, token: &str, method_name: &str) -> reqwest::Url {
    base.join(&format!("/bot{token}/{method}", token = token, method = method_name))
        .expect("failed to format url")
}

/// Creates URL for downloading a file. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#file
fn file_url(base: reqwest::Url, token: &str, file_path: &str) -> reqwest::Url {
    base.join(&format!("file/bot{token}/{file}", token = token, file = file_path))
        .expect("failed to format url")
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
}
