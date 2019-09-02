use super::Request;
use crate::core::other::User;

use reqwest::r#async::multipart::Form;

#[derive(Debug, Constructor, PartialEq, Eq)]
pub struct GetMe {
    token: String,
}

impl Request for GetMe {
    type ReturnValue = User;

    fn name(&self) -> &str {
        "getMe"
    }
    fn params(self) -> Option<Form> {
        None
    }
    fn token(&self) -> &str {
        &self.token
    }
}
