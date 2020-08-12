use super::de::*;
use super::enums::{
    Bundle, Category, ComponentKind, Icon, Kudo, Launchable, ProjectUrl, Provide, Translation,
};
use super::translatable_string::{TranslatableString, TranslatableVec};
use super::{AppId, ContentRating, Language, License, Release, Screenshot};
use anyhow::Result;
#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
#[cfg(feature = "gzip")]
use std::fs::File;
#[cfg(feature = "gzip")]
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Component {
    #[serde(rename = "type", default)]
    pub kind: ComponentKind,
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
    pub keywords: Option<TranslatableVec>,
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

    #[cfg(feature = "gzip")]
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
    use crate::enums::{ComponentKind, FirmwareKind, Launchable, ProjectUrl, Provide};
    use crate::translatable_string::TranslatableString;
    use crate::{
        AppId, ComponentBuilder, Image, LanguageBuilder, ReleaseBuilder, ReleaseKind,
        ScreenshotBuilder,
    };
    use chrono::{TimeZone, Utc};
    use std::convert::TryFrom;
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn addon_component() {
        let c1 = Component::from_path("./tests/addon.xml".into()).unwrap();

        let id = AppId::try_from("org.gnome.gedit_code_assistance").unwrap();
        let name = TranslatableString::with_default("Code Assistance");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::Addon)
            .metadata_license("FSFAP".into())
            .project_license("GPL-3.0+".into())
            .summary(TranslatableString::with_default(
                "Code assistance for C, C++ and Objective-C",
            ))
            .update_contact("developer_AT_example.com")
            .url(ProjectUrl::Homepage(
                Url::from_str("http://projects.gnome.org/gedit").unwrap(),
            ))
            .extend(AppId::try_from("org.gnome.gedit").unwrap())
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn codec_component() {
        let c1 = Component::from_path("./tests/codec.xml".into()).unwrap();

        let id = AppId::try_from("org.freedesktop.gstreamer.codecs-good").unwrap();
        let name = TranslatableString::with_default("GStreamer Multimedia Codecs - Extra");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::Codec)
            .metadata_license("CC0".into())
            .provide(Provide::Codec("encoder-audio/mpeg".into()))
            .provide(Provide::Codec("mpegversion=(int){ 4, 2 }".into()))
            .provide(Provide::Codec("stream-format=(string){ adts, raw }".into()))
            .provide(Provide::Codec("encoder-video/mpeg".into()))
            .provide(Provide::Codec("systemstream=(boolean)false".into()))
            .provide(Provide::Codec("mpegversion=(int){ 1, 2, 4 }".into()))
            .provide(Provide::Codec("systemstream=(boolean)true".into()))
            .provide(Provide::Codec("encoder-video/x-xvid".into()))
            .provide(Provide::Codec("element-faac".into()))
            .provide(Provide::Codec("element-mpeg2enc".into()))
            .provide(Provide::Codec("element-mplex".into()))
            .provide(Provide::Codec("element-xviddec".into()))
            .provide(Provide::Codec("element-xvidenc".into()))
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn desktop_application_component() {
        let c1: Component = Component::from_path("./tests/desktop.xml".into()).unwrap();

        let id = AppId::try_from("org.gnome.gnome-power-statistics").unwrap();
        let name = TranslatableString::with_default("Power Statistics");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::DesktopApplication)
            .summary(TranslatableString::with_default("Observe power management"))
            .metadata_license("FSFAP".into())
            .project_license("GPL-2.0+".into())
            .project_group("GNOME")
            .launchable(Launchable::DesktopId(
                "org.gnome.gnome-power-statistics.desktop".to_string(),
            ))
            .url(ProjectUrl::Homepage(
                Url::from_str("http://www.gnome.org/projects/en_US/gnome-power-manager").unwrap(),
            ))
            .provide(Provide::Binary("gnome-power-statistics".into()))
            .provide(Provide::Id("gnome-power-statistics.desktop".into()))
            .screenshot(
                ScreenshotBuilder::new()
                    .caption(TranslatableString::with_default("The options dialog"))
                    .image(Image::Source {
                        url: Url::from_str("http://www.hughsie.com/en_US/main.png").unwrap(),
                        width: None,
                        height: None,
                    })
                    .build(),
            )
            .screenshot(
                ScreenshotBuilder::new()
                    .set_default(false)
                    .image(Image::Source {
                        url: Url::from_str("http://www.hughsie.com/en_US/preferences.png").unwrap(),
                        width: None,
                        height: None,
                    })
                    .build(),
            )
            .release(
                ReleaseBuilder::new("3.12.2")
                    .date(Utc.ymd(2013, 4, 12).and_hms_milli(0, 0, 0, 0))
                    .build(),
            )
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn driver_component() {
        let c1: Component = Component::from_path("./tests/driver.xml".into()).unwrap();

        let id = AppId::try_from("com.nvidia.GeForce").unwrap();
        let name = TranslatableString::with_default("NVIDIA GeForce");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::Driver)
            .metadata_license("CC0-1.0".into())
            .project_license("LicenseRef-proprietary:NVIDIA".into())
            .summary(TranslatableString::with_default("NVIDIA Graphics Driver"))
            .developer_name(TranslatableString::with_default("NVIDIA Corporation"))
            .provide(Provide::Modalias(
                "pci:v000010DEd*sv*sd*bc03sc00i00*".into(),
            ))
            .url(ProjectUrl::Homepage(
                Url::from_str("http://www.nvidia.com/Download/index.aspx").unwrap(),
            ))
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn firmware_component() {
        let c1 = Component::from_path("./tests/firmware.xml".into()).unwrap();

        let id = AppId::try_from("com.hughski.ColorHug2.firmware").unwrap();
        let name = TranslatableString::with_default("ColorHugALS Firmware");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::Firmware)
            .summary(TranslatableString::with_default(
                "Firmware for the ColorHugALS Ambient Light Sensor",
            ))
            .url(ProjectUrl::Homepage(
                Url::from_str("http://www.hughski.com/").unwrap(),
            ))
            .metadata_license("CC0-1.0".into())
            .project_license("GPL-2.0+".into())
            .developer_name(TranslatableString::with_default("Hughski Limited"))
            .provide(Provide::Firmware {
                kind: FirmwareKind::Flashed,
                item: "84f40464-9272-4ef7-9399-cd95f12da696".into(),
            })
            .release(
                ReleaseBuilder::new("3.0.2")
                    .date(Utc.ymd(2015, 2, 16).and_hms_milli(0, 0, 0, 0))
                    .build(),
            )
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn font_component() {
        let c1 = Component::from_path("./tests/font.xml".into()).unwrap();

        let id = AppId::try_from("com.latofonts.Lato").unwrap();
        let name = TranslatableString::with_default("Lato");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::Font)
            .metadata_license("MIT".into())
            .project_license("OFL-1.1".into())
            .summary(TranslatableString::with_default(
                "A sanserif type­face fam­ily",
            ))
            .provide(Provide::Font("Lato Regular".into()))
            .provide(Provide::Font("Lato Italic".into()))
            .provide(Provide::Font("Lato Bold".into()))
            .provide(Provide::Font("Lato Light".into()))
            .provide(Provide::Font("Lato Light Italic".into()))
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn generic_component() {
        let c1 = Component::from_path("./tests/generic.xml".into()).unwrap();

        let id = AppId::try_from("com.example.foobar").unwrap();
        let name = TranslatableString::with_default("Foo Bar");

        let c2 = ComponentBuilder::new(id, name)
            .metadata_license("CC0-1.0".into())
            .summary(TranslatableString::with_default("A foo-ish bar"))
            .url(ProjectUrl::Homepage(
                Url::from_str("http://www.example.org").unwrap(),
            ))
            .developer_name(TranslatableString::with_default("FooBar Team"))
            .provide(Provide::Library("libfoobar.so.2".into()))
            .provide(Provide::Font("foo.ttf".into()))
            .provide(Provide::Binary("foobar".into()))
            .release(
                ReleaseBuilder::new("1.2")
                    .date(Utc.ymd(2015, 2, 16).and_hms_milli(0, 0, 0, 0))
                    .build(),
            )
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn icon_theme_component() {
        let c1 = Component::from_path("./tests/icon-theme.xml".into()).unwrap();
        let id = AppId::try_from("io.git.PapirusIconTheme").unwrap();
        let name = TranslatableString::with_default("Papirus");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::IconTheme)
            .metadata_license("FSFAP".into())
            .project_license("GPL-3.0".into())
            .summary(TranslatableString::with_default("A free and open source icon theme for Linux, based on the Paper Icon Set"))
            .screenshot(ScreenshotBuilder::new().image(Image::Source {
                width: None,
                height:None,
                url: Url::from_str("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png").unwrap()
            }).build())
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn input_method_component() {
        let c1 = Component::from_path("./tests/input-method.xml".into()).unwrap();

        let id = AppId::try_from("com.github.ibus.mathwriter-ibus.db").unwrap();
        let name = TranslatableString::with_default("Mathwriter");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::InputMethod)
            .metadata_license("FSFAP".into())
            .summary(TranslatableString::with_default(
                "Math symbols input method",
            ))
            .url(ProjectUrl::Homepage(
                Url::from_str("https://github.com/mike-fabian/ibus-table-others").unwrap(),
            ))
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn localization_component() {
        let c1 = Component::from_path("./tests/localization.xml".into()).unwrap();

        let id = AppId::try_from("org.kde.l10n.de").unwrap();
        let name = TranslatableString::with_default("KDE German Language");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::Localization)
            .metadata_license("FSFAP".into())
            .summary(TranslatableString::with_default(
                "German localization for the KDE desktop and apps",
            ))
            .developer_name(TranslatableString::with_default("The KDE German L10N team"))
            .extend(AppId::try_from("org.kde.plasmashell").unwrap())
            .extend(AppId::try_from("org.kde.gwenview.desktop").unwrap())
            .extend(AppId::try_from("org.kde.dolphin.desktop").unwrap())
            .url(ProjectUrl::Homepage(
                Url::from_str("http://i18n.kde.org/team-infos.php?teamcode=de").unwrap(),
            ))
            .language(LanguageBuilder::new("de_DE").build())
            .language(LanguageBuilder::new("de_AT").percentage(96).build())
            .language(LanguageBuilder::new("de").percentage(100).build())
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn os_component() {
        let c1: Component = Component::from_path("./tests/os.xml".into()).unwrap();

        let id = AppId::try_from("org.debian.debian").unwrap();
        let name = TranslatableString::with_default("Debian GNU/Linux");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::OS)
            .summary(TranslatableString::with_default(
                "The universal operating system",
            ))
            .url(ProjectUrl::Homepage(
                Url::from_str("https://www.debian.org/").unwrap(),
            ))
            .metadata_license("FSFAP".into())
            .developer_name(TranslatableString::with_default("The Debian Project"))
            .release(
                ReleaseBuilder::new("10.0")
                    .kind(ReleaseKind::Development)
                    .build(),
            )
            .release(
                ReleaseBuilder::new("9.0")
                    .date(Utc.ymd(2017, 7, 17).and_hms_milli(0, 0, 0, 0))
                    .date_eol(Utc.ymd(2020, 7, 17).and_hms_milli(0, 0, 0, 0))
                    .build(),
            )
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn runtime_component() {
        let c1: Component = Component::from_path("./tests/runtime.xml".into()).unwrap();

        let id = AppId::try_from("org.freedesktop.Platform").unwrap();
        let name = TranslatableString::with_default("Freedesktop Platform");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::Runtime)
            .metadata_license("FSFAP".into())
            .project_license("LicenseRef-free=https://freedesktop-sdk.gitlab.io/".into())
            .summary(TranslatableString::with_default(
                "Basic libraries to run Linux desktop applications",
            ))
            .url(ProjectUrl::Homepage(
                Url::from_str("https://freedesktop-sdk.gitlab.io/").unwrap(),
            ))
            .release(ReleaseBuilder::new("10.0").build())
            .release(
                ReleaseBuilder::new("9.0")
                    .date(Utc.ymd(2020, 01, 12).and_hms_milli(0, 0, 0, 0))
                    .build(),
            )
            .build();

        assert_eq!(c1, c2);
    }
}
