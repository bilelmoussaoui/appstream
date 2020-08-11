use super::enums::ContentAttribute;
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContentRating {
    #[serde(rename = "type", default)]
    pub version: ContentRatingVersion,
    #[serde(rename = "content_attribute", default)]
    attributes: Vec<ContentAttribute>,
}

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub enum ContentRatingVersion {
    #[serde(rename = "oars-1.0")]
    Oars1_0,
    #[serde(rename = "oars-1.1")]
    Oars1_1,
    Unknown,
}
impl Default for ContentRatingVersion {
    fn default() -> Self {
        ContentRatingVersion::Unknown
    }
}

impl Ord for ContentRatingVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ContentRatingVersion::Oars1_0, ContentRatingVersion::Oars1_1) => Ordering::Less,
            (ContentRatingVersion::Oars1_1, ContentRatingVersion::Oars1_0) => Ordering::Greater,
            (ContentRatingVersion::Oars1_0, ContentRatingVersion::Oars1_0)
            | (ContentRatingVersion::Oars1_1, ContentRatingVersion::Oars1_1) => Ordering::Equal,
            (ContentRatingVersion::Unknown, _) | (_, ContentRatingVersion::Unknown) => {
                Ordering::Less
            }
        }
    }
}

impl PartialOrd for ContentRatingVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
