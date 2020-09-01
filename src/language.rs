use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Language {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<u32>,

    pub locale: String,
}
