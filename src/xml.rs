use super::{Collection, Component};
use std::convert::TryFrom;
use std::str::FromStr;
use url::Url;
use xmltree::Element;

use super::builders::{
    ArtifactBuilder, CollectionBuilder, ComponentBuilder, ImageBuilder, ReleaseBuilder,
    ScreenshotBuilder, VideoBuilder,
};
use super::enums::{
    ArtifactKind, Bundle, Category, Checksum, ComponentKind, ContentAttribute,
    ContentRatingVersion, ContentState, FirmwareKind, Icon, ImageKind, Kudo, Launchable,
    ProjectUrl, Provide, ReleaseKind, ReleaseUrgency, Size, Translation,
};
use super::types::{
    AppId, Artifact, ContentRating, Image, Language, License, MarkupTranslatableString, Release,
    Screenshot, TranslatableList, TranslatableString, Video,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};

impl TryFrom<&Element> for AppId {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        Ok(e.get_text().unwrap().into_owned().into())
    }
}

impl TryFrom<&Element> for Artifact {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let mut artifact = ArtifactBuilder::default();

        let kind = e
            .attributes
            .get("type")
            .map(|t| ArtifactKind::from_str(t).expect("invalid artifact type"))
            .expect("release artifact requires a type attribute");
        artifact = artifact.kind(kind);

        if let Some(platform) = e.attributes.get("platform") {
            artifact = artifact.platform(platform);
        }

        for node in &e.children {
            if let xmltree::XMLNode::Element(ref e) = node {
                match &*e.name {
                    "location" => {
                        let url = Url::parse(&e.get_text().unwrap().to_string())?;
                        artifact = artifact.url(url);
                    }
                    "size" => {
                        artifact = artifact.size(Size::try_from(e)?);
                    }
                    "checksum" => {
                        artifact = artifact.checksum(Checksum::try_from(e)?);
                    }
                    _ => (),
                }
            }
        }

        Ok(artifact.build())
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
                    reference: val,
                }),
                _ => anyhow::bail!("Invalid bundle type {}", t),
            },
            None => anyhow::bail!("bundle tag required a type"),
        }
    }
}

impl TryFrom<&Element> for Collection {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let version = e
            .attributes
            .get("version")
            .expect("A collection requires a version attribute");
        let mut collection = CollectionBuilder::new(version);

        if let Some(arch) = e.attributes.get("architecture") {
            collection = collection.architecture(arch);
        }

        if let Some(origin) = e.attributes.get("origin") {
            if !origin.is_empty() {
                collection = collection.origin(origin);
            }
        }

        for node in &e.children {
            if let xmltree::XMLNode::Element(ref e) = node {
                if &*e.name == "component" {
                    collection = collection.component(Component::try_from(e)?);
                }
            }
        }
        Ok(collection.build())
    }
}

impl TryFrom<&Element> for Component {
    type Error = anyhow::Error;
    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let mut component = ComponentBuilder::default();

        if let Some(kind) = e.attributes.get("type") {
            component = component.kind(ComponentKind::from_str(kind.as_str()).unwrap());
        }

        let app_id = AppId::try_from(e.get_child("id").expect("The 'id' tag is required"))?;

        let mut name = TranslatableString::default();
        let mut summary = TranslatableString::default();
        let mut developer_name = TranslatableString::default();
        let mut keywords = TranslatableList::default();
        let mut description = MarkupTranslatableString::default();
        for node in &e.children {
            if let xmltree::XMLNode::Element(ref e) = node {
                match &*e.name {
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
                    "icon" => {
                        component = component.icon(Icon::try_from(e)?);
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
                    "categories" => {
                        for child in e.children.iter() {
                            component = component.category(Category::from_str(
                                &child
                                    .as_element()
                                    .expect("invalid category tag")
                                    .get_text()
                                    .unwrap()
                                    .to_string(),
                            )?);
                        }
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
                    "kudos" => {
                        for child in e.children.iter() {
                            component = component.kudo(Kudo::from_str(
                                &child
                                    .as_element()
                                    .expect("invalid kudo tag")
                                    .get_text()
                                    .unwrap()
                                    .to_string(),
                            )?);
                        }
                    }
                    "mimetypes" => {
                        for child in e.children.iter() {
                            component = component.mimetype(
                                &child
                                    .as_element()
                                    .expect("Invalid mimetype tag")
                                    .get_text()
                                    .unwrap()
                                    .to_string(),
                            );
                        }
                    }
                    "screenshots" => {
                        for child in e.children.iter() {
                            component = component.screenshot(Screenshot::try_from(
                                child.as_element().expect("invalid screenshot tag"),
                            )?);
                        }
                    }

                    "releases" => {
                        for child in e.children.iter() {
                            component = component.release(Release::try_from(
                                child.as_element().expect("invalid release tag"),
                            )?);
                        }
                    }
                    "extends" => {
                        component = component.extend(AppId::try_from(e)?);
                    }
                    "translation" => {
                        component = component.translation(Translation::try_from(e)?);
                    }
                    "launchable" => {
                        component = component.launchable(Launchable::try_from(e)?);
                    }
                    "content_rating" => {
                        component = component.content_rating(ContentRating::try_from(e)?);
                    }
                    "languages" => {
                        for child in e.children.iter() {
                            component = component.language(Language::try_from(
                                child.as_element().expect("invalid languages tag"),
                            )?);
                        }
                    }
                    "provides" => {
                        for child in e.children.iter() {
                            component = component.provide(Provide::try_from(
                                child.as_element().expect("invalid provides tag"),
                            )?);
                        }
                    }
                    "url" => {
                        component = component.url(ProjectUrl::try_from(e)?);
                    }
                    "bundle" => {
                        component = component.bundle(Bundle::try_from(e)?);
                    }
                    _ => (),
                }
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

impl TryFrom<&Element> for Icon {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();
        let kind = match e.attributes.get("type") {
            Some(t) => t.as_str(),
            None => "local",
        };

        Ok(match kind {
            "stock" => Icon::Stock(val),
            "cached" => Icon::Cached {
                path: val.into(),
                width: e.attributes.get("width").map(|w| w.parse::<u32>().unwrap()),
                height: e
                    .attributes
                    .get("height")
                    .map(|w| w.parse::<u32>().unwrap()),
            },
            "remote" => Icon::Remote {
                url: Url::parse(&val)?,
                width: e.attributes.get("width").map(|w| w.parse::<u32>().unwrap()),
                height: e
                    .attributes
                    .get("height")
                    .map(|w| w.parse::<u32>().unwrap()),
            },
            _ => Icon::Local {
                path: val.into(),
                width: e.attributes.get("width").map(|w| w.parse::<u32>().unwrap()),
                height: e
                    .attributes
                    .get("height")
                    .map(|w| w.parse::<u32>().unwrap()),
            },
        })
    }
}

impl TryFrom<&Element> for Language {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let locale = e.get_text().unwrap().into_owned();

        let percentage = e
            .attributes
            .get("percentage")
            .map(|p| p.parse::<u32>().unwrap());
        Ok(Self { locale, percentage })
    }
}

impl TryFrom<&Element> for Checksum {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();
        Ok(match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "sha1" => Checksum::Sha1(val),
                "sha256" => Checksum::Sha256(val),
                "blake2b" => Checksum::Blake2b(val),
                "blake2s" => Checksum::Blake2s(val),
                _ => anyhow::bail!("Invalid checksum type {}", t),
            },
            None => anyhow::bail!("checksum tag requires a type"),
        })
    }
}

impl TryFrom<&Element> for Translation {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap_or_default().into_owned();
        Ok(match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "gettext" => Translation::Gettext(val),
                "qt" => Translation::Qt(val),
                _ => anyhow::bail!("Invalid translation type {}", t),
            },
            None => anyhow::bail!("translation tag required a type"),
        })
    }
}

impl TryFrom<&Element> for Launchable {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = match e.get_text() {
            Some(v) => v.into_owned(),
            None => "".to_string(),
        };
        let kind = match e.attributes.get("type") {
            Some(k) => k.as_str(),
            None => "unkown",
        };
        Ok(match kind {
            "cockpit-manifest" => Launchable::CockpitManifest(val),
            "desktop-id" => Launchable::DesktopId(val),
            "service" => Launchable::Service(val),
            "url" => Launchable::Url(Url::parse(&val).expect("invalid url in launchable")),
            _ => Launchable::Unknown(val),
        })
    }
}

impl TryFrom<&Element> for License {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        Ok(e.get_text().unwrap().into_owned().into())
    }
}

impl TryFrom<&Element> for ProjectUrl {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().expect("url tag requires a value").into_owned();
        Ok(match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "help" => ProjectUrl::Help(Url::parse(&val)?),
                "homepage" => ProjectUrl::Homepage(Url::parse(&val)?),
                "donation" => ProjectUrl::Donation(Url::parse(&val)?),
                "contact" => ProjectUrl::Contact(Url::parse(&val)?),
                "translate" => ProjectUrl::Translate(Url::parse(&val)?),
                "faq" => ProjectUrl::Faq(Url::parse(&val)?),
                "bugtracker" => ProjectUrl::BugTracker(Url::parse(&val)?),
                _ => ProjectUrl::Unknown(Url::parse(&val)?),
            },
            None => anyhow::bail!("url requires a type attribute"),
        })
    }
}

impl TryFrom<&Element> for Provide {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();

        match e.name.as_ref() {
            "library" => Ok(Provide::Library(val.into())),
            "binary" => Ok(Provide::Binary(val)),
            "font" => Ok(Provide::Font(val)),
            "modalias" => Ok(Provide::Modalias(val)),
            "python2" => Ok(Provide::Python2(val)),
            "python3" => Ok(Provide::Python3(val)),
            "dbus" => Ok(Provide::DBus(val)),
            "id" => Ok(Provide::Id(val.into())),
            "codec" => Ok(Provide::Codec(val)),
            "firmware" => {
                let kind = e
                    .attributes
                    .get("type")
                    .map(|k| FirmwareKind::from_str(k).expect("invalid release kind"))
                    .expect("firmware provides tag requires a type attribute");

                Ok(Provide::Firmware { kind, item: val })
            }
            _ => anyhow::bail!("Invalid provides tag"),
        }
    }
}

fn deserialize_date(date: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    chrono::Utc.datetime_from_str(&date, "%s").or_else(
        |_: chrono::ParseError| -> Result<DateTime<Utc>, chrono::ParseError> {
            let date: NaiveDateTime =
                NaiveDate::parse_from_str(&date, "%Y-%m-%d")?.and_hms(0, 0, 0);
            Ok(DateTime::<Utc>::from_utc(date, chrono::Utc))
        },
    )
}

impl TryFrom<&Element> for Release {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let version = e
            .attributes
            .get("version")
            .expect("release tag requires a version attribute")
            .to_string();

        let mut release = ReleaseBuilder::new(&version);

        let date = e
            .attributes
            .get("date")
            .map(|d| deserialize_date(d).expect("invalid date format for release date"));
        if let Some(d) = date {
            release = release.date(d);
        }

        // In case we have a timestamp attribute instead of a date one
        let timestamp = e
            .attributes
            .get("timestamp")
            .map(|d| deserialize_date(d).expect("invalid date format for release date"));
        if let Some(d) = timestamp {
            release = release.date(d);
        }

        let date_eol = e
            .attributes
            .get("date_eol")
            .map(|d| deserialize_date(d).expect("invalid date format for release date_eol"));
        if let Some(d) = date_eol {
            release = release.date_eol(d);
        }
        let urgency = e
            .attributes
            .get("urgency")
            .map(|k| ReleaseUrgency::from_str(k).expect("invalid release urgency"))
            .unwrap_or_default();
        release = release.urgency(urgency);

        let kind = e
            .attributes
            .get("type")
            .map(|k| ReleaseKind::from_str(k).expect("invalid release kind"))
            .unwrap_or_default();
        release = release.kind(kind);

        let mut description = MarkupTranslatableString::default();
        for node in &e.children {
            if let xmltree::XMLNode::Element(ref c) = node {
                match &*c.name {
                    "artifacts" => {
                        for child in c.children.iter() {
                            release = release.artifact(Artifact::try_from(
                                child.as_element().expect("invalid artifact tag"),
                            )?);
                        }
                    }
                    "size" => {
                        release = release.size(Size::try_from(c)?);
                    }
                    "description" => description.add_for_element(c),
                    "url" => {
                        release = release.url(Url::parse(&c.get_text().unwrap().to_string())?);
                    }
                    _ => (),
                }
            }
        }

        Ok(release.description(description).build())
    }
}

impl TryFrom<&Element> for Size {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().expect("url tag requires a value").into_owned();
        Ok(match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "download" => Size::Download(val.parse::<u64>().expect("invalid download size")),
                "installed" => Size::Installed(val.parse::<u64>().expect("invalid installed size")),
                _ => anyhow::bail!("invalid release size type"),
            },
            None => anyhow::bail!("url requires a type attribute"),
        })
    }
}
impl TryFrom<&Element> for Screenshot {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let mut s = ScreenshotBuilder::default().set_default(
            e.attributes
                .get("type")
                .map(|t| t.as_str() == "default")
                .unwrap_or_else(|| false),
        );
        let mut caption = TranslatableString::default();
        for node in &e.children {
            if let xmltree::XMLNode::Element(ref e) = node {
                match &*e.name {
                    "image" => {
                        s = s.image(Image::try_from(e)?);
                    }
                    "caption" => {
                        caption.add_for_element(e);
                    }
                    "video" => {
                        s = s.video(Video::try_from(e)?);
                    }
                    _ => (),
                }
            }
        }
        Ok(s.caption(caption).build())
    }
}

impl TryFrom<&Element> for Image {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let url = Url::parse(&e.get_text().unwrap().into_owned())?;
        let mut img = ImageBuilder::new(url);

        let kind = match e.attributes.get("type") {
            Some(t) => ImageKind::from_str(t)?,
            None => ImageKind::Source,
        };

        img = img.kind(kind);

        if let Some(w) = e.attributes.get("width") {
            img = img.width(w.parse::<u32>().expect("the width should be an integer"));
        }

        if let Some(h) = e.attributes.get("height") {
            img = img.height(h.parse::<u32>().expect("the height should be an integer"));
        }

        Ok(img.build())
    }
}

impl TryFrom<&Element> for Video {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let url = Url::parse(&e.get_text().unwrap().into_owned())?;
        let mut video = VideoBuilder::new(url);

        if let Some(container) = e.attributes.get("container") {
            video = video.container(container);
        }

        if let Some(codec) = e.attributes.get("codec") {
            video = video.codec(codec);
        }

        if let Some(w) = e.attributes.get("width") {
            video = video.width(w.parse::<u32>().expect("the width should be an integer"));
        }

        if let Some(h) = e.attributes.get("height") {
            video = video.height(h.parse::<u32>().expect("the height should be an integer"));
        }

        Ok(video.build())
    }
}

impl TryFrom<&Element> for ContentRating {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let version: ContentRatingVersion = match e.attributes.get("type") {
            Some(t) => match t.as_str() {
                "oars-1.0" => ContentRatingVersion::Oars1_0,
                "oars-1.1" => ContentRatingVersion::Oars1_1,
                _ => ContentRatingVersion::Unknown,
            },
            None => ContentRatingVersion::Unknown,
        };

        let mut attributes: Vec<ContentAttribute> = Vec::new();
        for child in e.children.iter() {
            attributes.push(ContentAttribute::try_from(
                child.as_element().expect("invalid content-attribute tag"),
            )?);
        }
        Ok(Self {
            version,
            attributes,
        })
    }
}

impl TryFrom<&Element> for ContentAttribute {
    type Error = anyhow::Error;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = ContentState::from_str(&e.get_text().unwrap().into_owned())
            .expect("invalid content-attribute state");

        match e.attributes.get("id").as_deref() {
            Some(t) => match t.as_str() {
                "violence-cartoon" => Ok(ContentAttribute::ViolenceCartoon(val)),
                "violence-fantasy" => Ok(ContentAttribute::ViolenceFantasy(val)),
                "violence-fealistic" => Ok(ContentAttribute::ViolenceFealistic(val)),
                "violence-bloodshed" => Ok(ContentAttribute::ViolenceBloodshed(val)),
                "violence-sexual" => Ok(ContentAttribute::ViolenceSexual(val)),
                "violence-desecration" => Ok(ContentAttribute::ViolenceDesecration(val)),
                "violence-slavery" => Ok(ContentAttribute::ViolenceSlavery(val)),
                "violence-realistic" => Ok(ContentAttribute::ViolenceRealistic(val)),
                "violence-worship" => Ok(ContentAttribute::ViolenceWorship(val)),
                "drugs-alcohol" => Ok(ContentAttribute::DrugsAlcohol(val)),
                "drugs-narcotics" => Ok(ContentAttribute::DrugsNarcotics(val)),
                "drugs-tobacco" => Ok(ContentAttribute::DrugsTobacco(val)),
                "sex-nudity" => Ok(ContentAttribute::SexNudity(val)),
                "sex-themes" => Ok(ContentAttribute::SexThemes(val)),
                "sex-homosexuality" => Ok(ContentAttribute::SexHomosexuality(val)),
                "sex-prostitution" => Ok(ContentAttribute::SexProstitution(val)),
                "sex-adultery" => Ok(ContentAttribute::SexAdultery(val)),
                "sex-appearance" => Ok(ContentAttribute::SexAppearance(val)),
                "language-profanity" => Ok(ContentAttribute::LanguageProfanity(val)),
                "language-humor" => Ok(ContentAttribute::LanguageHumor(val)),
                "language-discrimination" => Ok(ContentAttribute::LanguageDiscrimination(val)),
                "social-chat" => Ok(ContentAttribute::SocialChat(val)),
                "social-info" => Ok(ContentAttribute::SocialInfo(val)),
                "social-audio" => Ok(ContentAttribute::SocialAudio(val)),
                "social-location" => Ok(ContentAttribute::SocialLocation(val)),
                "social-contacts" => Ok(ContentAttribute::SocialContacts(val)),
                "money-advertising" => Ok(ContentAttribute::MoneyAdvertising(val)),
                "money-purchasing" => Ok(ContentAttribute::MoneyPurchasing(val)),
                "money-gambling" => Ok(ContentAttribute::MoneyGambling(val)),
                _ => anyhow::bail!("invalid content-attribute id"),
            },
            None => anyhow::bail!("content-attribute tag requires a type"),
        }
    }
}
