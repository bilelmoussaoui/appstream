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

    #[error("Error parsing {0}: {1}")]
    /// A parsing error requiring a reason.
    Other(String, String),
}

impl ParseError {
    /// Creates an invalid value error.
    pub fn invalid_value(val: &str, attr: &str, tag: &str) -> Self {
        ParseError::InvalidValue(val.to_string(), attr.to_string(), tag.to_string())
    }

    /// Creates an invalid tag error.
    pub fn invalid_tag(tag: &str) -> Self {
        ParseError::InvalidTag(tag.to_string())
    }

    /// Creates a missing attribute error.
    pub fn missing_attribute(attr: &str, tag: &str) -> Self {
        ParseError::MissingAttribute(attr.to_string(), tag.to_string())
    }

    /// Creates a missing tag error.
    pub fn missing_tag(tag: &str) -> Self {
        ParseError::MissingTag(tag.to_string())
    }

    /// Creates a missing value error.
    pub fn missing_value(tag: &str) -> Self {
        ParseError::MissingValue(tag.to_string())
    }

    /// Creates a other error.
    pub fn other(tag: &str, reason: &str) -> Self {
        ParseError::Other(tag.to_string(), reason.to_string())
    }
}
