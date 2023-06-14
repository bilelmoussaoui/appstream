use std::collections::HashMap;

use chrono::{DateTime, Utc};
use url::Url;

use super::{
    collection::Collection, component::Component, enums::*, release::Issue, AppId, Artifact,
    ContentRating, Image, Language, License, MarkupTranslatableString, Release, Requirement,
    Screenshot, TranslatableList, TranslatableString, Video,
};

#[derive(Default, Debug)]
/// A helper to build an `Artifact`.
pub struct ArtifactBuilder {
    /// The targeted platform.
    pub platform: Option<String>,
    /// The artifact kind.
    pub kind: Option<ArtifactKind>,
    /// The downloaded/installed sizes.
    pub sizes: Vec<Size>,
    /// The download url.
    pub url: Option<Url>,
    /// The various checksums to validate the artifact.
    pub checksums: Vec<Checksum>,
    /// The various bundles to grab the artifact from other 3rd-party
    /// installers.
    pub bundles: Vec<Bundle>,
}

#[allow(dead_code)]
impl ArtifactBuilder {
    /// Sets the artifact kind.
    #[must_use]
    pub fn kind(mut self, kind: ArtifactKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the artifact download url.
    #[must_use]
    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// Adds a `Bundle` to the artifact as a 3rd-party source to install it.
    #[must_use]
    pub fn bundle(mut self, bundle: Bundle) -> Self {
        self.bundles.push(bundle);
        self
    }

    /// Adds a `Size` to the artifact to specify the downloaded/installed sizes.
    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.sizes.push(size);
        self
    }

    /// Adds a `Checksum` to the artifact.
    #[must_use]
    pub fn checksum(mut self, checksum: Checksum) -> Self {
        self.checksums.push(checksum);
        self
    }

    /// Sets the targeted platform of the artifact.
    #[must_use]
    pub fn platform(mut self, platform: &str) -> Self {
        self.platform = Some(platform.to_string());
        self
    }

    /// Construct an `Artifact`.
    #[must_use]
    pub fn build(self) -> Artifact {
        Artifact {
            url: self.url.expect("an artifact location is required"),
            kind: self.kind.expect("artifact type is required"),
            sizes: self.sizes,
            checksums: self.checksums,
            platform: self.platform,
            bundles: self.bundles,
        }
    }
}

#[derive(Default, Debug)]
/// A helper to build an `Issue`.
pub struct IssueBuilder {
    /// The issue kind.
    pub kind: Option<IssueKind>,
    /// The issue url.
    pub url: Option<Url>,
    /// The issue identifier.
    pub identifier: Option<String>,
}

#[allow(dead_code)]
impl IssueBuilder {
    /// Sets the issue kind.
    #[must_use]
    pub fn kind(mut self, kind: IssueKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the issue information url.
    #[must_use]
    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// Sets the issue identifier
    #[must_use]
    pub fn identifier(mut self, identifier: String) -> Self {
        self.identifier = Some(identifier);
        self
    }

    /// Construct an `Issue`.
    #[must_use]
    pub fn build(self) -> Issue {
        Issue {
            url: self.url,
            kind: self.kind.unwrap_or(IssueKind::Generic),
            identifier: self.identifier.expect("Issue identifier is required"),
        }
    }
}

#[derive(Debug)]
/// A helper to build a `Collection`.
pub struct CollectionBuilder {
    /// The specification version used on this collection of components.
    pub version: String,
    /// The origin of the collection. Could be something like `flathub`.
    pub origin: Option<String>,
    /// The list of components on that collection.
    pub components: Vec<Component>,
    /// The targeted CPU architecture of the collection.
    pub architecture: Option<String>,
}

#[allow(dead_code)]
impl CollectionBuilder {
    /// Create a new `CollectionBuilder`.
    ///
    /// # Arguments
    ///
    /// * `version` - The specification version used on the collection.
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            origin: None,
            components: vec![],
            architecture: None,
        }
    }

    /// Specifies the targeted architecture.
    #[must_use]
    pub fn architecture(mut self, architecture: &str) -> Self {
        self.architecture = Some(architecture.to_string());
        self
    }

    /// Sets the origin of the collection.
    #[must_use]
    pub fn origin(mut self, origin: &str) -> Self {
        self.origin = Some(origin.to_string());
        self
    }

    /// Adds a new component to the collection.
    #[must_use]
    pub fn component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    /// Construct a `Collection`.
    #[must_use]
    pub fn build(self) -> Collection {
        Collection {
            version: self.version,
            origin: self.origin,
            components: self.components,
            architecture: self.architecture,
        }
    }
}
#[derive(Default, Debug)]
/// A helper to build a `Component`.
pub struct ComponentBuilder {
    /// The component type.
    pub kind: ComponentKind,
    /// A unique identifier of the component.
    pub id: Option<AppId>,
    /// The component name.
    pub name: Option<TranslatableString>,
    /// A short summary.
    pub summary: Option<TranslatableString>,
    /// A long description that might contains markup.
    pub description: Option<MarkupTranslatableString>,
    /// The project license.
    pub project_license: Option<License>,
    /// The license of the metainfo.
    pub metadata_license: Option<License>,
    /// The project group of the component.
    pub project_group: Option<String>,
    /// Indicate for which desktop environment the component is essential for
    /// its functionality.
    pub compulsory_for_desktop: Option<String>,
    /// The various AppId that the current component extends.
    pub extends: Vec<AppId>,
    /// The component icons.
    pub icons: Vec<Icon>,
    /// The component screenshots, composed of either images, videos or both.
    pub screenshots: Vec<Screenshot>,
    /// Web URLs.
    pub urls: Vec<ProjectUrl>,
    /// The developers or the projects responsible for the development of the
    /// project.
    pub developer_name: Option<TranslatableString>,
    /// Used by distributors to contact the project.
    pub update_contact: Option<String>,
    /// The categories this component is associated with.
    pub categories: Vec<Category>,
    /// Possible methods to launch the software.
    pub launchables: Vec<Launchable>,
    /// The pkgname, a distributor thing.
    pub pkgname: Option<String>,
    /// 3rd-party sources to grab the component from.
    pub bundles: Vec<Bundle>,
    /// Metainformation that describes the various releases.
    pub releases: Vec<Release>,
    /// The languages supported by the component.
    pub languages: Vec<Language>,
    /// The MIME types the component supports.
    pub mimetypes: Vec<String>,
    /// Defines the "awesomeness" of a component.
    pub kudos: Vec<Kudo>,
    /// A list of keywords, to help the user find the component easily.
    pub keywords: Option<TranslatableList>,
    /// Specifies the age rating of the component.
    pub content_rating: Option<ContentRating>,
    /// Public interfaces the component provides.
    pub provides: Vec<Provide>,
    /// Specifies the translation domains.
    pub translations: Vec<Translation>,
    /// The source pkgname, a distributor thing.
    pub source_pkgname: Option<String>,
    /// Suggested components.
    pub suggestions: Vec<AppId>,
    /// Custom metadata
    pub metadata: HashMap<String, Option<String>>,
    /// denotes a supported requirement, this is a weaker statement that
    /// `recommends`.
    pub supports: Vec<Requirement>,
    /// denotes a recommended requirement.
    pub recommends: Vec<Requirement>,
    /// denotes an absolute requirement.
    pub requires: Vec<Requirement>,
}

#[allow(dead_code)]
impl ComponentBuilder {
    /// Sets the component's unique identifier.
    #[must_use]
    pub fn id(mut self, id: AppId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the component name.
    #[must_use]
    pub fn name(mut self, name: TranslatableString) -> Self {
        self.name = Some(name);
        self
    }

    /// Specifies the age rating of component.
    #[must_use]
    pub fn content_rating(mut self, content_rating: ContentRating) -> Self {
        self.content_rating = Some(content_rating);
        self
    }

    /// Sets the component type.
    #[must_use]
    pub fn kind(mut self, kind: ComponentKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the developer name.
    #[must_use]
    pub fn developer_name(mut self, developer_name: TranslatableString) -> Self {
        if !developer_name.is_empty() {
            self.developer_name = Some(developer_name);
        }
        self
    }

    /// Sets the component summary.
    #[must_use]
    pub fn summary(mut self, summary: TranslatableString) -> Self {
        if !summary.is_empty() {
            self.summary = Some(summary);
        }
        self
    }

    /// Sets the component description.
    #[must_use]
    pub fn description(mut self, description: MarkupTranslatableString) -> Self {
        if !description.is_empty() {
            self.description = Some(description);
        }
        self
    }

    /// Sets the metainfo license.
    #[must_use]
    pub fn metadata_license(mut self, license: License) -> Self {
        self.metadata_license = Some(license);
        self
    }

    /// Sets the project license.
    #[must_use]
    pub fn project_license(mut self, license: License) -> Self {
        self.project_license = Some(license);
        self
    }

    /// Sets the keywords.
    #[must_use]
    pub fn keywords(mut self, keywords: TranslatableList) -> Self {
        if !keywords.is_empty() {
            self.keywords = Some(keywords);
        }
        self
    }
    /// Sets which desktop environment the component is essential for its
    /// functionality.
    #[must_use]
    pub fn compulsory_for_desktop(mut self, compulsory_for_desktop: &str) -> Self {
        self.compulsory_for_desktop = Some(compulsory_for_desktop.to_string());
        self
    }

    /// Sets the upstream umberall.
    /// Known values includes: GNOME, KDE, XFCE, MATE, LXDE.
    #[must_use]
    pub fn project_group(mut self, group: &str) -> Self {
        self.project_group = Some(group.to_string());
        self
    }

    /// Suggest a component to be installed.
    #[must_use]
    pub fn suggest(mut self, id: AppId) -> Self {
        self.suggestions.push(id);
        self
    }

    /// Adds a Web URL to the component.
    #[must_use]
    pub fn url(mut self, url: ProjectUrl) -> Self {
        self.urls.push(url);
        self
    }

    /// Adds a screenshot to the component.
    #[must_use]
    pub fn screenshot(mut self, screenshot: Screenshot) -> Self {
        self.screenshots.push(screenshot);
        self
    }

    /// Adds an icon to the component.
    #[must_use]
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icons.push(icon);
        self
    }

    /// Adds a kudo to the component.
    #[must_use]
    pub fn kudo(mut self, kudo: Kudo) -> Self {
        self.kudos.push(kudo);
        self
    }

    /// Adds a translation context to the component.
    #[must_use]
    pub fn translation(mut self, translation: Translation) -> Self {
        self.translations.push(translation);
        self
    }

    /// Adds a bundle to the component.
    #[must_use]
    pub fn bundle(mut self, bundle: Bundle) -> Self {
        self.bundles.push(bundle);
        self
    }

    /// Adds a language to the component.
    #[must_use]
    pub fn language(mut self, language: Language) -> Self {
        self.languages.push(language);
        self
    }

    /// Adds a category to the component.
    #[must_use]
    pub fn category(mut self, category: Category) -> Self {
        self.categories.push(category);
        self
    }

    /// Adds a mimetype to the component.
    #[must_use]
    pub fn mimetype(mut self, mimetype: &str) -> Self {
        self.mimetypes.push(mimetype.to_string());
        self
    }

    /// Adds a component that the current one extends.
    #[must_use]
    pub fn extend(mut self, extend: AppId) -> Self {
        self.extends.push(extend);
        self
    }

    /// Adds a release to the component.
    #[must_use]
    pub fn release(mut self, release: Release) -> Self {
        self.releases.push(release);
        self
    }

    /// Adds a launchable to the component.
    #[must_use]
    pub fn launchable(mut self, launchable: Launchable) -> Self {
        self.launchables.push(launchable);
        self
    }

    /// Adds a provided interface to the component.
    #[must_use]
    pub fn provide(mut self, provide: Provide) -> Self {
        self.provides.push(provide);
        self
    }

    /// Sets the pkgname, a distributor thing.
    #[must_use]
    pub fn pkgname(mut self, pkgname: &str) -> Self {
        self.pkgname = Some(pkgname.to_string());
        self
    }

    /// Sets the source pkgname, a distributor thing.
    #[must_use]
    pub fn source_pkgname(mut self, source_pkgname: &str) -> Self {
        self.source_pkgname = Some(source_pkgname.to_string());
        self
    }

    /// Sets a way to contact the developer of the project.
    #[must_use]
    pub fn update_contact(mut self, update_contact: &str) -> Self {
        self.update_contact = Some(update_contact.to_string());
        self
    }

    /// Adds a new metadata (key, value) to the component.
    #[must_use]
    pub fn metadata(mut self, key: String, val: Option<String>) -> Self {
        self.metadata.insert(key, val);
        self
    }

    #[must_use]
    /// Adds a supports to the component.
    pub fn supports(mut self, supports: Requirement) -> Self {
        self.supports.push(supports);
        self
    }

    #[must_use]
    /// Adds a recommends to the component.
    pub fn recommends(mut self, recommends: Requirement) -> Self {
        self.recommends.push(recommends);
        self
    }

    #[must_use]
    /// Adds a requires to the component.
    pub fn requires(mut self, requires: Requirement) -> Self {
        self.requires.push(requires);
        self
    }

    /// Constructs a `Component`.
    #[must_use]
    pub fn build(self) -> Component {
        Component {
            kind: self.kind,
            id: self.id.expect("An 'id' is required"),
            name: self.name.expect("A 'name' is required"),
            requires: self.requires,
            recommends: self.recommends,
            supports: self.supports,
            summary: self.summary,
            description: self.description,
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
            bundles: self.bundles,
            releases: self.releases,
            languages: self.languages,
            mimetypes: self.mimetypes,
            kudos: self.kudos,
            keywords: self.keywords,
            content_rating: self.content_rating,
            provides: self.provides,
            translations: self.translations,
            source_pkgname: self.source_pkgname,
            suggestions: self.suggestions,
            metadata: self.metadata,
        }
    }
}

#[derive(Debug)]
/// A helper to build an `Image`.
///
/// # Example
///
/// ```
/// use url::Url;
/// use appstream::{enums::ImageKind, builders::ImageBuilder};
///
/// fn main() -> Result<(), url::ParseError> {
///     let image = ImageBuilder::new(
///                     Url::parse("https://flathub.org/repo/screenshots/org.gnome.design.Contrast-stable/112x63/org.gnome.design.Contrast-ba707a21207a348d15171063edf9790a.png")?
///                 )
///                 .kind(ImageKind::Thumbnail)
///                 .width(112)
///                 .height(63)
///                 .build();
///
///     Ok(())
/// }
/// ```
pub struct ImageBuilder {
    /// The image width.
    pub width: Option<u32>,
    /// The image height.
    pub height: Option<u32>,
    /// The URL of the image.
    pub url: Url,
    /// The type of the image.
    pub kind: ImageKind,
}

#[allow(dead_code)]
impl ImageBuilder {
    /// Creates a new `ImageBuilder`
    ///
    /// # Arguments
    ///
    /// * `url` - The image url.
    pub fn new(url: Url) -> Self {
        Self {
            width: None,
            height: None,
            url,
            kind: ImageKind::Source,
        }
    }

    /// Sets the image type, either source or thumbnail.
    #[must_use]
    pub fn kind(mut self, kind: ImageKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the image width.
    #[must_use]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the image height.
    #[must_use]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Constructs an `Image`.
    #[must_use]
    pub fn build(self) -> Image {
        Image {
            width: self.width,
            height: self.height,
            url: self.url,
            kind: self.kind,
        }
    }
}

#[derive(Debug)]
/// A helper to build a `Language`.
pub struct LanguageBuilder {
    /// A percentage represnting how complete the language support is.
    pub percentage: Option<u32>,
    /// The locale identifier, e.g fr_BE.
    pub locale: String,
}

#[allow(dead_code)]
impl LanguageBuilder {
    /// Creates a new `LanguageBuilder`
    ///
    /// # Arguments
    ///
    /// * `locale` - The language locale identifier.
    pub fn new(locale: &str) -> Self {
        Self {
            percentage: None,
            locale: locale.to_string(),
        }
    }

    /// Sets how complete the translation is in percentage.
    #[must_use]
    pub fn percentage(mut self, percentage: u32) -> Self {
        self.percentage = Some(percentage);
        self
    }

    /// Constructs a `Language`.
    #[must_use]
    pub fn build(self) -> Language {
        Language {
            locale: self.locale,
            percentage: self.percentage,
        }
    }
}

#[derive(Debug)]
/// A helper to build a `Release`.
pub struct ReleaseBuilder {
    /// The release date.
    pub date: Option<DateTime<Utc>>,
    /// The end-of-life date of the release.
    pub date_eol: Option<DateTime<Utc>>,
    /// The release description.
    pub description: Option<MarkupTranslatableString>,
    /// The version of the release.
    pub version: String,
    /// The release type.
    pub kind: Option<ReleaseKind>,
    /// The download/installed sizes of the release.
    pub sizes: Vec<Size>,
    /// The urgency to install the release.
    pub urgency: ReleaseUrgency,
    /// The release artifacts.
    pub artifacts: Vec<Artifact>,
    /// A web page containing the release changelog.
    pub url: Option<Url>,
    /// A list of issues resolved by this release.
    pub issues: Vec<Issue>,
}

#[allow(dead_code)]
impl ReleaseBuilder {
    /// Create a new `ReleaseBuilder`.
    ///
    /// # Arguments
    ///
    /// * `version` - The release's version number.
    pub fn new(version: &str) -> Self {
        Self {
            date: None,
            date_eol: None,
            description: None,
            kind: Some(ReleaseKind::Stable),
            sizes: vec![],
            version: version.to_string(),
            urgency: ReleaseUrgency::Medium,
            artifacts: vec![],
            url: None,
            issues: vec![],
        }
    }

    /// Sets the release description.
    #[must_use]
    pub fn description(mut self, description: MarkupTranslatableString) -> Self {
        if !description.is_empty() {
            self.description = Some(description);
        }
        self
    }

    /// Sets a web page URL that contains the release changelog.
    #[must_use]
    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// Sets the urgency to install the release.
    #[must_use]
    pub fn urgency(mut self, urgency: ReleaseUrgency) -> Self {
        self.urgency = urgency;
        self
    }

    /// Sets the release date.
    #[must_use]
    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    /// Sets the End-of-life release date.
    #[must_use]
    pub fn date_eol(mut self, date_eol: DateTime<Utc>) -> Self {
        self.date_eol = Some(date_eol);
        self
    }

    /// Sets the release type.
    #[must_use]
    pub fn kind(mut self, kind: ReleaseKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Adds either a download or installed size to the release.
    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.sizes.push(size);
        self
    }

    /// Sets the download & installed sizes of the release.
    #[must_use]
    pub fn sizes(mut self, sizes: Vec<Size>) -> Self {
        self.sizes = sizes;
        self
    }

    /// Adds an artifact to the release.
    #[must_use]
    pub fn artifact(mut self, artifact: Artifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    /// Adds an issue to the release.
    #[must_use]
    pub fn issue(mut self, issue: Issue) -> Self {
        self.issues.push(issue);
        self
    }

    /// Constructs a `Release`.
    #[must_use]
    pub fn build(self) -> Release {
        let kind = self.kind.unwrap_or_default();
        Release {
            version: self.version,
            date: self.date,
            date_eol: self.date_eol,
            kind,
            description: self.description,
            sizes: self.sizes,
            urgency: self.urgency,
            artifacts: self.artifacts,
            url: self.url,
            issues: self.issues,
        }
    }
}
#[derive(Default, Debug)]
/// A helper to build a `Screenshot`
///
/// # Example
/// ```
/// use appstream::{
///     builders::{ImageBuilder, ScreenshotBuilder},
///     TranslatableString,
/// };
/// use url::Url;
///
/// fn main() -> Result<(), url::ParseError> {
///     let screenshot = ScreenshotBuilder::default()
///         .caption(
///             TranslatableString::with_default("FooBar showing kitchen-sink functionality.")
///                 .and_locale("de", "FooBar beim Ausführen der Spühlbecken-Funktion."),
///         )
///         .image(
///             ImageBuilder::new(Url::parse("https://www.example.org/en_US/main.png")?)
///                 .width(800)
///                 .height(600)
///                 .build(),
///         )
///         .build();
///
///     Ok(())
/// }
/// ```
pub struct ScreenshotBuilder {
    /// Whether the screenhot is the default one or not.
    pub is_default: Option<bool>,
    /// A translatable short description of the screenshot.
    pub caption: Option<TranslatableString>,
    /// The various images on that screenshot.
    pub images: Vec<Image>,
    /// The various videos on that screenshot.
    pub videos: Vec<Video>,
}

#[allow(dead_code)]
impl ScreenshotBuilder {
    /// Sets a short translatable description of the `Screenshot`.
    #[must_use]
    pub fn caption(mut self, caption: TranslatableString) -> Self {
        if !caption.is_empty() {
            self.caption = Some(caption);
        }
        self
    }

    /// Sets whether the current screenshot is the default one.
    #[must_use]
    pub fn set_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    /// Adds a new `Image` to the `Screenshot`.
    #[must_use]
    pub fn image(mut self, image: Image) -> Self {
        self.images.push(image);
        self
    }

    /// Sets the list of images corresponding to the screenshot.
    #[must_use]
    pub fn images(mut self, images: Vec<Image>) -> Self {
        self.images = images;
        self
    }

    /// Adds a new `Video` to the `Screenshot`.
    #[must_use]
    pub fn video(mut self, video: Video) -> Self {
        self.videos.push(video);
        self
    }

    /// Sets the list of videos corresponding to the screenshot.
    #[must_use]
    pub fn videos(mut self, videos: Vec<Video>) -> Self {
        self.videos = videos;
        self
    }

    /// Construct a `Screenshot`.
    #[must_use]
    pub fn build(self) -> Screenshot {
        Screenshot {
            caption: self.caption,
            images: self.images,
            videos: self.videos,
            is_default: self.is_default.unwrap_or(true),
        }
    }
}

#[derive(Debug)]
/// A helper to build a `Video`.
///
/// # Example
/// ```
/// use appstream::builders::VideoBuilder;
/// use url::Url;
///
/// fn main() -> Result<(), url::ParseError> {
///     let video = VideoBuilder::new(Url::parse("https://example.com/foobar/screencast.mkv")?)
///         .width(1600)
///         .height(900)
///         .codec("av1")
///         .build();
///
///     Ok(())
/// }
/// ```
pub struct VideoBuilder {
    /// The video width.
    pub width: Option<u32>,
    /// The video height.
    pub height: Option<u32>,
    /// The necesssary codec to play the video.
    pub codec: Option<String>,
    /// The video container. Possible values are Matroska(.mkv) or WebM.
    pub container: Option<String>,
    /// The video URL.
    pub url: Url,
}

#[allow(dead_code)]
impl VideoBuilder {
    ///  Creates a new VideoBuilder
    ///
    /// # Arguments
    ///
    /// * `url` - The video URL.
    pub fn new(url: Url) -> Self {
        Self {
            width: None,
            height: None,
            container: None,
            codec: None,
            url,
        }
    }

    /// Set the video width.
    #[must_use]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the video height.
    #[must_use]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// The video container, either `mkv` or `webm`.
    #[must_use]
    pub fn container(mut self, container: &str) -> Self {
        self.container = Some(container.to_string());
        self
    }

    /// The video codec, either `vp9` or `av1`.
    #[must_use]
    pub fn codec(mut self, codec: &str) -> Self {
        self.codec = Some(codec.to_string());
        self
    }

    /// Construct a Video.
    #[must_use]
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
