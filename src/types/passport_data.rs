use serde::{Deserialize, Serialize};

use super::{EncryptedCredentials, EncryptedPassportElement};

/// Contains information about Telegram Passport data shared with the bot by the
/// user.
///
/// [The official docs](https://core.telegram.org/bots/api#passportdata).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport
    /// elements that was shared with the bot.
    pub data: Vec<EncryptedPassportElement>,

    /// Encrypted credentials required to decrypt the data.
    pub credentials: EncryptedCredentials,
}
