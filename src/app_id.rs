use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AppId(pub String);

impl TryFrom<&str> for AppId {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Validate the app-id
        Ok(AppId(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn validate_app_id() {
        let app_id = AppId::try_from("org.gnome.app");
        assert_eq!(app_id.is_ok(), true);
    }
    /*
    #[test]
    fn invalid_app_id() {
        let app_id = AppId::try_from("something");
        assert_eq!(app_id.is_err(), true);
    }*/
}
