use serde::{Deserialize, Serialize};
use std::string::ToString;
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

impl ToString for AppId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
