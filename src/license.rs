use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct License(pub String);

impl From<String> for License {
    fn from(l: String) -> Self {
        Self(l)
    }
}

impl From<&str> for License {
    fn from(l: &str) -> Self {
        Self(l.to_string())
    }
}

impl Into<String> for License {
    fn into(self) -> String {
        self.0
    }
}

impl ToString for License {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
