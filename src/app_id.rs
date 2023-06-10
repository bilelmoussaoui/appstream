use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Unique identifier of a component. It should be reverse-DNS name.
pub struct AppId(pub String);

impl From<&str> for AppId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<String> for AppId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl fmt::Display for AppId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
