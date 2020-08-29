use super::enums::{ContentAttribute, ContentRatingVersion};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContentRating {
    #[serde(rename = "type", default)]
    pub version: ContentRatingVersion,
    #[serde(
        rename = "content_attribute",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attributes: Vec<ContentAttribute>,
}
