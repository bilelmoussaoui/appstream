use super::types::AppId;
use super::Component;
use anyhow::Result;
#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs::File;
#[cfg(feature = "gzip")]
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use xmltree::Element;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Collection {
    pub version: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
}

impl Collection {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let file = BufReader::new(File::open(path)?);
        let collection = Collection::try_from(&Element::parse(file)?)?;
        Ok(collection)
    }

    #[cfg(feature = "gzip")]
    pub fn from_gzipped(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;

        let mut d = GzDecoder::new(f);
        let mut xml = String::new();
        d.read_to_string(&mut xml)?;

        let collection: Collection = Collection::try_from(&Element::parse(xml.as_bytes())?)?;

        Ok(collection)
    }

    pub fn find_by_id(&self, id: AppId) -> Vec<&Component> {
        self.components
            .iter()
            .filter(|c| c.id == id)
            .collect::<Vec<&Component>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::{
        CollectionBuilder, ComponentBuilder, ImageBuilder, ReleaseBuilder, ScreenshotBuilder,
    };
    use crate::enums::{Category, ComponentKind, Icon, ImageKind, ProjectUrl, Provide};
    use crate::types::{MarkupTranslatableString, TranslatableList, TranslatableString};
    use anyhow::Result;
    use url::Url;

    #[test]
    fn spec_example_collection() {
        let c1 = Collection::from_path("./tests/collections/spec_example.xml".into()).unwrap();

        let c2 = CollectionBuilder::new("0.10")
        .component(
            ComponentBuilder::default()
            .id("org.mozilla.Firefox".into())
            .name(
                TranslatableString::with_default("Firefox").and_locale("en_GB", "Firefoux")
            )
            .kind(ComponentKind::DesktopApplication)
            .pkgname("firefox-bin")
            .project_license("MPL-2.0".into())
            .keywords(TranslatableList::with_default(vec!["internet","web", "browser"]).and_locale("fr_FR", vec!["navigateur"]))
            .summary(TranslatableString::with_default("Web browser").and_locale("fr_FR", "Navigateur web"))
            .url(ProjectUrl::Homepage(Url::parse("https://www.mozilla.com").unwrap()))
            .screenshot(
                ScreenshotBuilder::default()
                .image(
                    ImageBuilder::new(Url::parse("https://www.awesomedistro.example.org/en_US/firefox.desktop/main.png").unwrap())
                        .width(800)
                        .height(600)
                        .build(),
                )
                .image(
                    ImageBuilder::new(Url::parse("https://www.awesomedistro.example.org/en_US/firefox.desktop/main-small.png").unwrap())
                        .kind(ImageKind::Thumbnail)
                        .width(200)
                        .height(150)
                        .build(),
                )
                .build()
            )
            .provide(Provide::Binary("firefox".into()))
            .mimetype("text/html")
            .mimetype("text/xml")
            .mimetype("application/xhtml+xml")
            .mimetype("application/vnd.mozilla.xul+xml")
            .mimetype("text/mml")
            .mimetype("application/x-xpinstall")
            .mimetype("x-scheme-handler/http")
            .mimetype("x-scheme-handler/https")
            .category(Category::Unknown("network".into()))
            .category(Category::Unknown("webbrowser".into()))
            .icon(Icon::Stock("web-browser".into()))
            .icon(Icon::Cached {
                path: "firefox.png".into(),
                width: None,
                height: None
            })
            .build()
        )
        .component(
            ComponentBuilder::default()
            .id("org.freedesktop.PulseAudio".into())
            .name(
                TranslatableString::with_default("PulseAudio")
            )
            .summary(TranslatableString::with_default("The PulseAudio sound server"))
            .project_license("GPL-2.0+".into())
            .url(ProjectUrl::Homepage(Url::parse("https://www.freedesktop.org/wiki/Software/PulseAudio/").unwrap()))
            .provide(Provide::Library("libpulse-simple.so.0".into()))
            .provide(Provide::Library("libpulse.so.0".into()))
            .provide(Provide::Binary("start-pulseaudio-kde".into()))
            .provide(Provide::Binary("start-pulseaudio-x11".into()))
            .release(ReleaseBuilder::new("2.0").build())
            .build()
        )
        .component(
            ComponentBuilder::default()
            .id(
                "org.linuxlibertine.LinuxLibertine".into()
            )
            .name(
                TranslatableString::with_default("Linux Libertine")
            )
            .kind(ComponentKind::Font)
            .summary(TranslatableString::with_default("Linux Libertine Open fonts"))
            .provide(Provide::Font("LinLibertine_M.otf".into()))
            .build()
        )
        .build();
        assert_eq!(c1, c2);
    }

    #[test]
    fn generic_collection() {
        let c1 =
            Collection::from_path("./tests/collections/fedora-other-repos.xml".into()).unwrap();

        let c2 = CollectionBuilder::new("0.8")
            .component(
                ComponentBuilder::default()
                    .id("adobe-release-x86_64".into())
                    .name(TranslatableString::with_default("Adobe"))
                    .pkgname("adobe-release-x86_64")
                    .metadata_license("CC0-1.0".into())
                    .summary(TranslatableString::with_default(
                        "Adobe Repository Configuration",
                    ))
                    .build(),
            )
            .component(
                ComponentBuilder::default()
                    .id("livna-release".into())
                    .name(TranslatableString::with_default("Livna"))
                    .pkgname("livna-release")
                    .metadata_license("CC0-1.0".into())
                    .summary(TranslatableString::with_default(
                        "Livna Repository Configuration",
                    ))
                    .build(),
            )
            .component(
                ComponentBuilder::default()
                    .id("rpmfusion-free-release".into())
                    .name(TranslatableString::with_default("RPM Fusion Free"))
                    .pkgname("rpmfusion-free-release")
                    .metadata_license("CC0-1.0".into())
                    .summary(TranslatableString::with_default(
                        "RPM Fusion Repository Configuration",
                    ))
                    .build(),
            )
            .component(
                ComponentBuilder::default()
                    .id("rpmfusion-nonfree-release".into())
                    .name(TranslatableString::with_default("RPM Fusion Non-Free"))
                    .pkgname("rpmfusion-nonfree-release")
                    .metadata_license("CC0-1.0".into())
                    .summary(TranslatableString::with_default(
                        "RPM Fusion Repository Configuration",
                    ))
                    .build(),
            )
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn web_collection() {
        let c1 = Collection::from_path("./tests/collections/fedora-web-apps.xml".into()).unwrap();

        let c2 = CollectionBuilder::new("0.8")
            .component(
                ComponentBuilder::default()
                .id("epiphany-kindlecloud.desktop".into())
                .name(TranslatableString::with_default("Kindle Cloud Reader"))
                .kind(ComponentKind::WebApplication)
                .metadata_license("CC0-1.0".into())
                .project_license("proprietary".into())
                .summary(TranslatableString::with_default("Read instantly in your browser"))
                .description(
                    MarkupTranslatableString::with_default(
                        "<p>\n        Buy Once, Read Everywhere: You don\'t need to own a Kindle device to\n        enjoy Kindle books.\n        Automatically save and synchronize your furthest page read, bookmarks,\n        notes, and highlights across all your devices.\n        That means you can start reading a book on one device, and pick up where\n        you left off on another device.\n            </p><p>\n        Read the first chapter of a book before you decide whether to buy it.\n        Read thousands of free books with a Kindle app, including popular\n        classics like The Adventures of Sherlock Holmes, Pride and Prejudice,\n        and Treasure Island.\n            </p><p>\n        To use the Kindle Cloud reader you must have Amazon.com account.\n            </p>"
                    )
                )
                .icon(Icon::Remote{
                    width: None,
                    height: None,
                    url: Url::parse("http://g-ecx.images-amazon.com/images/G/01/kindle/www/ariel/kindle-icon-kcp120._SL90_.png").unwrap()
                })
                .category(Category::Education)
                .category(Category::Literature)
                .keywords(TranslatableList::with_default(vec!["book", "ebook", "reader"]))
                .url(ProjectUrl::Homepage(Url::parse("https://read.amazon.com").unwrap()))
                .build()
            )
            .build();

        assert_eq!(c1, c2);
    }

    #[test]
    fn endless_os_collection() {
        let c1: Result<Collection> =
            Collection::from_path("./tests/collections/endless-apps.xml".into());
        // assert_eq!(c1.is_ok(), true);
        let collection = c1.unwrap();
        assert_eq!(631, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);
    }

    #[test]
    fn gnome_collection() {
        let c1: Result<Collection> =
            Collection::from_path("./tests/collections/gnome-apps.xml".into());
        // assert_eq!(c1.is_ok(), true);
        let collection = c1.unwrap();
        assert_eq!(24, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);
    }

    #[test]
    fn kde_collection() {
        let c1: Result<Collection> =
            Collection::from_path("./tests/collections/kde-apps.xml".into());
        assert_eq!(c1.is_ok(), true);
        let collection = c1.unwrap();
        assert_eq!(69, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);
    }

    #[test]
    fn flathub_collection() {
        let c1: Result<Collection> =
            Collection::from_path("./tests/collections/flathub-apps.xml".into());
        // assert_eq!(c1.is_ok(), true);
        let collection = c1.unwrap();
        assert_eq!(376, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);
    }
}
