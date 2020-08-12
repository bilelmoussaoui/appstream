use super::de::*;
use super::enums::{ReleaseKind, ReleaseSize};
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

#[cfg(test)]
mod tests {
    use super::Release;
    use crate::builders::ReleaseBuilder;
    use crate::enums::ReleaseSize;
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
                ReleaseBuilder::new("1.8")
                    .date(chrono::Utc.datetime_from_str("1424116753", "%s").unwrap())
                    .sizes(vec![
                        ReleaseSize::Download(12345678),
                        ReleaseSize::Installed(42424242)
                    ])
                    .build(),
                ReleaseBuilder::new("1.2")
                    .date(chrono::Utc.datetime_from_str("1397253600", "%s").unwrap())
                    .build(),
                ReleaseBuilder::new("1.0")
                    .date(chrono::Utc.datetime_from_str("1345932000", "%s").unwrap())
                    .build()
            ]
        )
    }
}
