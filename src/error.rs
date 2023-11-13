use toml::de::Error;

pub type Result<T> = std::result::Result<T, HtomlError>;

#[derive(Debug)]
pub enum HtomlError {
    InvalidToml(Error),
    UndeclaredFile,
    ReadFileError,
    WriteFileError,
    NonTableHead,
    UnknownHead(String),
    UnknownElement(String),
    UntypedElement,
    NoContent,
    UnknownContent,
    NonStrLang,
    AWithoutHref,
}
