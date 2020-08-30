use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Language {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<u32>,
    #[serde(rename(deserialize = "$value", serialize = "locale"))]
    pub locale: String,
}
