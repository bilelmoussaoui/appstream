use super::AppId;
use super::Component;
use anyhow::Result;
#[cfg(feature="gzip")]
use flate2::read::GzDecoder;
use quick_xml::de::from_str;
use serde::Deserialize;
#[cfg(feature="gzip")]
use std::fs::File;
#[cfg(feature="gzip")]
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Collection {
    version: String,
    #[serde(default)]
    origin: Option<String>,
    #[serde(rename = "component", default)]
    pub components: Vec<Component>,
    // TODO: architecture
}

impl Collection {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let xml = std::fs::read_to_string(path)?;

        let collection: Collection = from_str(&xml)?;
        Ok(collection)
    }

    #[cfg(feature="gzip")]
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
            .filter(|c| c.id.0 == id.0)
            .collect::<Vec<&Component>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{Category, ComponentType, Icon, Provide};
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn collection_test_1() {
        let c = Collection::from_path("./tests/collections/spec_example.xml".into()).unwrap();

        assert_eq!(c.version, "0.10");

        let first = c.components.get(0).unwrap();
        assert_eq!(first._type, ComponentType::DesktopApplication);

        assert_eq!(first.provides, vec![Provide::Binary("firefox".into())]);
        assert_eq!(
            first.categories,
            vec![
                Category::Unknown("network".into()),
                Category::Unknown("webbrowser".into())
            ]
        );
        assert_eq!(
            first.mimetypes,
            vec![
                "text/html",
                "text/xml",
                "application/xhtml+xml",
                "application/vnd.mozilla.xul+xml",
                "text/mml",
                "application/x-xpinstall",
                "x-scheme-handler/http",
                "x-scheme-handler/https",
            ]
        );
        assert_eq!(
            first.icons,
            vec![
                Icon::Stock("web-browser".into()),
                Icon::Cached("firefox.png".into()),
            ]
        );

        let second = c.components.get(1).unwrap();
        assert_eq!(second._type, ComponentType::Generic);
        assert_eq!(
            second.provides,
            vec![
                Provide::Library("libpulse-simple.so.0".into()),
                Provide::Library("libpulse.so.0".into()),
                Provide::Binary("start-pulseaudio-kde".into()),
                Provide::Binary("start-pulseaudio-x11".into()),
            ]
        );

        let third = c.components.get(2).unwrap();
        assert_eq!(third._type, ComponentType::Font);
        assert_eq!(
            third.provides,
            vec![Provide::Font("LinLibertine_M.otf".into())]
        );
    }
    #[test]
    fn collection_test_2() {
        let c = Collection::from_path("./tests/collections/fedora-other-repos.xml".into()).unwrap();

        assert_eq!(c.version, "0.8");
        c.components.iter().for_each(|comp| {
            assert_eq!(comp._type, ComponentType::Generic);
        });

        assert_eq!(
            c.components.get(0).unwrap().pkgname,
            Some("adobe-release-x86_64".into())
        );
    }
    #[test]
    fn collection_test_3() {
        let c = Collection::from_path("./tests/collections/fedora-web-apps.xml".into()).unwrap();

        assert_eq!(c.version, "0.8");
        let comp = c.components.get(0).unwrap();
        assert_eq!(comp._type, ComponentType::WebApplication);
        assert_eq!(comp.icons, vec![
            Icon::Remote{
                url: Url::from_str("http://g-ecx.images-amazon.com/images/G/01/kindle/www/ariel/kindle-icon-kcp120._SL90_.png").unwrap(),
                width: None,
                height: None
            }
        ]);
    }
}
