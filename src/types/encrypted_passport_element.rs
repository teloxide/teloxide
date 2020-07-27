use serde::{Deserialize, Serialize};

use super::PassportFile;

/// Contains information about documents or other Telegram Passport elements
/// shared with the bot by the user.
///
/// [The official docs](https://core.telegram.org/bots/api#encryptedpassportelement).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElement {
    /// Base64-encoded element hash for using in
    /// [`PassportElementErrorKind::Unspecified`].
    ///
    /// [`PassportElementErrorKind::Unspecified`]:
    /// crate::types::PassportElementErrorKind::Unspecified
    pub hash: String,

    #[serde(flatten)]
    pub kind: EncryptedPassportElementKind,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum EncryptedPassportElementKind {
    PersonalDetails(EncryptedPassportElementPersonalDetails),
    Passport(EncryptedPassportElementPassport),
    DriverLicense(EncryptedPassportElementDriverLicense),
    IdentityCard(EncryptedPassportElementIdentityCard),
    InternalPassport(EncryptedPassportElementInternalPassport),
    Address(EncryptedPassportElementAddress),
    UtilityBill(EncryptedPassportElementUtilityBill),
    BankStatement(EncryptedPassportElementBankStatement),
    RentalAgreement(EncryptedPassportElementRentalAgreement),
    PassportRegistration(EncryptedPassportElementPassportRegistration),
    EncryptedPassportElement(EncryptedPassportElementTemporaryRegistration),
    PhoneNumber(EncryptedPassportElementPhoneNumber),
    Email(EncryptedPassportElementEmail),
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementPersonalDetails {
    ///  Base64-encoded encrypted Telegram Passport element data provided
    /// by the user, available for `personal_details`, `passport`,
    /// `driver_license`, `identity_card`, `internal_passport` and
    /// `address` types. Can be decrypted and verified using the
    /// accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub data: String,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementPassport {
    ///  Base64-encoded encrypted Telegram Passport element data provided
    /// by the user, available for `personal_details`, `passport`,
    /// `driver_license`, `identity_card`, `internal_passport` and
    /// `address` types. Can be decrypted and verified using the
    /// accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub data: String,

    /// Encrypted file with the front side of the document, provided by the
    /// user. Available for `passport`, `driver_license`, `identity_card`
    /// and `internal_passport`. The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub front_side: PassportFile,

    /// Encrypted file with the selfie of the user holding a document,
    /// provided by the user; available for `passport`, `driver_license`,
    /// `identity_card` and `internal_passport`. The file can be decrypted
    /// and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub selfie: PassportFile,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementDriverLicense {
    ///  Base64-encoded encrypted Telegram Passport element data provided
    /// by the user, available for `personal_details`, `passport`,
    /// `driver_license`, `identity_card`, `internal_passport` and
    /// `address` types. Can be decrypted and verified using the
    /// accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub data: String,

    /// Encrypted file with the front side of the document, provided by the
    /// user. Available for `passport`, `driver_license`, `identity_card`
    /// and `internal_passport`. The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub front_side: PassportFile,

    /// Encrypted file with the reverse side of the document, provided by
    /// the user. Available for `driver_license` and `identity_card`. The
    /// file can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub reverse_side: PassportFile,

    /// Encrypted file with the selfie of the user holding a document,
    /// provided by the user; available for `passport`, `driver_license`,
    /// `identity_card` and `internal_passport`. The file can be decrypted
    /// and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub selfie: PassportFile,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementIdentityCard {
    ///  Base64-encoded encrypted Telegram Passport element data provided
    /// by the user, available for `personal_details`, `passport`,
    /// `driver_license`, `identity_card`, `internal_passport` and
    /// `address` types. Can be decrypted and verified using the
    /// accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub data: String,

    /// Encrypted file with the front side of the document, provided by the
    /// user. Available for `passport`, `driver_license`, `identity_card`
    /// and `internal_passport`. The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub front_side: PassportFile,

    /// Encrypted file with the reverse side of the document, provided by
    /// the user. Available for `driver_license` and `identity_card`. The
    /// file can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub reverse_side: PassportFile,

    /// Encrypted file with the selfie of the user holding a document,
    /// provided by the user; available for `passport`, `driver_license`,
    /// `identity_card` and `internal_passport`. The file can be decrypted
    /// and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub selfie: PassportFile,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementInternalPassport {
    ///  Base64-encoded encrypted Telegram Passport element data provided
    /// by the user, available for `personal_details`, `passport`,
    /// `driver_license`, `identity_card`, `internal_passport` and
    /// `address` types. Can be decrypted and verified using the
    /// accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub data: String,

    /// Encrypted file with the front side of the document, provided by the
    /// user. Available for `passport`, `driver_license`, `identity_card`
    /// and `internal_passport`. The file can be decrypted and verified
    /// using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub front_side: PassportFile,

    /// Encrypted file with the selfie of the user holding a document,
    /// provided by the user; available for `passport`, `driver_license`,
    /// `identity_card` and `internal_passport`. The file can be decrypted
    /// and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub selfie: PassportFile,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementAddress {
    ///  Base64-encoded encrypted Telegram Passport element data provided
    /// by the user, available for `personal_details`, `passport`,
    /// `driver_license`, `identity_card`, `internal_passport` and
    /// `address` types. Can be decrypted and verified using the
    /// accompanying [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub data: String,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementUtilityBill {
    /// Array of encrypted files with documents provided by the user,
    /// available for `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub files: Vec<PassportFile>,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementBankStatement {
    /// Array of encrypted files with documents provided by the user,
    /// available for `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub files: Vec<PassportFile>,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementRentalAgreement {
    /// Array of encrypted files with documents provided by the user,
    /// available for `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub files: Vec<PassportFile>,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementPassportRegistration {
    /// Array of encrypted files with documents provided by the user,
    /// available for `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub files: Vec<PassportFile>,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementTemporaryRegistration {
    /// Array of encrypted files with documents provided by the user,
    /// available for `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub files: Vec<PassportFile>,

    /// Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for `passport`,
    /// `driver_license`, `identity_card`, `internal_passport`,
    /// `utility_bill`, `bank_statement`, `rental_agreement`,
    /// `passport_registration` and `temporary_registration` types. Files
    /// can be decrypted and verified using the accompanying
    /// [`EncryptedCredentials`].
    ///
    /// [`EncryptedCredentials`]:
    /// crate::types::EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementPhoneNumber {
    /// User's verified phone number, available only for `phone_number`
    /// type.
    pub phone_number: String,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedPassportElementEmail {
    /// User's verified email address, available only for `email` type.
    pub email: String,
}
