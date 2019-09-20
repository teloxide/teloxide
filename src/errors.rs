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
