use std::{collections::HashMap, convert::TryFrom, fs::File, io::BufReader, path::PathBuf};

#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use xmltree::Element;

use super::{
    enums::{
        Bundle, Category, ComponentKind, Icon, Kudo, Launchable, ProjectUrl, Provide, Translation,
    },
    error::ParseError,
    AppId, ContentRating, Language, License, MarkupTranslatableString, Release, Requirement,
    Screenshot, TranslatableList, TranslatableString,
};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// A component is wrapper around a `metainfo.xml` file or previously an
/// `appdata.xml` file. It describes an application to the various stores out
/// there on Linux.
pub struct Component {
    #[serde(default, rename = "type")]
    /// The component type.
    pub kind: ComponentKind,
    /// Unique identifier for this component.
    pub id: AppId,
    /// A human-readable name.
    pub name: TranslatableString,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Absolute requirements of the component. See
    /// <https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-relations>.
    pub requires: Vec<Requirement>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Recomended requirements of the component. See
    /// <https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-relations>.
    pub recommends: Vec<Requirement>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Supported features of the component. See
    /// <https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-relations>.
    pub supports: Vec<Requirement>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// A short summary of the component.
    pub summary: Option<TranslatableString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// A long description of this component.
    pub description: Option<MarkupTranslatableString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The license of the compoonent.
    pub project_license: Option<License>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The license the metainfo XML file released under.
    pub metadata_license: Option<License>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Identify the project with a specific upstream umbrella.
    /// Known values includes: GNOME, KDE, XFCE, MATE, LXDE.
    pub project_group: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Indicate for which desktop environment the component is essential for
    /// its functionality.
    pub compulsory_for_desktop: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The various AppId that the current component extends.
    pub extends: Vec<AppId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The icons of the component.
    pub icons: Vec<Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The component screenshots, composed of either images, videos or both.
    pub screenshots: Vec<Screenshot>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Web URLs.
    pub urls: Vec<ProjectUrl>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The developers or the projects responsible for the development of the
    /// project.
    pub developer_name: Option<TranslatableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Used by distributors to contact the project.
    /// The information should not be exposed to the user.
    pub update_contact: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The categories this component is associated with.
    pub categories: Vec<Category>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Possible methods to launch the software.
    pub launchables: Vec<Launchable>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The pkgname, a distributor thing.
    pub pkgname: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The source pkgname, a distributor thing.
    pub source_pkgname: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// 3rd-party sources to grab the component from.
    pub bundles: Vec<Bundle>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Metainformation that describes the various releases.
    pub releases: Vec<Release>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The languages supported by the component.
    pub languages: Vec<Language>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The MIME types the component supports.
    pub mimetypes: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Defines the "awesomeness" of a component.
    pub kudos: Vec<Kudo>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// A list of keywords, to help the user find the component easily.
    pub keywords: Option<TranslatableList>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Specifies the age rating of the component.
    pub content_rating: Option<ContentRating>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Public interfaces the component provides.
    pub provides: Vec<Provide>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Specifies the translation domains.
    pub translations: Vec<Translation>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Suggested components to install.
    pub suggestions: Vec<AppId>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    /// Custom metadata.
    pub metadata: HashMap<String, Option<String>>,
}

impl Component {
    /// Create a new `Component` from an XML file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the component.
    pub fn from_path(path: PathBuf) -> Result<Self, ParseError> {
        let file = BufReader::new(File::open(path)?);
        let component = Component::try_from(&Element::parse(file)?)?;
        Ok(component)
    }

    #[cfg(feature = "gzip")]
    /// Create a new `Component` from a gzipped XML file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the gzipped component.
    pub fn from_gzipped(path: PathBuf) -> Result<Self, ParseError> {
        let f = File::open(path)?;

        let d = GzDecoder::new(f);
        let element = Element::parse(d)?;

        let component: Component = Component::try_from(&element)?;
        Ok(component)
    }

    #[cfg(feature = "gzip")]
    /// Create a new `Component` from a gzipped bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte slice (gzip compressed).
    pub fn from_gzipped_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        let d = GzDecoder::new(bytes);
        let element = Element::parse(d)?;

        let component: Component = Component::try_from(&element)?;
        Ok(component)
    }
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use chrono::{TimeZone, Utc};
    use url::Url;

    use super::Component;
    use crate::{
        builders::{
            ArtifactBuilder, ComponentBuilder, ImageBuilder, LanguageBuilder, ReleaseBuilder,
            ScreenshotBuilder,
        },
        enums::{
            ArtifactKind, Bundle, Category, ComponentKind, ContentRatingVersion, FirmwareKind,
            Icon, ImageKind, Kudo, Launchable, ProjectUrl, Provide, ReleaseKind, Translation,
        },
        ContentRating, MarkupTranslatableString, TranslatableList, TranslatableString,
    };

    #[test]
    fn addon_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/addon.xml".into())?;

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
            .url(ProjectUrl::Homepage(Url::parse(
                "http://projects.gnome.org/gedit",
            )?))
            .extend("org.gnome.gedit".into())
            .build();
        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn codec_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/codec.xml".into())?;

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
        Ok(())
    }

    #[test]
    fn desktop_application_component() -> Result<(), Box<dyn Error>> {
        let c1: Component = Component::from_path("./tests/desktop.xml".into())?;

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
                Url::parse("http://www.gnome.org/projects/en_US/gnome-power-manager")?,
            ))
            .provide(Provide::Binary("gnome-power-statistics".into()))
            .provide(Provide::Id("gnome-power-statistics.desktop".into()))
            .icon(Icon::Cached { path: "org.gnome.gnome-power-statistics.png".into(), width: Some(128), height: Some(128), scale: Some(2) })
            .screenshot(
                ScreenshotBuilder::default()
                    .caption(TranslatableString::with_default("The options dialog"))
                    .image(
                        ImageBuilder::new(
                            Url::parse("http://www.hughsie.com/en_US/main.png")?,
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
                            Url::parse("http://www.hughsie.com/en_US/preferences.png")?,
                        )
                        .build(),
                    )
                    .build(),
            )
            .release(
                ReleaseBuilder::new("3.12.2")
                    .description(MarkupTranslatableString::with_default("<p>Fixes issues X, Y and Z</p>"))
                    .date(Utc.with_ymd_and_hms(2013, 4, 12, 0, 0, 0).unwrap())
                    .build(),
            )
            .build();
        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn driver_component() -> Result<(), Box<dyn Error>> {
        let c1: Component = Component::from_path("./tests/driver.xml".into())?;

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
                Url::parse("http://www.nvidia.com/Download/index.aspx")?,
            ))
            .build();
        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn firmware_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/firmware.xml".into())?;

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
                Url::parse("http://www.hughski.com/")?,
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
                    .date(Utc.with_ymd_and_hms(2015, 2, 16, 0, 0, 0).unwrap())
                    .artifact(
                        ArtifactBuilder::default()
                        .url(Url::parse("http://www.hughski.com/downloads/colorhug-als/firmware/colorhug-als-3.0.2.cab")?)
                        .kind(ArtifactKind::Binary)
                        .build()
                    )
                    .description(MarkupTranslatableString::with_default("<p>This stable release fixes the following bugs:</p><ul><li>Fix the return code from GetHardwareVersion</li><li>Scale the output of TakeReadingRaw by the datasheet values</li></ul>"))
                    .build(),
            )
            .build();

        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn font_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/font.xml".into())?;

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
        Ok(())
    }

    #[test]
    fn generic_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/generic.xml".into())?;

        let c2 = ComponentBuilder::default()
            .id("com.example.foobar".into())
            .name(TranslatableString::with_default("Foo Bar"))
            .metadata_license("CC0-1.0".into())
            .summary(TranslatableString::with_default("A foo-ish bar"))
            .url(ProjectUrl::Homepage(Url::parse("http://www.example.org")?))
            .developer_name(TranslatableString::with_default("FooBar Team"))
            .provide(Provide::Library("libfoobar.so.2".into()))
            .provide(Provide::Font("foo.ttf".into()))
            .provide(Provide::Binary("foobar".into()))
            .release(
                ReleaseBuilder::new("1.2")
                    .date(Utc.with_ymd_and_hms(2015, 2, 16, 0, 0, 0).unwrap())
                    .build(),
            )
            .build();
        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn icon_theme_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/icon-theme.xml".into())?;

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
                        Url::parse("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png")?
                    )
                    .build()
                )
                .build()
            )
            .build();

        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn input_method_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/input-method.xml".into())?;

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
                Url::parse("https://github.com/mike-fabian/ibus-table-others")?,
            ))
            .build();

        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn localization_component() -> Result<(), Box<dyn Error>> {
        let c1 = Component::from_path("./tests/localization.xml".into())?;

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
            .url(ProjectUrl::Homepage(Url::parse(
                "http://i18n.kde.org/team-infos.php?teamcode=de",
            )?))
            .language(LanguageBuilder::new("de_DE").build())
            .language(LanguageBuilder::new("de_AT").percentage(96).build())
            .language(LanguageBuilder::new("de").percentage(100).build())
            .build();
        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn os_component() -> Result<(), Box<dyn Error>> {
        let c1: Component = Component::from_path("./tests/os.xml".into())?;

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
                Url::parse("https://www.debian.org/")?,
            ))
            .metadata_license("FSFAP".into())
            .developer_name(TranslatableString::with_default("The Debian Project"))
            .release(
                ReleaseBuilder::new("10.0")
                    .kind(ReleaseKind::Development)
                    .description(MarkupTranslatableString::with_default("<p>The next release of Debian.</p>"))
                    .build(),
            )
            .release(
                ReleaseBuilder::new("9.0")
                    .description(MarkupTranslatableString::with_default("<p>Now contains the Linux kernel 4.9, GNOME 3.22, KDE Plasma 5, LibreOffice 5.2 and Qt 5.7. LXQt has been added.</p>"))
                    .date(Utc.with_ymd_and_hms(2017, 7, 17, 0, 0, 0).unwrap())
                    .date_eol(Utc.with_ymd_and_hms(2020, 7, 17, 0, 0, 0).unwrap())
                    .build(),
            )
            .build();
        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn runtime_component() -> Result<(), Box<dyn Error>> {
        let c1: Component = Component::from_path("./tests/runtime.xml".into())?;

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
                Url::parse("https://freedesktop-sdk.gitlab.io/")?,
            ))
            .release(ReleaseBuilder::new("10.0").build())
            .release(
                ReleaseBuilder::new("9.0")
                    .date(Utc.with_ymd_and_hms(2020, 01, 12, 0, 0, 0).unwrap())
                    .build(),
            )
            .build();

        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    #[cfg(feature = "test_json")]
    fn serde_json_component() -> Result<(), Box<dyn Error>> {
        let file = std::fs::File::open("./tests/app-com.github.utsushi.Utsushi.json")?;
        let c: Component = serde_json::from_reader(&file)?;

        assert_eq!(
            c.icons,
            vec![
                Icon::Stock("scanner".to_string()),
                Icon::Cached {
                    path: "com.github.utsushi.Utsushi.png".into(),
                    width: Some(64),
                    height: Some(64),
                    scale: Some(2)
                }
            ]
        );
        Ok(())
    }

    #[test]
    fn contrast_metainfo_component() -> Result<(), Box<dyn Error>> {
        use crate::{AppId, Control, DisplayLength, DisplayLengthValue, Requirement};

        let c1: Component =
            Component::from_path("./tests/app-org.gnome.design.Contrast.xml".into())?;

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

        let app_id_req = Requirement::AppId(AppId::from("org.gnome.design.AppIconPreview"));
        let display_length = Requirement::DisplayLength(DisplayLength {
            value: DisplayLengthValue::Value(360),
            compare: Default::default(),
            side: Default::default(),
        });
        let keyboard = Requirement::Control(Control::Keyboard);

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
            .suggest("org.gnome.design.Palette".into())
            .requires(app_id_req)
            .requires(display_length)
            .supports(keyboard)
            .bundle(Bundle::Flatpak {
                runtime: Some("org.gnome.Platform/x86_64/3.36".into()),
                sdk: Some("org.gnome.Sdk/x86_64/3.36".into()),
                reference: "app/org.gnome.design.Contrast/x86_64/stable".into()
            })
            .url(ProjectUrl::BugTracker(Url::parse("https://gitlab.gnome.org/World/design/contrast/issues")?))
            .url(ProjectUrl::Donation(Url::parse("https://liberapay.com/bielmoussaoui")?))
            .url(ProjectUrl::Homepage(Url::parse("https://gitlab.gnome.org/World/design/contrast")?))
            .url(ProjectUrl::Translate(Url::parse("https://l10n.gnome.org/module/contrast/")?))
            .translation(Translation::Gettext("contrast".into()))
            .launchable(Launchable::DesktopId("org.gnome.design.Contrast.desktop".into()))
            .developer_name(TranslatableString::with_default("Bilal Elmoussaoui"))
            .metadata("x-appcenter-suggested-price".to_string(), Some("5".to_string()))
            .icon(Icon::Cached {
                path: "org.gnome.design.Contrast.png".into(),
                width: Some(64),
                height: Some(64),
                scale: None,
            })
            .icon(Icon::Cached {
                path: "org.gnome.design.Contrast.png".into(),
                width: Some(128),
                height: Some(128),
                scale: None,
            }).content_rating(ContentRating {
                attributes: vec![],
                version: ContentRatingVersion::Oars1_0
            })
            .release(
                ReleaseBuilder::new("0.0.3")
                    .date(Utc.datetime_from_str("1582329600", "%s")?)
                    .description(MarkupTranslatableString::with_default("<p>Stylesheet fixes</p><p>Translations updates</p>"))
                    .build()
            )
            .release(
                ReleaseBuilder::new("0.0.2")
                    .date(Utc.datetime_from_str("1566691200", "%s")?)
                    .description(MarkupTranslatableString::with_default("<p>Translations updates</p>"))
                    .build()
            )
            .release(
                ReleaseBuilder::new("0.0.1")
                    .date(Utc.datetime_from_str("1565136000", "%s")?)
                    .description(MarkupTranslatableString::with_default("<p>First release of Contrast</p>"))
                    .build()
            )
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
                            Url::parse("https://gitlab.gnome.org/World/design/contrast/raw/master/data/resources/screenshots/screenshot1.png")?
                        ).build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/624x351/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png")?
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(624)
                        .height(351)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/112x63/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png")?
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(112)
                        .height(63)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/224x126/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png")?
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(224)
                        .height(126)
                        .build()
                    )
                    .image(
                        ImageBuilder::new(
                            Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/752x423/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png")?
                        )
                        .kind(ImageKind::Thumbnail)
                        .width(752)
                        .height(423)
                        .build()
                    ).build()
                )
            .build();

        assert_eq!(c1, c2);
        Ok(())
    }
}
