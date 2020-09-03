use thiserror::Error;

#[derive(Debug, Error)]
/// Error happened during the parsing process.
pub enum ParseError {
    #[error("XML parser error: {0}")]
    /// Xml error.
    XmlParserError(#[from] xmltree::ParseError),

    #[error("strum error: {0}")]
    /// Strum error.
    StrumParseError(#[from] strum::ParseError),

    #[error("URL parser error: {0}")]
    /// url failed to parse a URL.
    UrlParseError(#[from] url::ParseError),

    #[error("Input/output error: {0} ")]
    /// IO.
    IOError(#[from] std::io::Error),

    #[error("Missing attribute: {0} for tag: {1}")]
    /// A required attribute is missing.
    MissingAttribute(String, String),

    #[error("Invalid value: {0} for attribute {1} for tag {2}")]
    /// A value passed to an attribute for a specific tag is invalid.
    InvalidValue(String, String, String),
}
