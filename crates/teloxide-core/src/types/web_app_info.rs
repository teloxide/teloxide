use serde::{Deserialize, Serialize};
use url::Url;

/// Contains information about a [Web App].
///
/// [Web App]: https://core.telegram.org/bots/webapps
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified
    /// in [Initializing Web Apps].
    ///
    /// [Initializing Web Apps]: https://core.telegram.org/bots/webapps#initializing-web-apps
    pub url: Url,
}
