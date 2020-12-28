use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum ApiUrl {
    Default,
    Custom(Arc<reqwest::Url>),
}

impl ApiUrl {
    pub(crate) fn get(&self) -> reqwest::Url {
        match self {
            // FIXME(waffle): parse once
            ApiUrl::Default => reqwest::Url::parse(crate::net::TELEGRAM_API_URL)
                .expect("failed to parse default url"),
            ApiUrl::Custom(url) => (&**url).clone(),
        }
    }
}
