use super::{AppId, Component};
use anyhow::Result;
#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
use quick_xml::de::from_str;
use serde::Deserialize;
#[cfg(feature = "gzip")]
use std::fs::File;
#[cfg(feature = "gzip")]
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Collection {
    pub version: String,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(rename = "component", default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub architecture: Option<String>,
}

impl Collection {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let xml = std::fs::read_to_string(path)?;

        let collection: Collection = from_str(&xml)?;
        Ok(collection)
    }

    #[cfg(feature = "gzip")]
    pub fn from_gzipped(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;

        let mut d = GzDecoder::new(f);
        let mut xml = String::new();
        d.read_to_string(&mut xml)?;

        let collection: Collection = from_str(&xml)?;
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
    use crate::enums::{Category, ComponentKind, Icon, ImageKind, ProjectUrl, Provide};
    use crate::TranslatableVec;
    use crate::{
        CollectionBuilder, ComponentBuilder, ImageBuilder, ReleaseBuilder, ScreenshotBuilder,
        TranslatableString,
    };
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn spec_example_collection() {
        let c1 = Collection::from_path("./tests/collections/spec_example.xml".into()).unwrap();

        let c2 = CollectionBuilder::new("0.10")
        .component(
            ComponentBuilder::new(
                "org.mozilla.Firefox".into(),
                TranslatableString::with_default("Firefox").and_locale("en_GB", "Firefoux")
            )
            .kind(ComponentKind::DesktopApplication)
            .pkgname("firefox-bin")
            .project_license("MPL-2.0".into())
            .keywords(TranslatableVec::with_default(vec!["internet","web", "browser"]).and_locale("fr_FR", vec!["navigateur"]))
            .summary(TranslatableString::with_default("Web browser").and_locale("fr_FR", "Navigateur web"))
            .url(ProjectUrl::Homepage(Url::from_str("https://www.mozilla.com").unwrap()))
            .screenshot(
                ScreenshotBuilder::new()
                .image(
                    ImageBuilder::new(Url::from_str("https://www.awesomedistro.example.org/en_US/firefox.desktop/main.png").unwrap())
                        .width(800)
                        .height(600)
                        .build(),
                )
                .image(
                    ImageBuilder::new(Url::from_str("https://www.awesomedistro.example.org/en_US/firefox.desktop/main-small.png").unwrap())
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
            ComponentBuilder::new(
                "org.freedesktop.PulseAudio".into(),
                TranslatableString::with_default("PulseAudio")
            )
            .summary(TranslatableString::with_default("The PulseAudio sound server"))
            .project_license("GPL-2.0+".into())
            .url(ProjectUrl::Homepage(Url::from_str("https://www.freedesktop.org/wiki/Software/PulseAudio/").unwrap()))
            .provide(Provide::Library("libpulse-simple.so.0".into()))
            .provide(Provide::Library("libpulse.so.0".into()))
            .provide(Provide::Binary("start-pulseaudio-kde".into()))
            .provide(Provide::Binary("start-pulseaudio-x11".into()))
            .release(ReleaseBuilder::new("2.0").build())
            .build()
        )
        .component(
            ComponentBuilder::new(
                "org.linuxlibertine.LinuxLibertine".into(),
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
                ComponentBuilder::new(
                    "adobe-release-x86_64".into(),
                    TranslatableString::with_default("Adobe"),
                )
                .pkgname("adobe-release-x86_64")
                .metadata_license("CC0-1.0".into())
                .summary(TranslatableString::with_default(
                    "Adobe Repository Configuration",
                ))
                .build(),
            )
            .component(
                ComponentBuilder::new(
                    "livna-release".into(),
                    TranslatableString::with_default("Livna"),
                )
                .pkgname("livna-release")
                .metadata_license("CC0-1.0".into())
                .summary(TranslatableString::with_default(
                    "Livna Repository Configuration",
                ))
                .build(),
            )
            .component(
                ComponentBuilder::new(
                    "rpmfusion-free-release".into(),
                    TranslatableString::with_default("RPM Fusion Free"),
                )
                .pkgname("rpmfusion-free-release")
                .metadata_license("CC0-1.0".into())
                .summary(TranslatableString::with_default(
                    "RPM Fusion Repository Configuration",
                ))
                .build(),
            )
            .component(
                ComponentBuilder::new(
                    "rpmfusion-nonfree-release".into(),
                    TranslatableString::with_default("RPM Fusion Non-Free"),
                )
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
                ComponentBuilder::new(
                    "epiphany-kindlecloud.desktop".into(),
                    TranslatableString::with_default("Kindle Cloud Reader")
                )
                .kind(ComponentKind::WebApplication)
                .metadata_license("CC0-1.0".into())
                .project_license("proprietary".into())
                .summary(TranslatableString::with_default("Read instantly in your browser"))
                .icon(Icon::Remote{
                    width: None,
                    height: None,
                    url: Url::from_str("http://g-ecx.images-amazon.com/images/G/01/kindle/www/ariel/kindle-icon-kcp120._SL90_.png").unwrap()
                })
                .category(Category::Education)
                .category(Category::Literature)
                .keywords(TranslatableVec::with_default(vec!["book", "ebook", "reader"]))
                .url(ProjectUrl::Homepage(Url::from_str("https://read.amazon.com").unwrap()))
                .build()
            )
            .build();

        assert_eq!(c1, c2);
    }
}
