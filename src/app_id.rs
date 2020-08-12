use serde::{Deserialize, Serialize};
use std::string::ToString;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AppId(pub String);

impl std::convert::From<&str> for AppId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl std::convert::From<String> for AppId {
    fn from(id: String) -> Self {
        Self(id)
    }
}


impl Into<String> for AppId {
    fn into(self) -> String {
        self.0
    }
}

impl ToString for AppId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
