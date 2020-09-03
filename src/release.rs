use super::enums::{ArtifactKind, Bundle, Checksum, ReleaseKind, ReleaseUrgency, Size};
use super::MarkupTranslatableString;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Represents the metainformation that defines a Release.
/// See [\<releases\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-releases).
pub struct Release {
    #[serde(default, alias = "timestamp", skip_serializing_if = "Option::is_none")]
    /// The release date.
    pub date: Option<DateTime<Utc>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The end-of-life date of the release.
    pub date_eol: Option<DateTime<Utc>>,
    /// The release version
    pub version: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// A long description of the release.
    pub description: Option<MarkupTranslatableString>,

    #[serde(default, rename = "type")]
    /// The release type.
    pub kind: ReleaseKind,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Defines the downloaded & installed sizes of the release.
    pub sizes: Vec<Size>,

    #[serde(default)]
    /// The urgency to install this release.
    pub urgency: ReleaseUrgency,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Defines the different artifacts shipped with the release.
    pub artifacts: Vec<Artifact>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// A web page with the release changelog.
    pub url: Option<Url>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Defines the release artifacts, whether it's the source-code or the binary distribution.
/// See [\<releases\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-releases).
pub struct Artifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The targeted platform of the artifact.
    pub platform: Option<String>,

    #[serde(rename = "type")]
    /// The artifact type.
    pub kind: ArtifactKind,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Downloaded & installed sizes.
    pub sizes: Vec<Size>,

    /// Download link.
    pub url: Url,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// At least one checksum of released artifact.
    pub checksums: Vec<Checksum>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// 3rd-party bundles from where you can grab this release.
    pub bundles: Vec<Bundle>,
}

#[cfg(test)]
mod tests {
    use super::{
        ArtifactKind, Checksum, MarkupTranslatableString, Release, ReleaseKind, ReleaseUrgency,
        Size, Url,
    };
    use crate::builders::{ArtifactBuilder, ReleaseBuilder};
    use chrono::{TimeZone, Utc};
    use std::convert::TryFrom;

    #[test]
    fn release_artifacts() {
        let x = r"
        <releases>
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
        <release version='1.0' date='2012-08-26' />
        </releases>";

        let element = xmltree::Element::parse(x.as_bytes()).unwrap();
        let mut releases1: Vec<Release> = vec![];
        for e in element.children.iter() {
            releases1.push(Release::try_from(e.as_element().unwrap()).unwrap());
        }

        let releases2 = vec![
            ReleaseBuilder::new("1.2")
                .urgency(ReleaseUrgency::High)
                .description(MarkupTranslatableString::with_default(
                    "<p>This stable release fixes bugs.</p>",
                ))
                .date(Utc.ymd(2014, 4, 12).and_hms_milli(0, 0, 0, 0))
                .url(Url::parse("https://example.org/releases/version-1.2.html").unwrap())
                .artifact(
                    ArtifactBuilder::default()
                        .url(Url::parse("https://example.com/mytarball.bin.tar.xz").unwrap())
                        .kind(ArtifactKind::Binary)
                        .platform("x86_64-linux-gnu")
                        .size(Size::Download(12345678))
                        .size(Size::Installed(42424242))
                        .checksum(Checksum::Sha256("....".into()))
                        .checksum(Checksum::Blake2b("....".into()))
                        .build(),
                )
                .artifact(
                    ArtifactBuilder::default()
                        .url(Url::parse("https://example.com/mytarball.bin.exe").unwrap())
                        .kind(ArtifactKind::Binary)
                        .platform("win32")
                        .build(),
                )
                .artifact(
                    ArtifactBuilder::default()
                        .url(Url::parse("https://example.com/mytarball.tar.xz").unwrap())
                        .kind(ArtifactKind::Source)
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
        <releases>
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
        </releases>
        ";
        let element = xmltree::Element::parse(x.as_bytes()).unwrap();
        let mut releases: Vec<Release> = vec![];
        for e in element.children.iter() {
            releases.push(Release::try_from(e.as_element().unwrap()).unwrap());
        }

        assert_eq!(
            releases,
            vec![
                ReleaseBuilder::new("1.8")
                    .description(MarkupTranslatableString::with_default("<p>This stable release fixes the following bug:</p><ul><li>CPU no longer overheats when you hold down spacebar</li></ul>"))
                    .date(Utc.datetime_from_str("1424116753", "%s").unwrap())
                    .sizes(vec![Size::Download(12345678), Size::Installed(42424242)])
                    .build(),
                ReleaseBuilder::new("1.2")
                    .date(Utc.datetime_from_str("1397253600", "%s").unwrap())
                    .build(),
                ReleaseBuilder::new("1.0")
                    .date(Utc.datetime_from_str("1345932000", "%s").unwrap())
                    .build()
            ]
        )
    }
}
