use super::{EncryptedCredentials, EncryptedPassportElement};

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
