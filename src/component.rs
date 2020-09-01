use super::enums::{
    Bundle, Category, ComponentKind, Icon, Kudo, Launchable, ProjectUrl, Provide, Translation,
};
use super::types::{
    AppId, ContentRating, Language, License, MarkupTranslatableString, Release, Screenshot,
    TranslatableList, TranslatableString,
};
use anyhow::Result;
#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
#[cfg(feature = "gzip")]
use std::io::prelude::*;
use std::path::PathBuf;

use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use xmltree::Element;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Component {
    #[serde(default, rename = "type")]
    pub kind: ComponentKind,
    pub id: AppId,
    pub name: TranslatableString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<TranslatableString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<MarkupTranslatableString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_license: Option<License>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_license: Option<License>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_group: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compulsory_for_desktop: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extends: Vec<AppId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: Vec<Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub screenshots: Vec<Screenshot>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<ProjectUrl>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub developer_name: Option<TranslatableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_contact: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Category>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub launchables: Vec<Launchable>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkgname: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_pkgname: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<Bundle>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub releases: Vec<Release>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<Language>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mimetypes: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kudos: Vec<Kudo>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<TranslatableList>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_rating: Option<ContentRating>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provides: Vec<Provide>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translation>,
}

impl Component {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let file = BufReader::new(File::open(path)?);
        let component = Component::try_from(&Element::parse(file)?)?;
        Ok(component)
    }

    #[cfg(feature = "gzip")]
    pub fn from_gzipped(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;

        let mut d = GzDecoder::new(f);
        let mut xml = String::new();
        d.read_to_string(&mut xml)?;

        let component: Component = Component::try_from(&Element::parse(xml.as_bytes())?)?;
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
    use crate::types::{
        ContentRating, MarkupTranslatableString, TranslatableList, TranslatableString,
    };
    use chrono::{TimeZone, Utc};
    use url::Url;

    #[test]
    fn addon_component() {
        let c1 = Component::from_path("./tests/addon.xml".into()).unwrap();

        let c2 = ComponentBuilder::default()
            .id("org.gnome.gedit_code_assistance".into())
            .name(TranslatableString::with_default("Code Assistance"))
            .kind(ComponentKind::Addon)
            .metadata_license("FSFAP".into())
            .project_license("GPL-3.0+".into())
            .summary(TranslatableString::with_default(
                "Code assistance for C, C++ and Objective-C",
            ))
            .update_contact("developer_AT_example.com")
            .url(ProjectUrl::Homepage(
                Url::parse("http://projects.gnome.org/gedit").unwrap(),
            ))
            .extend("org.gnome.gedit".into())
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn codec_component() {
        let c1 = Component::from_path("./tests/codec.xml".into()).unwrap();

        let c2 = ComponentBuilder::default()
            .id("org.freedesktop.gstreamer.codecs-good".into())
            .name(TranslatableString::with_default(
                "GStreamer Multimedia Codecs - Extra",
            ))
            .kind(ComponentKind::Codec)
            .description(MarkupTranslatableString::with_default(
                "<p>\n      This addon includes several additional codecs that are missing\n      something - perhaps a good code review, some documentation, a set of\n      tests, a real live maintainer, or some actual wide use.\n      However, they might be good enough to play your media files.\n    </p><p>\n      These codecs can be used to encode and decode media files where the\n      format is not patent encumbered.\n    </p><p>\n      A codec decodes audio and video for for playback or editing and is also\n      used for transmission or storage.\n      Different codecs are used in video-conferencing, streaming media and\n      video editing applications.\n    </p>"
            ))
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

        let c2 = ComponentBuilder::default()
            .id("org.gnome.gnome-power-statistics".into())
            .name(TranslatableString::with_default("Power Statistics"))
            .kind(ComponentKind::DesktopApplication)
            .summary(TranslatableString::with_default("Observe power management"))
            .description(MarkupTranslatableString::with_default(
                "<p>\n      Power Statistics is a program used to view historical and current battery\n      information and will show programs running on your computer using power.\n        </p><p>Example list:</p><ul><li>First item</li><li>Second item</li></ul><p>\n      You probably only need to install this application if you are having problems\n      with your laptop battery, or are trying to work out what programs are using\n      significant amounts of power.\n        </p>"
            ))
            .metadata_license("FSFAP".into())
            .project_license("GPL-2.0+".into())
            .project_group("GNOME")
            .launchable(Launchable::DesktopId(
                "org.gnome.gnome-power-statistics.desktop".to_string(),
            ))
            .url(ProjectUrl::Homepage(
                Url::parse("http://www.gnome.org/projects/en_US/gnome-power-manager").unwrap(),
            ))
            .provide(Provide::Binary("gnome-power-statistics".into()))
            .provide(Provide::Id("gnome-power-statistics.desktop".into()))
            .screenshot(
                ScreenshotBuilder::default()
                    .caption(TranslatableString::with_default("The options dialog"))
                    .image(
                        ImageBuilder::new(
                            Url::parse("http://www.hughsie.com/en_US/main.png").unwrap(),
                        )
                        .build(),
                    )
                    .build(),
            )
            .screenshot(
                ScreenshotBuilder::default()
                    .set_default(false)
                    .image(
                        ImageBuilder::new(
                            Url::parse("http://www.hughsie.com/en_US/preferences.png").unwrap(),
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

        let c2 = ComponentBuilder::default()
            .id("com.nvidia.GeForce".into())
            .name(TranslatableString::with_default("NVIDIA GeForce"))
            .kind(ComponentKind::Driver)
            .metadata_license("CC0-1.0".into())
            .project_license("LicenseRef-proprietary:NVIDIA".into())
            .summary(TranslatableString::with_default("NVIDIA Graphics Driver"))
            .description(
                MarkupTranslatableString::with_default(
                    "<p>\n      The NVIDIA Accelerated Linux Graphics Driver brings accelerated 2D\n      functionality and high-performance OpenGL support to Linux x86 with the\n      use of NVIDIA graphics processing units.\n    </p>"
                )
            )
            .developer_name(TranslatableString::with_default("NVIDIA Corporation"))
            .provide(Provide::Modalias(
                "pci:v000010DEd*sv*sd*bc03sc00i00*".into(),
            ))
            .url(ProjectUrl::Homepage(
                Url::parse("http://www.nvidia.com/Download/index.aspx").unwrap(),
            ))
            .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn firmware_component() {
        let c1 = Component::from_path("./tests/firmware.xml".into()).unwrap();

        let c2 = ComponentBuilder::default()
            .id("com.hughski.ColorHug2.firmware".into())
            .name(TranslatableString::with_default("ColorHugALS Firmware"))
            .kind(ComponentKind::Firmware)
            .summary(TranslatableString::with_default(
                "Firmware for the ColorHugALS Ambient Light Sensor",
            ))
            .description(
                MarkupTranslatableString::with_default(
                    "<p>\n      Updating the firmware on your ColorHugALS device improves performance and\n      adds new features.\n    </p>"
                )
            )
            .url(ProjectUrl::Homepage(
                Url::parse("http://www.hughski.com/").unwrap(),
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
                        ArtifactBuilder::default()
                        .url(Url::parse("http://www.hughski.com/downloads/colorhug-als/firmware/colorhug-als-3.0.2.cab").unwrap()) 
                        .kind(ArtifactKind::Binary)
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

        let c2 = ComponentBuilder::default()
            .id("com.latofonts.Lato".into())
            .name(TranslatableString::with_default("Lato"))
            .kind(ComponentKind::Font)
            .metadata_license("MIT".into())
            .project_license("OFL-1.1".into())
            .summary(TranslatableString::with_default(
                "A sanserif type­face fam­ily",
            ))
            .description(
                MarkupTranslatableString::with_default(
                    "<p>\n      Lato is a sanserif type\u{ad}face fam\u{ad}ily designed in the Sum\u{ad}mer 2010 by Warsaw-\u{200b}\u{200b}based designer\n      Łukasz Dziedzic (“Lato” means “Sum\u{ad}mer” in Pol\u{ad}ish). In Decem\u{ad}ber 2010 the Lato fam\u{ad}ily\n      was pub\u{ad}lished under the open-\u{200b}\u{200b}source Open Font License by his foundry tyPoland, with\n      sup\u{ad}port from Google.\n    </p>"
                )
            )
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

        let c2 = ComponentBuilder::default()
            .id("com.example.foobar".into())
            .name(TranslatableString::with_default("Foo Bar"))
            .metadata_license("CC0-1.0".into())
            .summary(TranslatableString::with_default("A foo-ish bar"))
            .url(ProjectUrl::Homepage(
                Url::parse("http://www.example.org").unwrap(),
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

        let c2 = ComponentBuilder::default()
            .id("io.git.PapirusIconTheme".into())
            .name(TranslatableString::with_default("Papirus"))
            .kind(ComponentKind::IconTheme)
            .metadata_license("FSFAP".into())
            .project_license("GPL-3.0".into())
            .description(
                MarkupTranslatableString::with_default("<p>\n      Papirus is a free and open source SVG icon theme for Linux, based on Paper Icon Set\n      with a lot of new icons and a few extras, like Hardcode-Tray support, KDE colorscheme\n      support, Folder Color support, and others.\n      It is available in four variants:\n    </p><ul><li>Papirus</li><li>Papirus Dark</li><li>Papirus Light</li><li>ePapirus (for elementary OS and Pantheon Desktop)</li></ul>")
            )
            .summary(TranslatableString::with_default("A free and open source icon theme for Linux, based on the Paper Icon Set"))
            .screenshot(
                ScreenshotBuilder::default()
                .image(
                    ImageBuilder::new(
                        Url::parse("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png").unwrap()
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

        let c2 = ComponentBuilder::default()
            .id("com.github.ibus.mathwriter-ibus.db".into())
            .name(TranslatableString::with_default("Mathwriter"))
            .kind(ComponentKind::InputMethod)
            .metadata_license("FSFAP".into())
            .summary(TranslatableString::with_default(
                "Math symbols input method",
            ))
            .description(MarkupTranslatableString::with_default(
                "<p>\n      The input method is designed for entering mathematical symbols.\n    </p><p>\n      Input methods are typing systems allowing users to input complex languages.\n      They are necessary because these contain too many characters to simply be laid\n      out on a traditional keyboard.\n    </p>"
            ))
            .url(ProjectUrl::Homepage(
                Url::parse("https://github.com/mike-fabian/ibus-table-others").unwrap(),
            ))
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn localization_component() {
        let c1 = Component::from_path("./tests/localization.xml".into()).unwrap();

        let c2 = ComponentBuilder::default()
            .id("org.kde.l10n.de".into())
            .name(TranslatableString::with_default("KDE German Language"))
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
                Url::parse("http://i18n.kde.org/team-infos.php?teamcode=de").unwrap(),
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

        let description = "<p>\n      Debian is a free operating system (OS) for your computer.\n      An operating system is the set of basic programs and utilities that make your computer run.\n        </p>";
        let c2 = ComponentBuilder::default()
            .id("org.debian.debian".into())
            .name(TranslatableString::with_default("Debian GNU/Linux"))
            .description(MarkupTranslatableString::with_default(description))
            .kind(ComponentKind::OS)
            .summary(TranslatableString::with_default(
                "The universal operating system",
            ))
            .url(ProjectUrl::Homepage(
                Url::parse("https://www.debian.org/").unwrap(),
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

        let c2 = ComponentBuilder::default()
            .id("org.freedesktop.Platform".into())
            .name(TranslatableString::with_default("Freedesktop Platform"))
            .kind(ComponentKind::Runtime)
            .metadata_license("FSFAP".into())
            .project_license("LicenseRef-free=https://freedesktop-sdk.gitlab.io/".into())
            .summary(TranslatableString::with_default(
                "Basic libraries to run Linux desktop applications",
            ))
            .description(
                MarkupTranslatableString::with_default("<p>\n      The Freedesktop Platform is a runtime that contains the most basic libraries\n      and files needed to run a Linux desktop application.\n        </p>")
            )
            .url(ProjectUrl::Homepage(
                Url::parse("https://freedesktop-sdk.gitlab.io/").unwrap(),
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
            Component::from_path("./tests/app-org.gnome.design.Contrast.xml".into()).unwrap();

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
        let keywords = TranslatableList::with_default(vec!["Color", "Contrast", "GNOME", "GTK"])
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
        let description = MarkupTranslatableString::with_default("<p>Contrast checks whether the contrast between two colors meet the WCAG requirements.</p>")
            .and_locale("cs", "<p>Kontroluje kontrast mezi dvěma zadanými barvami, jestli vyhovuje požadavkům pravidel pro bezbariérové weby (WCAG).</p>")
            .and_locale("es", "<p>Contraste comprueba la diferencia de contraste entre dos colores que cumplen los requisitos WCAG.</p>")
            .and_locale("eu", "<p>Kontrastea aplikazioak bi koloreren arteko kontrasteak WCAG eskakizunak betetzen dituen ala ez egiaztatzen du.</p>")
            .and_locale("fur", "<p>Contrast al controle se il contrast tra doi colôrs al sodisfe i recuisîts WCAG.</p>")
            .and_locale("hu", "<p>A Kontraszt azt ellenőrzi, hogy a két szín közti kontraszt megfelel-e a WCAG követelményeinek.</p>")
            .and_locale("id", "<p>Kontras memeriksa apakah kontras antara dua warna memenuhi persyaratan WCAG.</p>")
            .and_locale("pl", "<p>Sprawdzanie, czy kontrast między dwoma kolorami spełnia wymagania WCAG.</p>")
            .and_locale("pt_BR", "<p>Contraste verifica se o contraste entre duas cores atende os requisitos WCAG.</p>")
            .and_locale("sv", "<p>Kontrast kontrollerar om kontrasten mellan två färger uppfyller WCAG-kraven.</p>")
            .and_locale("tr", "<p>Contrast, iki renk arasındaki karşıtlığın WCAG gereksinimlerini karşılayıp karşılamadığını gözden geçirir.</p>");

        let c2 = ComponentBuilder::default()
            .id("org.gnome.design.Contrast".into())
            .name(name)
            .kind(ComponentKind::DesktopApplication)
            .summary(summary)
            .description(description)
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
                reference: "app/org.gnome.design.Contrast/x86_64/stable".into()
            })
            .url(ProjectUrl::BugTracker(Url::parse("https://gitlab.gnome.org/World/design/contrast/issues").unwrap()))
            .url(ProjectUrl::Donation(Url::parse("https://liberapay.com/bielmoussaoui").unwrap()))
            .url(ProjectUrl::Homepage(Url::parse("https://gitlab.gnome.org/World/design/contrast").unwrap()))
            .url(ProjectUrl::Translate(Url::parse("https://l10n.gnome.org/module/contrast/").unwrap()))
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
            .screenshot(ScreenshotBuilder::default()
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://gitlab.gnome.org/World/design/contrast/raw/master/data/resources/screenshots/screenshot1.png").unwrap()
                        ).build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/624x351/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(624)
                        .height(351)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/112x63/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(112)
                        .height(63)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/224x126/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(224)
                        .height(126)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/752x423/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png").unwrap()
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
