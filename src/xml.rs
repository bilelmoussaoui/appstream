use super::Component;
use std::convert::TryFrom;
use url::Url;
use xmltree::Element;

use super::builders::ComponentBuilder;
use super::enums::{
    Bundle, Category, ComponentKind, Icon, Kudo, Launchable, ProjectUrl, Provide, Translation,
};
use super::types::{
    AppId, ContentRating, Language, License, Release, Screenshot, TranslatableString,
    TranslatableVec,
};

impl TryFrom<&Element> for AppId {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        Ok(e.get_text().unwrap().into_owned().into())
    }
}

impl TryFrom<&Element> for License {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        Ok(e.get_text().unwrap().into_owned().into())
    }
}

impl TryFrom<&Element> for Component {
    type Error = anyhow::Error;
    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let mut component = ComponentBuilder::default();

        if let Some(kind) = e.attributes.get("type") {
            component = component.kind(kind.as_str().into());
        }

        let app_id = AppId::try_from(e.get_child("id").expect("The 'id' tag is required"))?;

        let mut name = TranslatableString::default();
        let mut summary = TranslatableString::default();
        let mut developer_name = TranslatableString::default();
        let mut keywords = TranslatableVec::default();
        let mut description = TranslatableString::default();
        description.set_is_markup(true);
        for node in &e.children {
            match node {
                xmltree::XMLNode::Element(ref e) => match &*e.name {
                    "name" => name.add_for_element(e),
                    "summary" => summary.add_for_element(e),
                    "developer_name" => developer_name.add_for_element(e),
                    "description" => description.add_for_element(e),
                    "project_license" => {
                        component = component.project_license(License::try_from(e)?);
                    }
                    "metadata_license" => {
                        component = component.metadata_license(License::try_from(e)?);
                    }
                    "update_contact" => {
                        component = component.update_contact(&e.get_text().unwrap().into_owned());
                    }
                    "project_group" => {
                        component = component.project_group(&e.get_text().unwrap().into_owned());
                    }
                    "compulsory_for_desktop" => {
                        component =
                            component.compulsory_for_desktop(&e.get_text().unwrap().into_owned());
                    }
                    "pkgname" => {
                        component = component.pkgname(&e.get_text().unwrap().into_owned());
                    }
                    "source_pkgname" => {
                        component = component.source_pkgname(&e.get_text().unwrap().into_owned());
                    }
                    "keywords" => {
                        e.children.iter().for_each(|c| {
                            keywords
                                .add_for_element(c.as_element().expect("invalid 'keywords' format"))
                        });
                    }
                    "extends" => {
                        for child in e.children.iter() {
                            component = component.extend(AppId::try_from(
                                child.as_element().expect("invalid extend tag"),
                            )?);
                        }
                    }
                    "launchable" => {
                        component = component.launchable(Launchable::try_from(e)?);
                    }
                    "url" => {
                        component = component.url(ProjectUrl::try_from(e)?);
                    }
                    "bundle" => {
                        component = component.bundle(Bundle::try_from(e)?);
                    }
                    _ => (),
                },
                _ => (),
            };
        }
        component = component
            .name(name)
            .summary(summary)
            .keywords(keywords)
            .description(description)
            .developer_name(developer_name)
            .id(app_id);
        Ok(component.build())
    }
}

impl TryFrom<&Element> for Launchable {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();
        Ok(match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "cockpit-manifest" => Launchable::CockpitManifest(val),
                "desktop-id" => Launchable::DesktopId(val),
                "service" => Launchable::Service(val),
                "url" => Launchable::Url(Url::parse(&val).expect("invalid url in launchable")),
                _ => anyhow::bail!("Invalid launchable type {}", t),
            },
            None => anyhow::bail!("launchable tag required a type"),
        })
    }
}

impl TryFrom<&Element> for Bundle {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();
        match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "tarball" => Ok(Bundle::Tarball(val)),
                "snap" => Ok(Bundle::Snap(val)),
                "appimage" => Ok(Bundle::AppImage(val)),
                "limba" => Ok(Bundle::Limba(val)),
                "flatpak" => Ok(Bundle::Flatpak {
                    runtime: e.attributes.get("runtime").map(|r| r.to_string()),
                    sdk: e
                        .attributes
                        .get("sdk")
                        .expect("Flatpak bundle requires an sdk")
                        .to_string(),
                    reference: e
                        .get_text()
                        .expect("Flatpak bundle requires a reference")
                        .to_string(),
                }),
                _ => anyhow::bail!("Invalid bundle type {}", t),
            },
            None => anyhow::bail!("bundle tag required a type"),
        }
    }
}

impl TryFrom<&Element> for ProjectUrl {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();
        Ok(match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "help" => ProjectUrl::Help(Url::parse(&val)?),
                "homepage" => ProjectUrl::Homepage(Url::parse(&val)?),
                "donation" => ProjectUrl::Donation(Url::parse(&val)?),
                "contact" => ProjectUrl::Contact(Url::parse(&val)?),
                "translate" => ProjectUrl::Translate(Url::parse(&val)?),
                "faq" => ProjectUrl::Faq(Url::parse(&val)?),
                "bugtracker" => ProjectUrl::BugTracker(Url::parse(&val)?),
                _ => anyhow::bail!("Invalid url type {}", t),
            },
            None => anyhow::bail!("url requires a type attribute"),
        })
    }
}
