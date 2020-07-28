use serde::{Deserialize, Serialize};

/// This object represents an error in the Telegram Passport element which was
/// submitted that should be resolved by the user.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerror).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportElementError {
    /// Error message.
    message: String,

    #[serde(flatten)]
    kind: PassportElementErrorKind,
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

    pub fn kind(mut self, val: PassportElementErrorKind) -> Self {
        self.kind = val;
        self
    }
}

#[serde(tag = "source")]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PassportElementErrorKind {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),

    #[serde(rename = "snake_case")]
    FrontSide(PassportElementErrorFrontSide),

    #[serde(rename = "snake_case")]
    ReverseSide(PassportElementErrorReverseSide),

    #[serde(rename = "snake_case")]
    Selfie(PassportElementErrorSelfie),

    #[serde(rename = "snake_case")]
    File(PassportElementErrorFile),

    #[serde(rename = "snake_case")]
    Files(PassportElementErrorFiles),

    #[serde(rename = "snake_case")]
    TranslationFile(PassportElementErrorTranslationFile),

    #[serde(rename = "snake_case")]
    TranslationFiles(PassportElementErrorTranslationFiles),

    #[serde(rename = "snake_case")]
    Unspecified(PassportElementErrorUnspecified),
}

/// Represents an issue in one of the data fields that was provided by the
/// user.
///
/// The error is considered resolved when the field's value changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrordatafield).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

//// Represents an issue with the selfie with a document.
//
/// The error is considered resolved when the file with the selfie changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorselfie).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorSelfieType,

    /// Base64-encoded hash of the file with the selfie.
    pub file_hash: String,
}

/// Represents an issue with a document scan.
///
/// The error is considered resolved when the file with the document scan
/// changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfile).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportElementErrorFile {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFileType,

    /// Base64-encoded file hash.
    pub file_hash: String,
}

/// Represents an issue with a list of scans.
///
/// The error is considered resolved when the list of files containing the scans
/// changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfiles).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportElementErrorFiles {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFilesType,

    /// List of base64-encoded file hashes.
    pub file_hashes: Vec<String>,
}

/// Represents an issue with one of the files that constitute the
/// translation of a document.
///
/// The error is considered resolved when the file changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfile).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportElementErrorTranslationFile {
    /// Type of element of the user's Telegram Passport which has the
    /// issue.
    pub r#type: PassportElementErrorTranslationFileType,

    /// Base64-encoded file hash.
    pub file_hash: String,
}

/// Represents an issue with the translated version of a document.
///
/// The error is considered resolved when a file with the document translation
/// change.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfiles).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportElementErrorTranslationFiles {
    /// Type of element of the user's Telegram Passport which has the issue
    pub r#type: PassportElementErrorTranslationFilesType,

    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
}

/// Represents an issue in an unspecified place.
///
/// The error is considered resolved when new data is added.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorunspecified).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportElementErrorUnspecified {
    /// Type of element of the user's Telegram Passport which has the
    /// issue.
    pub r#type: PassportElementErrorUnspecifiedType,

    /// Base64-encoded element hash.
    pub element_hash: String,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
#[non_exhaustive]
pub enum PassportElementErrorFrontSideType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PassportElementErrorReverseSideType {
    DriverLicense,
    IdentityCard,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PassportElementErrorSelfieType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PassportElementErrorFileType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PassportElementErrorFilesType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
