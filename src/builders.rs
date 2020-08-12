use super::enums::*;
use super::{
    AppId, Collection, Component, ContentRating, Language, License, Release, Screenshot, Video,
};
use super::{TranslatableString, TranslatableVec};
use chrono::{DateTime, Utc};
use url::Url;

pub struct CollectionBuilder {
    pub version: String,
    pub origin: Option<String>,
    pub components: Vec<Component>,
}

impl CollectionBuilder {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            origin: None,
            components: vec![],
        }
    }

    pub fn origin(mut self, origin: &str) -> Self {
        self.origin = Some(origin.to_string());
        self
    }

    pub fn component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    pub fn build(self) -> Collection {
        Collection {
            version: self.version,
            origin: self.origin,
            components: self.components,
        }
    }
}

pub struct ComponentBuilder {
    pub kind: ComponentKind,
    pub id: AppId,
    pub name: TranslatableString,
    pub summary: Option<TranslatableString>,
    pub project_license: Option<License>,
    pub metadata_license: Option<License>,
    pub project_group: Option<String>,
    pub compulsory_for_desktop: Option<String>,
    pub extends: Vec<AppId>,
    pub icons: Vec<Icon>,
    pub screenshots: Vec<Screenshot>,
    pub urls: Vec<ProjectUrl>,
    pub developer_name: Option<TranslatableString>,
    pub update_contact: Option<String>,
    pub categories: Vec<Category>,
    pub launchables: Vec<Launchable>,
    pub pkgname: Option<String>,
    pub bundle: Vec<Bundle>,
    pub releases: Vec<Release>,
    pub languages: Vec<Language>,
    pub mimetypes: Vec<String>,
    pub kudos: Vec<Kudo>,
    pub keywords: Option<TranslatableVec>,
    pub content_rating: Option<ContentRating>,
    pub provides: Vec<Provide>,
    pub translation: Vec<Translation>,
}

#[allow(dead_code)]
impl ComponentBuilder {
    pub fn new(id: AppId, name: TranslatableString) -> Self {
        Self {
            kind: ComponentKind::Generic,
            id,
            name,
            summary: None,
            project_license: None,
            metadata_license: None,
            project_group: None,
            compulsory_for_desktop: None,
            extends: vec![],
            icons: vec![],
            screenshots: vec![],
            urls: vec![],
            developer_name: None,
            update_contact: None,
            categories: vec![],
            launchables: vec![],
            pkgname: None,
            bundle: vec![],
            releases: vec![],
            languages: vec![],
            mimetypes: vec![],
            kudos: vec![],
            keywords: None,
            content_rating: None,
            provides: vec![],
            translation: vec![],
        }
    }

    pub fn kind(mut self, kind: ComponentKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn developer_name(mut self, developer_name: TranslatableString) -> Self {
        self.developer_name = Some(developer_name);
        self
    }

    pub fn summary(mut self, summary: TranslatableString) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn metadata_license(mut self, license: License) -> Self {
        self.metadata_license = Some(license);
        self
    }

    pub fn project_license(mut self, license: License) -> Self {
        self.project_license = Some(license);
        self
    }

    pub fn keywords(mut self, keywords: TranslatableVec) -> Self {
        self.keywords = Some(keywords);
        self
    }

    pub fn project_group(mut self, group: &str) -> Self {
        self.project_group = Some(group.to_string());
        self
    }

    pub fn url(mut self, url: ProjectUrl) -> Self {
        self.urls.push(url);
        self
    }

    pub fn screenshot(mut self, screenshot: Screenshot) -> Self {
        self.screenshots.push(screenshot);
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icons.push(icon);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.languages.push(language);
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.categories.push(category);
        self
    }

    pub fn mimetype(mut self, mimetype: &str) -> Self {
        self.mimetypes.push(mimetype.to_string());
        self
    }

    pub fn extend(mut self, extend: AppId) -> Self {
        self.extends.push(extend);
        self
    }

    pub fn release(mut self, release: Release) -> Self {
        self.releases.push(release);
        self
    }

    pub fn launchable(mut self, launchable: Launchable) -> Self {
        self.launchables.push(launchable);
        self
    }

    pub fn provide(mut self, provide: Provide) -> Self {
        self.provides.push(provide);
        self
    }

    pub fn pkgname(mut self, pkgname: &str) -> Self {
        self.pkgname = Some(pkgname.to_string());
        self
    }

    pub fn update_contact(mut self, update_contact: &str) -> Self {
        self.update_contact = Some(update_contact.to_string());
        self
    }

    pub fn build(self) -> Component {
        Component {
            kind: self.kind,
            id: self.id,
            name: self.name,
            summary: self.summary,
            project_license: self.project_license,
            metadata_license: self.metadata_license,
            project_group: self.project_group,
            compulsory_for_desktop: self.compulsory_for_desktop,
            extends: self.extends,
            icons: self.icons,
            screenshots: self.screenshots,
            urls: self.urls,
            developer_name: self.developer_name,
            update_contact: self.update_contact,
            categories: self.categories,
            launchables: self.launchables,
            pkgname: self.pkgname,
            bundle: self.bundle,
            releases: self.releases,
            languages: self.languages,
            mimetypes: self.mimetypes,
            kudos: self.kudos,
            keywords: self.keywords,
            content_rating: self.content_rating,
            provides: self.provides,
            translation: self.translation,
        }
    }
}

pub struct LanguageBuilder {
    pub percentage: Option<u32>,
    pub locale: String,
}

#[allow(dead_code)]
impl LanguageBuilder {
    pub fn new(locale: &str) -> Self {
        Self {
            percentage: None,
            locale: locale.to_string(),
        }
    }

    pub fn percentage(mut self, percentage: u32) -> Self {
        self.percentage = Some(percentage);
        self
    }

    pub fn build(self) -> Language {
        Language {
            locale: self.locale,
            percentage: self.percentage,
        }
    }
}

pub struct ReleaseBuilder {
    pub date: Option<DateTime<Utc>>,
    pub date_eol: Option<DateTime<Utc>>,
    pub version: String,
    pub kind: Option<ReleaseKind>,
    pub sizes: Vec<ReleaseSize>,
}

#[allow(dead_code)]
impl ReleaseBuilder {
    pub fn new(version: &str) -> Self {
        Self {
            date: None,
            date_eol: None,
            kind: Some(ReleaseKind::Stable),
            sizes: vec![],
            version: version.to_string(),
        }
    }

    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }
    pub fn date_eol(mut self, date_eol: DateTime<Utc>) -> Self {
        self.date_eol = Some(date_eol);
        self
    }
    pub fn kind(mut self, kind: ReleaseKind) -> Self {
        self.kind = Some(kind);
        self
    }
    pub fn sizes(mut self, sizes: Vec<ReleaseSize>) -> Self {
        self.sizes = sizes;
        self
    }

    pub fn build(self) -> Release {
        let kind = self.kind.unwrap_or_default();
        Release {
            version: self.version,
            date: self.date,
            date_eol: self.date_eol,
            kind,
            sizes: self.sizes,
        }
    }
}

pub struct ScreenshotBuilder {
    pub is_default: Option<bool>,
    pub caption: Option<TranslatableString>,
    pub images: Vec<Image>,
    pub videos: Vec<Video>,
}

#[allow(dead_code)]
impl ScreenshotBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn caption(mut self, caption: TranslatableString) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn set_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    pub fn image(mut self, image: Image) -> Self {
        self.images.push(image);
        self
    }

    pub fn images(mut self, images: Vec<Image>) -> Self {
        self.images = images;
        self
    }

    pub fn video(mut self, video: Video) -> Self {
        self.videos.push(video);
        self
    }

    pub fn videos(mut self, videos: Vec<Video>) -> Self {
        self.videos = videos;
        self
    }

    pub fn build(self) -> Screenshot {
        let mut s = Screenshot::default();
        s.videos = self.videos;
        s.images = self.images;
        if let Some(is_default) = self.is_default {
            s.is_default = is_default;
        }
        s.caption = self.caption;
        s
    }
}

impl Default for ScreenshotBuilder {
    fn default() -> Self {
        Self {
            is_default: None,
            caption: None,
            videos: vec![],
            images: vec![],
        }
    }
}

pub struct VideoBuilder {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub url: Url,
}

#[allow(dead_code)]
impl VideoBuilder {
    pub fn new(url: Url) -> Self {
        Self {
            width: None,
            height: None,
            container: None,
            codec: None,
            url,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn container(mut self, container: &str) -> Self {
        self.container = Some(container.to_string());
        self
    }

    pub fn codec(mut self, codec: &str) -> Self {
        self.codec = Some(codec.to_string());
        self
    }

    pub fn build(self) -> Video {
        Video {
            width: self.width,
            height: self.height,
            codec: self.codec,
            container: self.container,
            url: self.url,
        }
    }
}
