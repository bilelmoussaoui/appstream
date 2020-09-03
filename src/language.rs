use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
/// Defines how well a language is supported by the component.
/// It provides access to a the locale and the percentage of the translation completion.
pub struct Language {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The percentage of translation completion.
    pub percentage: Option<u32>,
    /// The language locale.
    pub locale: String,
}
