use super::de::*;
use super::enums::{
    Bundle, Category, ComponentKind, Icon, Kudo, Launchable, ProjectUrl, Provide, Translation,
};
use super::types::{
    AppId, ContentRating, Language, License, Release, Screenshot, TranslatableString,
    TranslatableVec,
};
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
    #[serde(default)]
    pub source_pkgname: Option<String>,
    #[serde(rename = "bundle", deserialize_with = "bundle_deserialize", default)]
    pub bundles: Vec<Bundle>,
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
    #[serde(
        default,
        rename = "translation",
        deserialize_with = "translation_deserialize"
    )]
    pub translations: Vec<Translation>,
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
    use crate::builders::{
        ArtifactBuilder, ComponentBuilder, ImageBuilder, LanguageBuilder, ReleaseBuilder,
        ScreenshotBuilder,
    };
    use crate::enums::{
        ArtifactKind, Bundle, Category, ComponentKind, ContentRatingVersion, FirmwareKind, Icon,
        ImageKind, Kudo, Launchable, ProjectUrl, Provide, ReleaseKind, Translation,
    };
    use crate::types::{ContentRating, TranslatableString, TranslatableVec};
    use chrono::{TimeZone, Utc};
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn addon_component() {
        let c1 = Component::from_path("./tests/addon.xml".into()).unwrap();

        let id = "org.gnome.gedit_code_assistance".into();
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
            .extend("org.gnome.gedit".into())
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn codec_component() {
        let c1 = Component::from_path("./tests/codec.xml".into()).unwrap();

        let id = "org.freedesktop.gstreamer.codecs-good".into();
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

        let id = "org.gnome.gnome-power-statistics".into();
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
                    .image(
                        ImageBuilder::new(
                            Url::from_str("http://www.hughsie.com/en_US/main.png").unwrap(),
                        )
                        .build(),
                    )
                    .build(),
            )
            .screenshot(
                ScreenshotBuilder::new()
                    .set_default(false)
                    .image(
                        ImageBuilder::new(
                            Url::from_str("http://www.hughsie.com/en_US/preferences.png").unwrap(),
                        )
                        .build(),
                    )
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

        let id = "com.nvidia.GeForce".into();
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

        let id = "com.hughski.ColorHug2.firmware".into();
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
                    .artifact(
                        ArtifactBuilder::new(
                            Url::from_str("http://www.hughski.com/downloads/colorhug-als/firmware/colorhug-als-3.0.2.cab").unwrap(), 
                            ArtifactKind::Binary
                        )
                        .build()
                    )
                    .build(),
            )
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn font_component() {
        let c1 = Component::from_path("./tests/font.xml".into()).unwrap();

        let id = "com.latofonts.Lato".into();
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

        let id = "com.example.foobar".into();
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
        let id = "io.git.PapirusIconTheme".into();
        let name = TranslatableString::with_default("Papirus");

        let c2 = ComponentBuilder::new(id, name)
            .kind(ComponentKind::IconTheme)
            .metadata_license("FSFAP".into())
            .project_license("GPL-3.0".into())
            .summary(TranslatableString::with_default("A free and open source icon theme for Linux, based on the Paper Icon Set"))
            .screenshot(
                ScreenshotBuilder::new()
                .image(
                    ImageBuilder::new(
                        Url::from_str("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png").unwrap()
                    )
                    .build()
                )
                .build()
            )
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn input_method_component() {
        let c1 = Component::from_path("./tests/input-method.xml".into()).unwrap();

        let id = "com.github.ibus.mathwriter-ibus.db".into();
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

        let name = TranslatableString::with_default("KDE German Language");

        let c2 = ComponentBuilder::new("org.kde.l10n.de".into(), name)
            .kind(ComponentKind::Localization)
            .metadata_license("FSFAP".into())
            .summary(TranslatableString::with_default(
                "German localization for the KDE desktop and apps",
            ))
            .developer_name(TranslatableString::with_default("The KDE German L10N team"))
            .extend("org.kde.plasmashell".into())
            .extend("org.kde.gwenview.desktop".into())
            .extend("org.kde.dolphin.desktop".into())
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

        let name = TranslatableString::with_default("Debian GNU/Linux");

        let c2 = ComponentBuilder::new("org.debian.debian".into(), name)
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

        let name = TranslatableString::with_default("Freedesktop Platform");

        let c2 = ComponentBuilder::new("org.freedesktop.Platform".into(), name)
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

    #[test]
    fn contrast_metainfo_component() {
        let c1: Component =
            Component::from_path("./tests/org.gnome.design.Contrast.xml".into()).unwrap();

        let name = TranslatableString::with_default("Contrast")
            .and_locale("cs", "Kontrast")
            .and_locale("cs", "Kontrast")
            .and_locale("da", "Kontrast")
            .and_locale("de", "Kontrast")
            .and_locale("es", "Contraste")
            .and_locale("eu", "Kontrastea")
            .and_locale("fi", "Kontrasti")
            .and_locale("hu", "Kontraszt")
            .and_locale("id", "Kontras")
            .and_locale("pl", "Kontrast")
            .and_locale("pt_BR", "Contraste")
            .and_locale("sv", "Kontrast")
            .and_locale("tr", "Kontrast");
        let summary = TranslatableString::with_default("Check contrast between two colors")
            .and_locale("cs", "Zkontrolujte kontrast mezi dvěma barvami")
            .and_locale("da", "Undersøg kontrasten mellem to farver")
            .and_locale("de", "Kontrast zwischen zwei Farben vergleichen")
            .and_locale("en_GB", "Check contrast between two colours")
            .and_locale("es", "Comprobar el contraste entre dos colores")
            .and_locale("eu", "Aztertu bi koloreren arteko kontrastea")
            .and_locale("fi", "Tarkista kahden värin välinen kontrasti")
            .and_locale("fur", "Controle il contrast tra doi colôrs")
            .and_locale("hu", "Kontraszt ellenőrzése két szín között")
            .and_locale("id", "Periksa kontras antara dua warna")
            .and_locale("pl", "Sprawdzanie kontrastu między dwoma kolorami")
            .and_locale("pt_BR", "Verifique o contraste entre duas cores")
            .and_locale("sv", "Kontrollera kontrast mellan två färger")
            .and_locale("tr", "İki renk arasındaki karşıtlığı gözden geçir");
        let keywords = TranslatableVec::with_default(vec!["Color", "Contrast", "GNOME", "GTK"])
            .and_locale("cs", vec!["barva", "kontrast"])
            .and_locale("da", vec!["Farve", "Kontrast"])
            .and_locale("de", vec!["Farbe", "Farben", "GTK+", "Kontrast"])
            .and_locale("en_GB", vec!["Colour"])
            .and_locale("es", vec!["Contraste"])
            .and_locale("eu", vec!["Kolorea", "Kontrastea"])
            .and_locale("fi", vec!["kontrasti", "väri"])
            .and_locale("fur", vec!["Colôr"])
            .and_locale("hu", vec!["kontraszt", "szín"])
            .and_locale("id", vec!["Kontras", "Warna"])
            .and_locale("pl", vec!["Colour", "GTK+", "Kolor", "Kolory", "Kontrast"])
            .and_locale("pt_BR", vec!["Contraste", "cor"])
            .and_locale("sv", vec!["Färg", "Kontrast"])
            .and_locale("tr", vec!["Karşıtlık", "Kontrast", "Renk"]);

        let c2 = ComponentBuilder::new("org.gnome.design.Contrast".into(), name)
            .kind(ComponentKind::DesktopApplication)
            .summary(summary)
            .category(Category::Utility)
            .project_license("GPL-3.0+".into())
            .project_group("GNOME")
            .keywords(keywords)
            .kudo(Kudo::HiDpiIcon)
            .kudo(Kudo::HighContrast)
            .kudo(Kudo::ModernToolkit)
            .bundle(Bundle::Flatpak {
                runtime: Some("org.gnome.Platform/x86_64/3.36".into()),
                sdk: "org.gnome.Sdk/x86_64/3.36".into(),
                id: "app/org.gnome.design.Contrast/x86_64/stable".into()
            })
            .url(ProjectUrl::BugTracker(Url::from_str("https://gitlab.gnome.org/World/design/contrast/issues").unwrap()))
            .url(ProjectUrl::Donation(Url::from_str("https://liberapay.com/bielmoussaoui").unwrap()))
            .url(ProjectUrl::Homepage(Url::from_str("https://gitlab.gnome.org/World/design/contrast").unwrap()))
            .url(ProjectUrl::Translate(Url::from_str("https://l10n.gnome.org/module/contrast/").unwrap()))
            .translation(Translation::Gettext("contrast".into()))
            .launchable(Launchable::DesktopId("org.gnome.design.Contrast.desktop".into()))
            .developer_name(TranslatableString::with_default("Bilal Elmoussaoui"))
            .icon(Icon::Cached {
                path: "org.gnome.design.Contrast.png".into(),
                width: Some(64),
                height: Some(64),
            })
            .icon(Icon::Cached {
                path: "org.gnome.design.Contrast.png".into(),
                width: Some(128),
                height: Some(128),
            }).content_rating(ContentRating {
                attributes: vec![],
                version: ContentRatingVersion::Oars1_0
            })
            .release(ReleaseBuilder::new("0.0.3").date(chrono::Utc.datetime_from_str("1582329600", "%s").unwrap()).build())
            .release(ReleaseBuilder::new("0.0.2").date(chrono::Utc.datetime_from_str("1566691200", "%s").unwrap()).build())
            .release(ReleaseBuilder::new("0.0.1").date(chrono::Utc.datetime_from_str("1565136000", "%s").unwrap()).build())
            .language(LanguageBuilder::new("cs").percentage(100).build())
            .language(LanguageBuilder::new("da").percentage(93).build())
            .language(LanguageBuilder::new("de").percentage(93).build())
            .language(LanguageBuilder::new("en_GB").percentage(93).build())
            .language(LanguageBuilder::new("es").percentage(100).build())
            .language(LanguageBuilder::new("eu").percentage(100).build())
            .language(LanguageBuilder::new("fi").percentage(86).build())
            .language(LanguageBuilder::new("fur").percentage(100).build())
            .language(LanguageBuilder::new("hu").percentage(100).build())
            .language(LanguageBuilder::new("id").percentage(100).build())
            .language(LanguageBuilder::new("pl").percentage(100).build())
            .language(LanguageBuilder::new("pt_BR").percentage(100).build())
            .language(LanguageBuilder::new("sv").percentage(100).build())
            .language(LanguageBuilder::new("tr").percentage(100).build())
            .screenshot(ScreenshotBuilder::new()
                    .image(
                        ImageBuilder::new(
                            Url::from_str("https://gitlab.gnome.org/World/design/contrast/raw/master/data/resources/screenshots/screenshot1.png").unwrap()
                        ).build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::from_str("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/624x351/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(624)
                        .height(351)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::from_str("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/112x63/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(112)
                        .height(63)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::from_str("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/224x126/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(224)
                        .height(126)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::from_str("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/752x423/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(752)
                        .height(423)
                        .build()
                    ).build()
                )
            .build();

        assert_eq!(c1, c2);
    }
}
