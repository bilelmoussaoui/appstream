use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::string::ToString;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AppId(pub String);

impl TryFrom<&str> for AppId {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Validate the app-id
        Ok(AppId(value.to_string()))
    }
}

impl Into<String> for AppId {
    fn into(self) -> String {
        self.0
    }
}

impl ToString for AppId {
    fn to_string(&self) -> String {
        self.0.clone()
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
        assert_eq!(app_id.unwrap().to_string(), "org.gnome.app".to_string());
    }
    /*
    #[test]
    fn invalid_app_id() {
        let app_id = AppId::try_from("something");
        assert_eq!(app_id.is_err(), true);
    }*/
}
