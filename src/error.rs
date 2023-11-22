use toml::de::Error;

pub type Result<T> = std::result::Result<T, HtomlError>;

#[derive(Debug)]
pub enum HtomlError {
    NoCommand,
    UnknownCommand(String),
    NoFileGiven,
    InvalidToml(Error),
    UndeclaredFile,
    ReadFileError,
    WriteFileError,
    NonTableHead,
    UnknownHead(String),
    UnknownClass,
    NonStringAttr(String),
    UntypedElement,
    NoContent,
    UnknownContent,
    NonStrLang,
}
