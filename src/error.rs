use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

use crate::collection::Collection;

#[derive(Debug, Error)]
/// Error happened during the parsing process.
pub enum ParseError {
    #[error("XML parser error: {0}")]
    /// Xml error.
    XmlParserError(#[from] xmltree::ParseError),

    #[error("URL parser error: {0}")]
    /// url failed to parse a URL.
    UrlParseError(#[from] url::ParseError),

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

#[derive(Error)]
/// Error akin to `ParseError` with context where it occurred.
pub struct ContextParseError {
    error: ParseError,
    context: Option<xmltree::Element>,
}

impl ContextParseError {
    /// Create a new error with context from error and context.
    pub fn new(error: ParseError, context: xmltree::Element) -> Self {
        Self {
            error,
            context: Some(context),
        }
    }
}

impl Debug for ContextParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let context = self
            .context
            .as_ref()
            .map_or(String::from("None"), |x| display_context(x, f, true));

        f.debug_struct("ContextParseError")
            .field("error", &self.error)
            .field("context", &context)
            .finish()
    }
}

impl Display for ContextParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.error)?;

        if let Some(context) = &self.context {
            write!(f, "\n{}", display_context(context, f, false))?;
        }

        Ok(())
    }
}

impl From<ContextParseError> for ParseError {
    fn from(error: ContextParseError) -> Self {
        error.error
    }
}

impl From<ParseError> for ContextParseError {
    fn from(error: ParseError) -> Self {
        Self {
            error,
            context: None,
        }
    }
}

pub struct CollectionParseError {
    pub errors: Vec<ContextParseError>,
    pub partial_collection: Option<Collection>,
}

impl From<ParseError> for CollectionParseError {
    fn from(error: ParseError) -> Self {
        Self {
            errors: vec![error.into()],
            partial_collection: None,
        }
    }
}

impl From<CollectionParseError> for ParseError {
    fn from(mut error: CollectionParseError) -> Self {
        error.errors.remove(0).error
    }
}

pub fn collection_from_result(
    result: Result<Collection, CollectionParseError>,
) -> (Option<Collection>, Vec<ContextParseError>) {
    match result {
        Ok(collection) => (Some(collection), Vec::new()),
        Err(err) => (err.partial_collection, err.errors),
    }
}

fn display_context(context: &xmltree::Element, f: &Formatter<'_>, debug: bool) -> String {
    let mut code = Vec::new();
    let _ = context.write_with_config(
        &mut code,
        xmltree::EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(true),
    );
    let code_string = String::from_utf8_lossy(&code).to_string();

    // Limit output of context to avoid huge error messages
    let output_limit = if f.alternate() { 4000 } else { 200 };

    let snippet = match code_string.char_indices().nth(output_limit) {
        None => code_string,
        Some((idx, _)) => format!("{} …", &code_string[..idx]),
    };

    if debug {
        snippet
    } else {
        // Prefix lines with pipes
        format!(" | {}", snippet.replace('\n', "\n | "))
    }
}
