use reqwest::StatusCode;

//<editor-fold desc="download">
#[derive(Debug, Display, From)]
pub enum DownloadError {
    #[display(fmt = "Network error: {err}", err = _0)]
    NetworkError(reqwest::Error),

    #[display(fmt = "IO Error: {err}", err = _0)]
    Io(std::io::Error),
}

impl std::error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DownloadError::NetworkError(err) => Some(err),
            DownloadError::Io(err) => Some(err),
        }
    }
}
//</editor-fold>

//<editor-fold desc="request">
#[derive(Debug, Display)]
pub enum RequestError {
    #[display(fmt = "Telegram error #{}: {}", status_code, description)]
    ApiError {
        status_code: StatusCode,
        description: String,
    },

    /// The group has been migrated to a supergroup with the specified
    /// identifier.
    #[display(fmt = "The group has been migrated to a supergroup with id {id}", id = _0)]
    MigrateToChatId(i64),

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated
    #[display(fmt = "Retry after {secs} seconds", secs = _0)]
    RetryAfter(i32),

    #[display(fmt = "Network error: {err}", err = _0)]
    NetworkError(reqwest::Error),

    #[display(fmt = "InvalidJson error caused by: {err}", err = _0)]
    InvalidJson(serde_json::Error),
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::ApiError { .. } => None,
            RequestError::MigrateToChatId(_) => None,
            RequestError::RetryAfter(_) => None,
            RequestError::NetworkError(err) => Some(err),
            RequestError::InvalidJson(err) => Some(err),
        }
    }
}
//</editor-fold>
