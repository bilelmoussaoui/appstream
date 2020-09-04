use thiserror::Error;

#[derive(Debug, Error)]
/// Error happened during the parsing process.
pub enum ParseError {
    #[error("XML parser error: {0}")]
    /// Xml error.
    XmlParserError(#[from] xmltree::ParseError),

    #[error("URL parser error: {0}")]
    /// url failed to parse a URL.
    UrlParseError(#[from] url::ParseError),

    #[error("chrono parser error: {0}")]
    /// url failed to parse a URL.
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Input/output error: {0} ")]
    /// IO.
    IOError(#[from] std::io::Error),

    #[error("Invalid tag: {0}")]
    /// The expected tag is misused.
    InvalidTag(String),

    #[error("A required tag is missing: {0}")]
    /// Required tag is missing.
    MissingTag(String),

    #[error("Missing attribute {0} required by tag {1}")]
    /// A required attribute is missing.
    MissingAttribute(String, String),

    #[error("The tag {0} doesn't have a value")]
    /// A missing value that's required.
    MissingValue(String),

    #[error("Invalid value {0} passed to attribute {1} for tag {2}")]
    /// A value passed to an attribute for a specific tag is invalid.
    InvalidValue(String, String, String),
}
