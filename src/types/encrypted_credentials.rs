use serde::{Deserialize, Serialize};

/// Contains data required for decrypting and authenticating
/// [`EncryptedPassportElement`].
///
/// See the [Telegram Passport Documentation] for a complete description of the
/// data decryption and authentication processes.
///
/// [The official docs](https://core.telegram.org/bots/api#encryptedcredentials).
///
/// [`EncryptedPassportElement`]:
/// crate::types::EncryptedPassportElement
/// [Telegram Passport Documentation]: https://core.telegram.org/passport#receiving-information
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's
    /// payload, data hashes and secrets required for
    /// [`EncryptedPassportElement`] decryption and authentication.
    ///
    /// [`EncryptedPassportElement`]:
    /// crate::types::EncryptedPassportElement
    pub data: String, // TODO: check base64 type

    /// Base64-encoded data hash for data authentication.
    pub hash: String,

    /// A base64-encoded secret, encrypted with the bot's public RSA key,
    /// required for data decryption.
    pub secret: String,
}

impl EncryptedCredentials {
    pub fn new<S1, S2, S3>(data: S1, hash: S2, secret: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self { data: data.into(), hash: hash.into(), secret: secret.into() }
    }

    pub fn data<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.data = val.into();
        self
    }

    pub fn hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.hash = val.into();
        self
    }

    pub fn secret<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.secret = val.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_serialize_encrypted_credentials_to_json() {
        // given
        let expected_json = r#"
        {
            "data":"someData",
            "hash":"1122",
            "secret":"secret"
        }"#
        .replace("\n", "")
        .replace(" ", "");
        let encrypted_credentials = EncryptedCredentials {
            data: "someData".to_string(),
            hash: "1122".to_string(),
            secret: "secret".to_string(),
        };
        // when
        let actual_json = serde_json::to_string(&encrypted_credentials).unwrap();
        //then
        assert_eq!(actual_json, expected_json)
    }
}
