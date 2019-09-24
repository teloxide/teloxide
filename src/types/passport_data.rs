use super::encrypted_credintials::EncryptedCredentials;
use super::encrypted_passport_element::EncryptedPassportElement;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
