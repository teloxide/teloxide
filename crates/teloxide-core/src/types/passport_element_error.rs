use serde::{Deserialize, Serialize};

/// This object represents an error in the Telegram Passport element which was
/// submitted that should be resolved by the user.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerror).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementError {
    /// Error message.
    pub message: String,

    #[serde(flatten)]
    pub kind: PassportElementErrorKind,
}

impl PassportElementError {
    pub fn new<S>(message: S, kind: PassportElementErrorKind) -> Self
    where
        S: Into<String>,
    {
        Self { message: message.into(), kind }
    }

    pub fn message<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.message = val.into();
        self
    }

    #[must_use]
    pub fn kind(mut self, val: PassportElementErrorKind) -> Self {
        self.kind = val;
        self
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "source")]
pub enum PassportElementErrorKind {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),
    FrontSide(PassportElementErrorFrontSide),
    ReverseSide(PassportElementErrorReverseSide),
    Selfie(PassportElementErrorSelfie),
    File(PassportElementErrorFile),
    Files(PassportElementErrorFiles),
    TranslationFile(PassportElementErrorTranslationFile),
    TranslationFiles(PassportElementErrorTranslationFiles),
    Unspecified(PassportElementErrorUnspecified),
}

/// Represents an issue in one of the data fields that was provided by the
/// user.
///
/// The error is considered resolved when the field's value changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrordatafield).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorDataField {
    /// The section of the user's Telegram Passport which has the error.
    pub r#type: PassportElementErrorDataFieldType,

    /// Name of the data field which has the error.
    pub field_name: String,

    /// Base64-encoded data hash.
    pub data_hash: String,
}

impl PassportElementErrorDataField {
    pub fn new<S1, S2>(
        r#type: PassportElementErrorDataFieldType,
        field_name: S1,
        data_hash: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self { r#type, field_name: field_name.into(), data_hash: data_hash.into() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorDataFieldType) -> Self {
        self.r#type = val;
        self
    }

    pub fn field_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.field_name = val.into();
        self
    }

    pub fn data_hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.data_hash = val.into();
        self
    }
}

/// Represents an issue with the front side of a document.
///
/// The error is considered resolved when the file with the front side of the
/// document changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfrontside).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFrontSideType,

    /// Base64-encoded hash of the file with the front side of the
    /// document.
    pub file_hash: String,
}

impl PassportElementErrorFrontSide {
    pub fn new<S>(r#type: PassportElementErrorFrontSideType, file_hash: S) -> Self
    where
        S: Into<String>,
    {
        Self { r#type, file_hash: file_hash.into() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorFrontSideType) -> Self {
        self.r#type = val;
        self
    }

    pub fn file_hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_hash = val.into();
        self
    }
}

/// Represents an issue with the reverse side of a document.
///
/// The error is considered resolved when the file with reverse side of the
/// document changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorreverseside).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorReverseSide {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorReverseSideType,

    //// Base64-encoded hash of the file with the reverse side of the
    //// document.
    pub file_hash: String,
}

impl PassportElementErrorReverseSide {
    pub fn new<S>(r#type: PassportElementErrorReverseSideType, file_hash: S) -> Self
    where
        S: Into<String>,
    {
        Self { r#type, file_hash: file_hash.into() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorReverseSideType) -> Self {
        self.r#type = val;
        self
    }

    pub fn file_hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_hash = val.into();
        self
    }
}

/// Represents an issue with the selfie with a document.
///
/// The error is considered resolved when the file with the selfie changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorselfie).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorSelfieType,

    /// Base64-encoded hash of the file with the selfie.
    pub file_hash: String,
}

impl PassportElementErrorSelfie {
    pub fn new<S>(r#type: PassportElementErrorSelfieType, file_hash: S) -> Self
    where
        S: Into<String>,
    {
        Self { r#type, file_hash: file_hash.into() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorSelfieType) -> Self {
        self.r#type = val;
        self
    }

    pub fn file_hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_hash = val.into();
        self
    }
}

/// Represents an issue with a document scan.
///
/// The error is considered resolved when the file with the document scan
/// changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfile).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFileType,

    /// Base64-encoded file hash.
    pub file_hash: String,
}

impl PassportElementErrorFile {
    pub fn new<S>(r#type: PassportElementErrorFileType, file_hash: S) -> Self
    where
        S: Into<String>,
    {
        Self { r#type, file_hash: file_hash.into() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorFileType) -> Self {
        self.r#type = val;
        self
    }

    pub fn file_hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_hash = val.into();
        self
    }
}

/// Represents an issue with a list of scans.
///
/// The error is considered resolved when the list of files containing the scans
/// changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfiles).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFilesType,

    /// List of base64-encoded file hashes.
    pub file_hashes: Vec<String>,
}

impl PassportElementErrorFiles {
    pub fn new<S>(r#type: PassportElementErrorFilesType, file_hashes: S) -> Self
    where
        S: IntoIterator<Item = String>,
    {
        Self { r#type, file_hashes: file_hashes.into_iter().collect() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorFilesType) -> Self {
        self.r#type = val;
        self
    }

    pub fn file_hashes<S>(mut self, val: S) -> Self
    where
        S: IntoIterator<Item = String>,
    {
        self.file_hashes = val.into_iter().collect();
        self
    }
}

/// Represents an issue with one of the files that constitute the
/// translation of a document.
///
/// The error is considered resolved when the file changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfile).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Type of element of the user's Telegram Passport which has the
    /// issue.
    pub r#type: PassportElementErrorTranslationFileType,

    /// Base64-encoded file hash.
    pub file_hash: String,
}

impl PassportElementErrorTranslationFile {
    pub fn new<S>(r#type: PassportElementErrorTranslationFileType, file_hash: S) -> Self
    where
        S: Into<String>,
    {
        Self { r#type, file_hash: file_hash.into() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorTranslationFileType) -> Self {
        self.r#type = val;
        self
    }

    pub fn file_hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_hash = val.into();
        self
    }
}

/// Represents an issue with the translated version of a document.
///
/// The error is considered resolved when a file with the document translation
/// change.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfiles).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Type of element of the user's Telegram Passport which has the issue
    pub r#type: PassportElementErrorTranslationFilesType,

    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
}

impl PassportElementErrorTranslationFiles {
    pub fn new<S>(r#type: PassportElementErrorTranslationFilesType, file_hashes: S) -> Self
    where
        S: IntoIterator<Item = String>,
    {
        Self { r#type, file_hashes: file_hashes.into_iter().collect() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorTranslationFilesType) -> Self {
        self.r#type = val;
        self
    }

    pub fn file_hashes<S>(mut self, val: S) -> Self
    where
        S: IntoIterator<Item = String>,
    {
        self.file_hashes = val.into_iter().collect();
        self
    }
}

/// Represents an issue in an unspecified place.
///
/// The error is considered resolved when new data is added.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorunspecified).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    /// Type of element of the user's Telegram Passport which has the
    /// issue.
    pub r#type: PassportElementErrorUnspecifiedType,

    /// Base64-encoded element hash.
    pub element_hash: String,
}

impl PassportElementErrorUnspecified {
    pub fn new<S>(r#type: PassportElementErrorUnspecifiedType, file_hash: S) -> Self
    where
        S: Into<String>,
    {
        Self { r#type, element_hash: file_hash.into() }
    }

    #[must_use]
    pub fn r#type(mut self, val: PassportElementErrorUnspecifiedType) -> Self {
        self.r#type = val;
        self
    }

    pub fn element_hash<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.element_hash = val.into();
        self
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorDataFieldType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFrontSideType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorReverseSideType {
    DriverLicense,
    IdentityCard,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorSelfieType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFileType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFilesType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorTranslationFileType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorTranslationFilesType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorUnspecifiedType {
    DataField,
    FrontSide,
    ReverseSide,
    Selfie,
    File,
    Files,
    TranslationFile,
    TranslationFiles,
    Unspecified,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_data_field() {
        let data = PassportElementError {
            message: "This is an error message!".to_owned(),
            kind: PassportElementErrorKind::DataField(PassportElementErrorDataField {
                r#type: PassportElementErrorDataFieldType::InternalPassport,
                field_name: "The field name".to_owned(),
                data_hash: "This is a data hash".to_owned(),
            }),
        };

        assert_eq!(
            serde_json::to_string(&data).unwrap(),
            r#"{"message":"This is an error message!","source":"data","type":"internal_passport","field_name":"The field name","data_hash":"This is a data hash"}"#
        );
    }
}
