mod api;
mod download;

use reqwest::r#async::Client;

use crate::requests::RequestContext;

pub struct Bot {
    token: String,
    client: Client,
}

/// Constructors
impl Bot {
    pub fn new(token: &str) -> Self {
        Bot {
            token: String::from(token),
            client: Client::new(),
        }
    }

    pub fn with_client(token: &str, client: Client) -> Self {
        Bot {
            token: String::from(token),
            client,
        }
    }
}

impl Bot {
    fn ctx(&self) -> RequestContext {
        RequestContext {
            token: &self.token,
            client: &self.client,
        }
    }
}
