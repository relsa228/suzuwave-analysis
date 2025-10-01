use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("Vibric file: bad signature")]
    VibricSignature,

    #[error("Unsupported file type")]
    UnsupportedType,

    #[error("File extension parse error")]
    ExtensionParseError,

    #[error("File path parse error")]
    PathParseError,
}
