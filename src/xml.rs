use super::error::ParseError;
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
use super::{
    AppId, Artifact, ContentRating, Image, Language, License, MarkupTranslatableString, Release,
    Screenshot, TranslatableList, TranslatableString, Video,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};

fn deserialize_date(date: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    Utc.datetime_from_str(&date, "%s").or_else(
        |_: chrono::ParseError| -> Result<DateTime<Utc>, chrono::ParseError> {
            let date: NaiveDateTime =
                NaiveDate::parse_from_str(&date, "%Y-%m-%d")?.and_hms(0, 0, 0);
            Ok(DateTime::<Utc>::from_utc(date, Utc))
        },
    )
}

impl TryFrom<&Element> for AppId {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        Ok(e.get_text().unwrap().into_owned().into())
    }
}

impl TryFrom<&Element> for Artifact {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let mut artifact = ArtifactBuilder::default();

        if let Some(kind) = e.attributes.get("type") {
            let kind = ArtifactKind::from_str(kind).map_err(|_| {
                ParseError::InvalidValue(
                    kind.to_string(),
                    "type".to_string(),
                    "artifact".to_string(),
                )
            })?;
            artifact = artifact.kind(kind);
        }

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
    type Error = ParseError;

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
                        .ok_or_else(|| {
                            ParseError::MissingAttribute("sdk".to_string(), "bundle".to_string())
                        })?
                        .to_string(),
                    reference: val,
                }),
                _ => Err(ParseError::InvalidValue(
                    t.to_string(),
                    "type".to_string(),
                    "bundle".to_string(),
                )),
            },
            None => Err(ParseError::MissingAttribute(
                "type".to_string(),
                "bundle".to_string(),
            )),
        }
    }
}

impl TryFrom<&Element> for Checksum {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();
        match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "sha1" => Ok(Checksum::Sha1(val)),
                "sha256" => Ok(Checksum::Sha256(val)),
                "blake2b" => Ok(Checksum::Blake2b(val)),
                "blake2s" => Ok(Checksum::Blake2s(val)),
                _ => Err(ParseError::InvalidValue(
                    t.to_string(),
                    "type".to_string(),
                    "checksum".to_string(),
                )),
            },
            None => Err(ParseError::MissingAttribute(
                "type".to_string(),
                "provide".to_string(),
            )),
        }
    }
}

impl TryFrom<&Element> for Collection {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let version = e.attributes.get("version").ok_or_else(|| {
            ParseError::MissingAttribute("version".to_string(), "collection".to_string())
        })?;

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
    type Error = ParseError;
    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let mut component = ComponentBuilder::default();

        if let Some(kind) = e.attributes.get("type") {
            component = component.kind(ComponentKind::from_str(kind.as_str()).map_err(|_| {
                ParseError::InvalidValue(
                    kind.to_string(),
                    "type".to_string(),
                    "component".to_string(),
                )
            })?);
        }

        let app_id = AppId::try_from(
            e.get_child("id")
                .ok_or_else(|| ParseError::MissingTag("id".to_string()))?,
        )?;

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
                            let category = child
                                .as_element()
                                .ok_or_else(|| ParseError::InvalidTag("category".to_string()))?
                                .get_text()
                                .unwrap()
                                .to_string();
                            component = component.category(Category::from_str(&category).map_err(
                                |_| {
                                    ParseError::InvalidValue(
                                        category,
                                        "$value".to_string(),
                                        "category".to_string(),
                                    )
                                },
                            )?);
                        }
                    }
                    "source_pkgname" => {
                        component = component.source_pkgname(&e.get_text().unwrap().into_owned());
                    }
                    "keywords" => {
                        for c in e.children.iter() {
                            keywords.add_for_element(
                                c.as_element().ok_or_else(|| {
                                    ParseError::InvalidTag("keywords".to_string())
                                })?,
                            );
                        }
                    }
                    "kudos" => {
                        for child in e.children.iter() {
                            let kudo = child
                                .as_element()
                                .ok_or_else(|| ParseError::InvalidTag("kudo".to_string()))?
                                .get_text()
                                .unwrap()
                                .to_string();
                            component = component.kudo(Kudo::from_str(&kudo).map_err(|_| {
                                ParseError::InvalidValue(
                                    kudo,
                                    "$value".to_string(),
                                    "kudo".to_string(),
                                )
                            })?);
                        }
                    }
                    "mimetypes" => {
                        for child in e.children.iter() {
                            component = component.mimetype(
                                &child
                                    .as_element()
                                    .ok_or_else(|| ParseError::InvalidTag("mimetype".to_string()))?
                                    .get_text()
                                    .unwrap()
                                    .to_string(),
                            );
                        }
                    }
                    "screenshots" => {
                        for child in e.children.iter() {
                            component = component.screenshot(Screenshot::try_from(
                                child.as_element().ok_or_else(|| {
                                    ParseError::InvalidTag("screenshots".to_string())
                                })?,
                            )?);
                        }
                    }

                    "releases" => {
                        for child in e.children.iter() {
                            component = component.release(Release::try_from(
                                child.as_element().ok_or_else(|| {
                                    ParseError::InvalidTag("releases".to_string())
                                })?,
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
                                child.as_element().ok_or_else(|| {
                                    ParseError::InvalidTag("languages".to_string())
                                })?,
                            )?);
                        }
                    }
                    "provides" => {
                        for child in e.children.iter() {
                            component = component.provide(Provide::try_from(
                                child.as_element().ok_or_else(|| {
                                    ParseError::InvalidTag("prorivdes".to_string())
                                })?,
                            )?);
                        }
                    }
                    "url" => {
                        component = component.url(ProjectUrl::try_from(e)?);
                    }
                    "bundle" => {
                        component = component.bundle(Bundle::try_from(e)?);
                    }
                    "suggests" => {
                        for child in e.children.iter() {
                            component = component.suggest(AppId::try_from(
                                child
                                    .as_element()
                                    .ok_or_else(|| ParseError::InvalidTag("id".to_string()))?,
                            )?);
                        }
                    }
                    "metadata" => {
                        for child in e.children.iter() {
                            let child = child
                                .as_element()
                                .ok_or_else(|| ParseError::InvalidTag("value".to_string()))?;

                            let key = child.attributes.get("key").ok_or_else(|| {
                                ParseError::MissingAttribute("key".to_string(), "value".to_string())
                            })?;

                            let value = child.get_text().map(|v| v.to_string());
                            component = component.metadata(key.to_string(), value);
                        }
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

impl TryFrom<&Element> for ContentRating {
    type Error = ParseError;

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
            attributes.push(ContentAttribute::try_from(child.as_element().ok_or_else(
                || ParseError::InvalidTag("content-attribute".to_string()),
            )?)?);
        }
        Ok(Self {
            version,
            attributes,
        })
    }
}

impl TryFrom<&Element> for ContentAttribute {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap().into_owned();

        let val = ContentState::from_str(&val).map_err(|_| {
            ParseError::InvalidValue(val, "$value".to_string(), "content-attribute".to_string())
        })?;

        match e.attributes.get("id").as_deref() {
            Some(t) => match t.as_str() {
                "violence-cartoon" => Ok(ContentAttribute::ViolenceCartoon(val)),
                "violence-fantasy" => Ok(ContentAttribute::ViolenceFantasy(val)),
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
                id => Err(ParseError::InvalidValue(
                    id.to_string(),
                    "id".to_string(),
                    "content-attribute".to_string(),
                )),
            },
            None => Err(ParseError::MissingAttribute(
                "id".to_string(),
                "content-attribute".to_string(),
            )),
        }
    }
}

impl TryFrom<&Element> for Icon {
    type Error = ParseError;

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

impl TryFrom<&Element> for Image {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let url = Url::parse(&e.get_text().unwrap().into_owned())?;
        let mut img = ImageBuilder::new(url);

        let kind = match e.attributes.get("type") {
            Some(t) => ImageKind::from_str(t).map_err(|_| {
                ParseError::InvalidValue(t.to_string(), "type".to_string(), "image".to_string())
            })?,
            None => ImageKind::Source,
        };

        img = img.kind(kind);

        if let Some(w) = e.attributes.get("width") {
            img = img.width(w.parse::<u32>().map_err(|_| {
                ParseError::InvalidValue(w.to_string(), "width".to_string(), "image".to_string())
            })?);
        }

        if let Some(h) = e.attributes.get("height") {
            img = img.height(h.parse::<u32>().map_err(|_| {
                ParseError::InvalidValue(h.to_string(), "height".to_string(), "image".to_string())
            })?);
        }

        Ok(img.build())
    }
}

impl TryFrom<&Element> for Language {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let locale = e.get_text().unwrap().into_owned();

        match e.attributes.get("percentage") {
            Some(p) => {
                let percentage = p.parse::<u32>().map_err(|_| {
                    ParseError::InvalidValue(
                        p.to_string(),
                        "percentage".to_string(),
                        "language".to_string(),
                    )
                })?;
                Ok(Self {
                    locale,
                    percentage: Some(percentage),
                })
            }
            None => Ok(Self {
                locale,
                percentage: None,
            }),
        }
    }
}

impl TryFrom<&Element> for Launchable {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = match e.get_text() {
            Some(v) => v.into_owned(),
            None => "".to_string(),
        };
        let kind = match e.attributes.get("type") {
            Some(k) => k.as_str(),
            None => "unknown",
        };
        Ok(match kind {
            "cockpit-manifest" => Launchable::CockpitManifest(val),
            "desktop-id" => Launchable::DesktopId(val),
            "service" => Launchable::Service(val),
            "url" => Launchable::Url(Url::parse(&val)?),
            _ => Launchable::Unknown(val),
        })
    }
}

impl TryFrom<&Element> for License {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        Ok(e.get_text().unwrap().into_owned().into())
    }
}

impl TryFrom<&Element> for ProjectUrl {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e
            .get_text()
            .ok_or_else(|| ParseError::MissingValue("url".to_string()))?
            .into_owned();

        match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "help" => Ok(ProjectUrl::Help(Url::parse(&val)?)),
                "homepage" => Ok(ProjectUrl::Homepage(Url::parse(&val)?)),
                "donation" => Ok(ProjectUrl::Donation(Url::parse(&val)?)),
                "contact" => Ok(ProjectUrl::Contact(Url::parse(&val)?)),
                "translate" => Ok(ProjectUrl::Translate(Url::parse(&val)?)),
                "faq" => Ok(ProjectUrl::Faq(Url::parse(&val)?)),
                "bugtracker" => Ok(ProjectUrl::BugTracker(Url::parse(&val)?)),
                _ => Ok(ProjectUrl::Unknown(Url::parse(&val)?)),
            },
            None => Err(ParseError::MissingAttribute(
                "type".to_string(),
                "url".to_string(),
            )),
        }
    }
}

impl TryFrom<&Element> for Provide {
    type Error = ParseError;

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
            "firmware" => match e.attributes.get("type") {
                Some(kind) => {
                    let kind = FirmwareKind::from_str(kind).map_err(|_| {
                        ParseError::InvalidValue(
                            kind.to_string(),
                            "type".to_string(),
                            "firmware".to_string(),
                        )
                    })?;
                    Ok(Provide::Firmware { kind, item: val })
                }
                None => Err(ParseError::MissingAttribute(
                    "type".to_string(),
                    "firmware".to_string(),
                )),
            },
            t => Err(ParseError::InvalidValue(
                t.to_string(),
                "type".to_string(),
                "provide".to_string(),
            )),
        }
    }
}

impl TryFrom<&Element> for Release {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let version = e
            .attributes
            .get("version")
            .ok_or_else(|| {
                ParseError::MissingAttribute("version".to_string(), "release".to_string())
            })?
            .to_string();

        let mut release = ReleaseBuilder::new(&version);

        let date = e.attributes.get("date").map(|d| {
            deserialize_date(d).map_err(|_| {
                ParseError::InvalidValue(d.to_string(), "date".to_string(), "release".to_string())
            })
        });
        if let Some(d) = date {
            release = release.date(d?);
        }

        // In case we have a timestamp attribute instead of a date one
        let timestamp = e.attributes.get("timestamp").map(|d| {
            deserialize_date(d).map_err(|_| {
                ParseError::InvalidValue(
                    d.to_string(),
                    "timestamp".to_string(),
                    "release".to_string(),
                )
            })
        });
        if let Some(d) = timestamp {
            release = release.date(d?);
        }

        let date_eol = e.attributes.get("date_eol").map(|d| {
            deserialize_date(d).map_err(|_| {
                ParseError::InvalidValue(
                    d.to_string(),
                    "date_eol".to_string(),
                    "release".to_string(),
                )
            })
        });
        if let Some(d) = date_eol {
            release = release.date_eol(d?);
        }

        if let Some(urgency) = e.attributes.get("urgency") {
            let urgency = ReleaseUrgency::from_str(urgency).map_err(|_| {
                ParseError::InvalidValue(
                    urgency.to_string(),
                    "urgency".to_string(),
                    "release".to_string(),
                )
            })?;
            release = release.urgency(urgency);
        }

        if let Some(kind) = e.attributes.get("type") {
            let kind = ReleaseKind::from_str(kind).map_err(|_| {
                ParseError::InvalidValue(
                    kind.to_string(),
                    "type".to_string(),
                    "release".to_string(),
                )
            })?;
            release = release.kind(kind);
        }

        let mut description = MarkupTranslatableString::default();

        for node in &e.children {
            if let xmltree::XMLNode::Element(ref c) = node {
                match &*c.name {
                    "artifacts" => {
                        for child in c.children.iter() {
                            release = release.artifact(Artifact::try_from(
                                child.as_element().ok_or_else(|| {
                                    ParseError::InvalidTag("artifact".to_string())
                                })?,
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

impl TryFrom<&Element> for Screenshot {
    type Error = ParseError;

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

impl TryFrom<&Element> for Size {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e
            .get_text()
            .ok_or_else(|| ParseError::MissingValue("size".to_string()))?
            .into_owned();

        match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "download" => Ok(Size::Download(val.parse::<u64>().map_err(|_| {
                    ParseError::InvalidValue(val, "download".to_string(), "size".to_string())
                })?)),
                "installed" => Ok(Size::Installed(val.parse::<u64>().map_err(|_| {
                    ParseError::InvalidValue(val, "installed".to_string(), "size".to_string())
                })?)),
                _ => Err(ParseError::InvalidValue(
                    t.to_string(),
                    "type".to_string(),
                    "size".to_string(),
                )),
            },
            None => Err(ParseError::MissingAttribute(
                "type".to_string(),
                "size".to_string(),
            )),
        }
    }
}

impl TryFrom<&Element> for Translation {
    type Error = ParseError;

    fn try_from(e: &Element) -> Result<Self, Self::Error> {
        let val = e.get_text().unwrap_or_default().into_owned();
        match e.attributes.get("type").as_deref() {
            Some(t) => match t.as_str() {
                "gettext" => Ok(Translation::Gettext(val)),
                "qt" => Ok(Translation::Qt(val)),
                _ => Err(ParseError::InvalidValue(
                    t.to_string(),
                    "type".to_string(),
                    "translation".to_string(),
                )),
            },
            None => Err(ParseError::MissingAttribute(
                "type".to_string(),
                "translation".to_string(),
            )),
        }
    }
}

impl TryFrom<&Element> for Video {
    type Error = ParseError;

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
            video = video.width(w.parse::<u32>().map_err(|_| {
                ParseError::InvalidValue(w.to_string(), "width".to_string(), "video".to_string())
            })?);
        }

        if let Some(h) = e.attributes.get("height") {
            video = video.height(h.parse::<u32>().map_err(|_| {
                ParseError::InvalidValue(h.to_string(), "height".to_string(), "video".to_string())
            })?);
        }

        Ok(video.build())
    }
}
