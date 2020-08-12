use super::enums::{Image, ReleaseKind, ReleaseSize};
use super::Release;
use super::TranslatableString;
use super::{Screenshot, Video};
use chrono::{DateTime, Utc};
use url::Url;

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
