use super::de::*;
use super::enums::{
    Bundle, Category, ComponentType, Icon, Kudo, Launchable, ProjectUrl, Provide, Translation,
};
use super::translatable_string::{TranslatableString, TranslatableVec};
use super::{AppId, ContentRating, Language, License, Release, Screenshot};
use anyhow::Result;
#[cfg(feature="gzip")]
use flate2::read::GzDecoder;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
#[cfg(feature="gzip")]
use std::fs::File;
#[cfg(feature="gzip")]
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Component {
    #[serde(rename = "type", default)]
    pub _type: ComponentType,
    #[serde(deserialize_with = "app_id_deserialize")]
    pub id: AppId,
    #[serde(rename = "name", deserialize_with = "translatable_deserialize")]
    pub name: TranslatableString,
    #[serde(
        rename = "summary",
        deserialize_with = "some_translatable_deserialize",
        default
    )]
    pub summary: Option<TranslatableString>,
    #[serde(default, deserialize_with = "license_deserialize")]
    pub project_license: Option<License>,
    #[serde(default, deserialize_with = "license_deserialize")]
    pub metadata_license: Option<License>,
    pub project_group: Option<String>,
    pub compulsory_for_desktop: Option<String>,
    #[serde(default, deserialize_with = "extends_deserialize")]
    pub extends: Vec<AppId>,

    #[serde(rename = "icon", deserialize_with = "icon_deserialize", default)]
    pub icons: Vec<Icon>,
    #[serde(deserialize_with = "screenshots_deserialize", default)]
    pub screenshots: Vec<Screenshot>,
    #[serde(rename = "url", deserialize_with = "urls_deserialize", default)]
    pub urls: Vec<ProjectUrl>,
    #[serde(
        rename = "developer_name",
        deserialize_with = "some_translatable_deserialize",
        default
    )]
    pub developer_name: Option<TranslatableString>,
    pub update_contact: Option<String>,
    #[serde(default, deserialize_with = "category_deserialize")]
    pub categories: Vec<Category>,
    #[serde(
        rename = "launchable",
        deserialize_with = "launchable_deserialize",
        default
    )]
    pub launchables: Vec<Launchable>,
    #[serde(default)]
    pub pkgname: Option<String>,
    #[serde(rename = "bundle", deserialize_with = "bundle_deserialize", default)]
    pub bundle: Vec<Bundle>,
    #[serde(default, deserialize_with = "releases_deserialize")]
    pub releases: Vec<Release>,
    #[serde(deserialize_with = "languages_deserialize", default)]
    pub languages: Vec<Language>,

    #[serde(default, deserialize_with = "mimetypes_deserialize")]
    pub mimetypes: Vec<String>,
    #[serde(default, deserialize_with = "kudos_deserialize")]
    pub kudos: Vec<Kudo>,

    #[serde(default, deserialize_with = "keywords_deserialize")]
    pub keywords: TranslatableVec,
    #[serde(default, deserialize_with = "content_rating_deserialize")]
    pub content_rating: Option<ContentRating>,
    #[serde(default, deserialize_with = "provides_deserialize")]
    pub provides: Vec<Provide>,
    #[serde(default, deserialize_with = "translation_deserialize")]
    pub translation: Vec<Translation>,
}

impl Component {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let xml = std::fs::read_to_string(path)?;

        let component: Component = from_str(&xml)?;
        Ok(component)
    }

    #[cfg(feature="gzip")]
    pub fn from_gzipped(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;

        let mut d = GzDecoder::new(f);
        let mut xml = String::new();
        d.read_to_string(&mut xml)?;

        let component: Component = from_str(&xml)?;
        Ok(component)
    }
}

#[cfg(test)]
mod tests {

    use super::Component;
    use crate::enums::{ComponentType, Launchable, ProjectUrl, Provide};
    use crate::translatable_string::TranslatableString;
    use crate::{AppId, Language, Release, ReleaseType};
    use chrono::{TimeZone, Utc};
    use std::convert::TryFrom;
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn desktop_application_component() {
        let c: Component = Component::from_path("./tests/desktop.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::DesktopApplication);
        assert_eq!(
            c.provides,
            vec![
                Provide::Binary("gnome-power-statistics".into()),
                Provide::Id("gnome-power-statistics.desktop".into())
            ]
        );
        assert_eq!(
            c.launchables,
            vec![Launchable::DesktopId(
                "org.gnome.gnome-power-statistics.desktop".to_string()
            )]
        );
        assert_eq!(c.project_group, Some("GNOME".into()));
    }

    #[test]
    fn runtime_component() {
        let c: Component = Component::from_path("./tests/runtime.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::Runtime);
    }

    #[test]
    fn os_component() {
        let c: Component = Component::from_path("./tests/os.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::OS);
        assert_eq!(
            c.releases,
            vec![
                Release {
                    version: "10.0".into(),
                    date: None,
                    date_eol: None,
                    _type: ReleaseType::Development,
                    sizes: vec![]
                },
                Release {
                    version: "9.0".into(),
                    date: Some(Utc.ymd(2017, 7, 17).and_hms_milli(0, 0, 0, 0)),
                    date_eol: Some(Utc.ymd(2020, 7, 17).and_hms_milli(0, 0, 0, 0)),
                    _type: ReleaseType::default(),
                    sizes: vec![]
                },
            ]
        );
    }

    #[test]
    fn localization_component() {
        let c = Component::from_path("./tests/localization.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::Localization);
        assert_eq!(
            c.extends,
            vec![
                AppId::try_from("org.kde.plasmashell").unwrap(),
                AppId::try_from("org.kde.gwenview.desktop").unwrap(),
                AppId::try_from("org.kde.dolphin.desktop").unwrap(),
            ]
        );

        assert_eq!(
            c.languages,
            vec![
                Language {
                    identifier: "de_DE".into(),
                    percentage: None
                },
                Language {
                    identifier: "de_AT".into(),
                    percentage: Some(96)
                },
                Language {
                    identifier: "de".into(),
                    percentage: Some(100)
                },
            ]
        );
    }

    #[test]
    fn driver_component() {
        let c: Component = Component::from_path("./tests/driver.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::Driver);
        assert_eq!(
            c.provides,
            vec![Provide::Modalias(
                "pci:v000010DEd*sv*sd*bc03sc00i00*".into()
            )]
        );
    }

    #[test]
    fn firmware_component() {
        let c = Component::from_path("./tests/firmware.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::Firmware);
        assert_eq!(
            c.developer_name,
            Some(TranslatableString::with_default("Hughski Limited"))
        );
        assert_eq!(
            c.provides,
            vec![
                // Todo: add type="flashed" into account
                Provide::Firmware("84f40464-9272-4ef7-9399-cd95f12da696".into())
            ]
        )
    }

    #[test]
    fn input_method_component() {
        let c = Component::from_path("./tests/input-method.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::InputMethod);
        assert_eq!(c.metadata_license, Some("FSFAP".into()));
        assert_eq!(c.name, TranslatableString::with_default("Mathwriter"));
        assert_eq!(
            c.summary,
            Some(TranslatableString::with_default(
                "Math symbols input method"
            ))
        );
        assert_eq!(
            c.urls,
            vec![ProjectUrl::Homepage(
                Url::from_str("https://github.com/mike-fabian/ibus-table-others").unwrap()
            )]
        );
    }

    #[test]
    fn codec_component() {
        let c = Component::from_path("./tests/codec.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::Codec);
    }

    #[test]
    fn icon_theme_component() {
        let c = Component::from_path("./tests/icon-theme.xml".into()).unwrap();
        assert_eq!(c._type, ComponentType::IconTheme);
        assert_eq!(c.metadata_license, Some("FSFAP".into()));
        assert_eq!(c.project_license, Some("GPL-3.0".into()));
        assert_eq!(c.name, TranslatableString::with_default("Papirus"));
        assert_eq!(
            c.summary,
            Some(TranslatableString::with_default(
                "A free and open source icon theme for Linux, based on the Paper Icon Set"
            ))
        );
    }

    #[test]
    fn addon_component() {
        let c = Component::from_path("./tests/addon.xml".into()).unwrap();

        assert_eq!(c._type, ComponentType::Addon);
        assert_eq!(c.name, TranslatableString::with_default("Code Assistance"));
        assert_eq!(c.update_contact, Some("developer_AT_example.com".into()));
        assert_eq!(
            c.summary,
            Some(TranslatableString::with_default(
                "Code assistance for C, C++ and Objective-C"
            ))
        );
        assert_eq!(
            c.urls,
            vec![ProjectUrl::Homepage(
                Url::from_str("http://projects.gnome.org/gedit").unwrap()
            )]
        );
        assert_eq!(c.metadata_license, Some("FSFAP".into()));
        assert_eq!(c.project_license, Some("GPL-3.0+".into()));
        assert_eq!(c.extends, vec![AppId::try_from("org.gnome.gedit").unwrap()]);
    }

    #[test]
    fn font_component() {
        let c = Component::from_path("./tests/font.xml".into()).unwrap();

        assert_eq!(c.metadata_license, Some("MIT".into()));
        assert_eq!(c.project_license, Some("OFL-1.1".into()));
        assert_eq!(c.name, TranslatableString::with_default("Lato"));
        assert_eq!(
            c.summary,
            Some(TranslatableString::with_default(
                "A sanserif type­face fam­ily"
            ))
        );
        assert_eq!(
            c.provides,
            vec![
                Provide::Font("Lato Regular".into()),
                Provide::Font("Lato Italic".into()),
                Provide::Font("Lato Bold".into()),
                Provide::Font("Lato Light".into()),
                Provide::Font("Lato Light Italic".into()),
            ]
        );
    }

    #[test]
    fn generic_component() {
        let c = Component::from_path("./tests/generic.xml".into()).unwrap();
        assert_eq!(
            c.urls.first().unwrap(),
            &ProjectUrl::Homepage(Url::from_str("http://www.example.org").unwrap())
        );
        assert_eq!(c.metadata_license, Some("CC0-1.0".into()));
        assert_eq!(c._type, ComponentType::Generic);
        assert_eq!(c.name, TranslatableString::with_default("Foo Bar"));
        assert_eq!(
            c.summary,
            Some(TranslatableString::with_default("A foo-ish bar"))
        );
        assert_eq!(
            c.developer_name,
            Some(TranslatableString::with_default("FooBar Team"))
        );
        assert_eq!(
            c.provides,
            vec![
                Provide::Library("libfoobar.so.2".into()),
                Provide::Font("foo.ttf".into()),
                Provide::Binary("foobar".into())
            ]
        );
        assert_eq!(
            c.releases,
            vec![Release {
                version: "1.2".into(),
                date_eol: None,
                _type: ReleaseType::default(),
                date: Some(Utc.ymd(2015, 2, 16).and_hms_milli(0, 0, 0, 0)),
                sizes: vec![]
            }]
        );
    }
}
