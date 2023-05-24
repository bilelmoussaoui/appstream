use super::error::ParseError;
use super::AppId;
use super::Component;
#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use xmltree::Element;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// A collection is a wrapper around multiple components at once.
/// Provided by the source of the components (a repository).
/// See [Collection Metadata](https://www.freedesktop.org/software/appstream/docs/chap-CollectionData.html).
pub struct Collection {
    /// The specification version used on the components.
    pub version: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The origin of the collection, could be something like `flathub`.
    pub origin: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The components that are part of this collection.
    pub components: Vec<Component>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The targeted CPU architecture of the collection.
    pub architecture: Option<String>,
}

impl Collection {
    /// Create a new `Collection` from an XML file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the collection.
    pub fn from_path(path: PathBuf) -> Result<Self, ParseError> {
        let file = BufReader::new(File::open(path)?);
        let collection = Collection::try_from(&Element::parse(file)?)?;
        Ok(collection)
    }

    #[cfg(feature = "gzip")]
    /// Create a new `Collection` from a gzipped XML file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the gzipped collection.
    pub fn from_gzipped(path: PathBuf) -> Result<Self, ParseError> {
        let f = File::open(path)?;

        let d = GzDecoder::new(f);
        let element = Element::parse(d)?;
        let collection: Collection = Collection::try_from(&element)?;

        Ok(collection)
    }

    #[cfg(feature = "gzip")]
    /// Create a new `Collection` from a gzipped bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte slice (gzip compressed).
    pub fn from_gzipped_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        let d = GzDecoder::new(bytes);
        let element = Element::parse(d)?;

        let collection: Collection = Collection::try_from(&element)?;
        Ok(collection)
    }

    /// Find the components that corresponds to a specific `AppId`
    pub fn find_by_id(&self, id: AppId) -> Vec<&Component> {
        // For some obscure reasons & history
        // Some apps uses $app-id.desktop as the id on the appdata/metainfo file
        // Let's automatically check for those as well.
        let alternative_id: AppId = format!("{}.desktop", id).into();

        self.components
            .iter()
            .filter(|c| c.id == id || c.id == alternative_id)
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
    use crate::{MarkupTranslatableString, TranslatableList, TranslatableString};
    use std::error::Error;
    use url::Url;

    #[cfg(feature = "gzip")]
    #[test]
    fn flathub_latest_collection() -> Result<(), Box<dyn Error>> {
        let c1 = Collection::from_gzipped("./tests/collections/flathub.xml.gz".into())?;
        assert_eq!(c1.components.len(), 1420);

        #[cfg(feature = "test_json")]
        {
            let c2: Collection = serde_json::from_str(&serde_json::to_string(&c1)?)?;
            assert_eq!(c1, c2);
        }
        Ok(())
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn flathub_beta_collection() -> Result<(), Box<dyn Error>> {
        let c1 = Collection::from_gzipped("./tests/collections/flathub-beta.xml.gz".into())?;
        assert_eq!(c1.components.len(), 149);

        #[cfg(feature = "test_json")]
        {
            let c2: Collection = serde_json::from_str(&serde_json::to_string(&c1)?)?;
            assert_eq!(c1, c2);
        }
        Ok(())
    }

    #[test]
    fn spec_example_collection() -> Result<(), Box<dyn Error>> {
        let c1 = Collection::from_path("./tests/collections/spec_example.xml".into())?;

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
            .url(ProjectUrl::Homepage(Url::parse("https://www.mozilla.com")?))
            .screenshot(
                ScreenshotBuilder::default()
                .image(
                    ImageBuilder::new(Url::parse("https://www.awesomedistro.example.org/en_US/firefox.desktop/main.png")?)
                        .width(800)
                        .height(600)
                        .build(),
                )
                .image(
                    ImageBuilder::new(Url::parse("https://www.awesomedistro.example.org/en_US/firefox.desktop/main-small.png")?)
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
                height: None,
                scale: None,
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
            .url(ProjectUrl::Homepage(Url::parse("https://www.freedesktop.org/wiki/Software/PulseAudio/")?))
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

        Ok(())
    }

    #[test]
    fn generic_collection() -> Result<(), Box<dyn Error>> {
        let c1 = Collection::from_path("./tests/collections/fedora-other-repos.xml".into())?;

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
        Ok(())
    }

    #[test]
    fn web_collection() -> Result<(), Box<dyn Error>> {
        let c1 = Collection::from_path("./tests/collections/fedora-web-apps.xml".into())?;

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
                    scale: None,
                    url: Url::parse("http://g-ecx.images-amazon.com/images/G/01/kindle/www/ariel/kindle-icon-kcp120._SL90_.png")?
                })
                .metadata("X-Needs-Dark-Theme".to_string(), None)
                .metadata("X-Kudo-Popular".to_string(), None)
                .category(Category::Education)
                .category(Category::Literature)
                .keywords(TranslatableList::with_default(vec!["book", "ebook", "reader"]))
                .url(ProjectUrl::Homepage(Url::parse("https://read.amazon.com")?))
                .build()
            )
            .build();

        assert_eq!(c1, c2);
        Ok(())
    }

    #[test]
    fn endless_os_collection() -> Result<(), Box<dyn Error>> {
        let collection = Collection::from_path("./tests/collections/endless-apps.xml".into())?;

        assert_eq!(631, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);

        #[cfg(feature = "test_json")]
        {
            let c2: Collection = serde_json::from_str(&serde_json::to_string(&collection)?)?;
            assert_eq!(collection, c2);
        }
        Ok(())
    }

    #[test]
    fn gnome_collection() -> Result<(), Box<dyn Error>> {
        let collection = Collection::from_path("./tests/collections/gnome-apps.xml".into())?;

        assert_eq!(24, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);

        #[cfg(feature = "test_json")]
        {
            let c2: Collection = serde_json::from_str(&serde_json::to_string(&collection)?)?;
            assert_eq!(collection, c2);
        }
        Ok(())
    }

    #[test]
    fn kde_collection() -> Result<(), Box<dyn Error>> {
        let collection = Collection::from_path("./tests/collections/kde-apps.xml".into())?;
        assert_eq!(69, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);

        #[cfg(feature = "test_json")]
        {
            let c2: Collection = serde_json::from_str(&serde_json::to_string(&collection)?)?;
            assert_eq!(collection, c2);
        }
        Ok(())
    }

    #[test]
    fn flathub_collection() -> Result<(), Box<dyn Error>> {
        let collection = Collection::from_path("./tests/collections/flathub-old.xml".into())?;
        assert_eq!(376, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);

        #[cfg(feature = "test_json")]
        {
            let c2: Collection = serde_json::from_str(&serde_json::to_string(&collection)?)?;
            assert_eq!(collection, c2);
        }
        Ok(())
    }

    #[test]
    fn gnome_nightly_collection() -> Result<(), Box<dyn Error>> {
        let collection = Collection::from_path("./tests/collections/gnome-nightly.xml".into())?;
        assert_eq!(58, collection.components.len());
        assert_eq!(Some("flatpak".into()), collection.origin);
        assert_eq!("0.8", collection.version);

        #[cfg(feature = "test_json")]
        {
            let c2: Collection = serde_json::from_str(&serde_json::to_string(&collection)?)?;
            assert_eq!(collection, c2);
        }
        Ok(())
    }
}
