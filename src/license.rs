use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct License(pub String);

impl std::convert::From<&str> for License {
    fn from(l: &str) -> Self {
        Self(l.to_string())
    }
}
