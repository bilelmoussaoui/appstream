use super::de::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Release {
    #[serde(
        deserialize_with = "timestamp_deserialize",
        alias = "timestamp",
        default
    )]
    pub date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "timestamp_deserialize", default)]
    pub date_eol: Option<DateTime<Utc>>,
    pub version: String,
    #[serde(rename = "type", default)]
    pub _type: ReleaseType,
    #[serde(default, rename = "size")]
    pub sizes: Vec<ReleaseSize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseType {
    Stable,
    Development,
}

impl Default for ReleaseType {
    fn default() -> Self {
        ReleaseType::Stable
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "$value", rename_all = "kebab-case")]
pub enum ReleaseSize {
    Download(u64),
    Installed(u64),
}

#[cfg(test)]
mod tests {
    use super::{Release, ReleaseSize, ReleaseType};
    use chrono::TimeZone;
    use quick_xml::de::from_str;
    #[test]
    fn release_size() {
        let x = r"
            <release version='1.8' timestamp='1424116753'>
                <description>
                <p>This stable release fixes the following bug:</p>
                <ul>
                    <li>CPU no longer overheats when you hold down spacebar</li>
                </ul>
                </description>
                <size type='download'>12345678</size>
                <size type='installed'>42424242</size>
            </release>
            <release version='1.2' timestamp='1397253600' />
            <release version='1.0' timestamp='1345932000' />
        ";
        let releases: Vec<Release> = from_str(&x).unwrap();

        assert_eq!(
            releases,
            vec![
                Release {
                    date: Some(chrono::Utc.datetime_from_str("1424116753", "%s").unwrap()),
                    date_eol: None,
                    _type: ReleaseType::default(),
                    version: "1.8".to_string(),
                    sizes: vec![
                        ReleaseSize::Download(12345678),
                        ReleaseSize::Installed(42424242)
                    ]
                },
                Release {
                    date: Some(chrono::Utc.datetime_from_str("1397253600", "%s").unwrap()),
                    date_eol: None,
                    _type: ReleaseType::default(),
                    version: "1.2".to_string(),
                    sizes: vec![]
                },
                Release {
                    date: Some(chrono::Utc.datetime_from_str("1345932000", "%s").unwrap()),
                    date_eol: None,
                    _type: ReleaseType::default(),
                    version: "1.0".to_string(),
                    sizes: vec![]
                }
            ]
        )
    }
}
