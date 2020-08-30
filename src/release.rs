use super::de::*;
use super::enums::{ArtifactKind, Bundle, Checksum, ReleaseKind, ReleaseUrgency, Size};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Release {
    #[serde(
        default,
        alias = "timestamp",
        deserialize_with = "timestamp_deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub date: Option<DateTime<Utc>>,

    #[serde(
        default,
        deserialize_with = "timestamp_deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_eol: Option<DateTime<Utc>>,

    pub version: String,

    #[serde(rename = "type", default)]
    pub kind: ReleaseKind,

    #[serde(default, rename = "size", skip_serializing_if = "Vec::is_empty")]
    pub sizes: Vec<Size>,

    #[serde(default)]
    pub urgency: ReleaseUrgency,

    #[serde(
        default,
        deserialize_with = "artifacts_deserialize",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<Artifact>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Artifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    #[serde(rename = "type")]
    pub kind: ArtifactKind,

    #[serde(default, rename = "size", skip_serializing_if = "Vec::is_empty")]
    pub sizes: Vec<Size>,

    #[serde(rename = "location")]
    pub url: Url,

    #[serde(default, rename = "checksum", skip_serializing_if = "Vec::is_empty")]
    pub checksums: Vec<Checksum>,

    #[serde(default, rename = "bundle", skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<Bundle>,
}

#[cfg(test)]
mod tests {
    use super::{ArtifactKind, Checksum, Release, ReleaseKind, ReleaseUrgency, Size, Url};
    use crate::builders::{ArtifactBuilder, ReleaseBuilder};
    use quick_xml::de::from_str;

    use chrono::{TimeZone, Utc};

    #[test]
    fn release_artifacts() {
        let x = r"
        <release version='1.2' date='2014-04-12' urgency='high'>
          <description>
            <p>This stable release fixes bugs.</p>
          </description>
  
          <url>https://example.org/releases/version-1.2.html</url>
  
          <issues>
            <issue url='https://example.com/bugzilla/12345'>bz#12345</issue>
            <issue type='cve'>CVE-2019-123456</issue>
          </issues>
  
          <artifacts>
            <artifact type='binary' platform='x86_64-linux-gnu'>
              <location>https://example.com/mytarball.bin.tar.xz</location>
              <checksum type='sha256'>....</checksum>
              <checksum type='blake2b'>....</checksum>
              <size type='download'>12345678</size>
              <size type='installed'>42424242</size>
            </artifact>
            <artifact type='binary' platform='win32'>
              <location>https://example.com/mytarball.bin.exe</location>
            </artifact>
            <artifact type='source'>
              <location>https://example.com/mytarball.tar.xz</location>
              <checksum type='sha256'>....</checksum>
            </artifact>
          </artifacts>
        </release>
        <release version='1.1' type='development' date='2013-10-20' />
        <release version='1.0' date='2012-08-26' />";

        let releases1: Vec<Release> = from_str(&x).unwrap();
        let releases2 = vec![
            ReleaseBuilder::new("1.2")
                .urgency(ReleaseUrgency::High)
                .date(Utc.ymd(2014, 4, 12).and_hms_milli(0, 0, 0, 0))
                .url(Url::parse("https://example.org/releases/version-1.2.html").unwrap())
                .artifact(
                    ArtifactBuilder::new(
                        Url::parse("https://example.com/mytarball.bin.tar.xz").unwrap(),
                        ArtifactKind::Binary,
                    )
                    .platform("x86_64-linux-gnu")
                    .size(Size::Download(12345678))
                    .size(Size::Installed(42424242))
                    .checksum(Checksum::Sha256("....".into()))
                    .checksum(Checksum::Blake2b("....".into()))
                    .build(),
                )
                .artifact(
                    ArtifactBuilder::new(
                        Url::parse("https://example.com/mytarball.bin.exe").unwrap(),
                        ArtifactKind::Binary,
                    )
                    .platform("win32")
                    .build(),
                )
                .artifact(
                    ArtifactBuilder::new(
                        Url::parse("https://example.com/mytarball.tar.xz").unwrap(),
                        ArtifactKind::Source,
                    )
                    .checksum(Checksum::Sha256("....".into()))
                    .build(),
                )
                .build(),
            ReleaseBuilder::new("1.1")
                .kind(ReleaseKind::Development)
                .date(Utc.ymd(2013, 10, 20).and_hms_milli(0, 0, 0, 0))
                .build(),
            ReleaseBuilder::new("1.0")
                .date(Utc.ymd(2012, 8, 26).and_hms_milli(0, 0, 0, 0))
                .build(),
        ];
        assert_eq!(releases1, releases2);
    }

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
                    .sizes(vec![Size::Download(12345678), Size::Installed(42424242)])
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
