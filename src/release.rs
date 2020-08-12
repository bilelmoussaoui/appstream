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
    pub kind: ReleaseKind,
    #[serde(default, rename = "size")]
    pub sizes: Vec<ReleaseSize>,
}

impl Default for Release {
    fn default() -> Self {
        Self {
            date: None,
            date_eol: None,
            kind: ReleaseKind::Stable,
            sizes: vec![],
            version: "".to_string(),
        }
    }
}

pub struct ReleaseBuilder {
    pub date: Option<DateTime<Utc>>,
    pub date_eol: Option<DateTime<Utc>>,
    pub version: String,
    pub kind: Option<ReleaseKind>,
    pub sizes: Vec<ReleaseSize>,
}

impl Default for ReleaseBuilder {
    fn default() -> Self {
        Self {
            date: None,
            date_eol: None,
            kind: Some(ReleaseKind::Stable),
            sizes: vec![],
            version: "".to_string(),
        }
    }
}
#[allow(dead_code)]
impl ReleaseBuilder {
    pub fn new(version: String) -> Self {
        let mut r = ReleaseBuilder::default();
        r.version = version;
        r
    }

    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }
    pub fn date_eol(mut self, date_eol: DateTime<Utc>) -> Self {
        self.date_eol = Some(date_eol);
        self
    }
    pub fn kind(mut self, kind: ReleaseKind) -> Self {
        self.kind = Some(kind);
        self
    }
    pub fn sizes(mut self, sizes: Vec<ReleaseSize>) -> Self {
        self.sizes = sizes;
        self
    }

    pub fn build(self) -> Release {
        let mut r = Release::default();
        r.version = self.version;
        r.date = self.date;
        r.date_eol = self.date_eol;
        if let Some(kind) = self.kind {
            r.kind = kind;
        }
        r.sizes = self.sizes;
        r
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseKind {
    Stable,
    Development,
}

impl Default for ReleaseKind {
    fn default() -> Self {
        ReleaseKind::Stable
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
    use super::{Release, ReleaseBuilder, ReleaseSize};
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
                ReleaseBuilder::new("1.8".to_string())
                    .date(chrono::Utc.datetime_from_str("1424116753", "%s").unwrap())
                    .sizes(vec![
                        ReleaseSize::Download(12345678),
                        ReleaseSize::Installed(42424242)
                    ])
                    .build(),
                ReleaseBuilder::new("1.2".to_string())
                    .date(chrono::Utc.datetime_from_str("1397253600", "%s").unwrap())
                    .build(),
                ReleaseBuilder::new("1.0".to_string())
                    .date(chrono::Utc.datetime_from_str("1345932000", "%s").unwrap())
                    .build()
            ]
        )
    }
}
