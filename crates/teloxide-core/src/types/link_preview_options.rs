use serde::{Deserialize, Serialize};

/// Describes the options used for link preview generation.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    /// `true`, if the link preview is disabled
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_disabled: bool,

    /// URL to use for the link preview. If empty, then the first URL found in
    /// the message text will be used
    pub url: Option<String>,

    /// `true`, if the media in the link preview is suppposed to be shrunk;
    /// ignored if the URL isn't explicitly specified or media size change isn't
    /// supported for the preview
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub prefer_small_media: bool,

    /// `true`, if the media in the link preview is suppposed to be enlarged;
    /// ignored if the URL isn't explicitly specified or media size change isn't
    /// supported for the preview
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub prefer_large_media: bool,

    /// `true`, if the link preview must be shown above the message text;
    /// otherwise, the link preview will be shown below the message text
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_above_text: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "is_disabled": true,
            "url": "https://kernel.org/",
            "prefer_small_media": false,
            "prefer_large_media": true,
            "show_above_text": true
        }
        "#;
        serde_json::from_str::<LinkPreviewOptions>(data).unwrap();
    }
}
