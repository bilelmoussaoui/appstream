use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Language {
    pub percentage: Option<u32>,
    #[serde(rename = "$value")]
    pub identifier: String,
}
