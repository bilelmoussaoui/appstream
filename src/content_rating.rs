use super::enums::{ContentAttribute, ContentRatingVersion};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContentRating {
    #[serde(default, rename = "type")]
    pub version: ContentRatingVersion,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<ContentAttribute>,
}
