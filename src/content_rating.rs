use super::enums::{ContentAttribute, ContentRatingVersion};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Defines an Open Age Rating service.
/// See [OARS](https://hughsie.github.io/oars/index.html) for more information.
pub struct ContentRating {
    #[serde(default, rename = "type")]
    /// The version of the OARS specification.
    pub version: ContentRatingVersion,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// A list of attributes that defines the OARS.
    pub attributes: Vec<ContentAttribute>,
}
