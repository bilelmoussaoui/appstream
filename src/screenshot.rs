use super::de::some_translatable_deserialize;
use super::translatable_string::TranslatableString;
use serde::de;
use serde::{Deserialize, Serialize};
use url::Url;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Screenshot {
    #[serde(
        rename = "type",
        deserialize_with = "screenshot_type_deserialize",
        default
    )]
    pub is_default: bool,
    #[serde(deserialize_with = "some_translatable_deserialize", default)]
    pub caption: Option<TranslatableString>,
    #[serde(
        rename = "image",
        deserialize_with = "screenshot_image_deserialize",
        default
    )]
    pub images: Vec<Image>,
    #[serde(rename = "video", default)]
    pub videos: Vec<Video>,
}

impl Default for Screenshot {
    fn default() -> Self {
        Self {
            is_default: true,
            caption: None,
            images: vec![],
            videos: vec![],
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
    pub fn new() -> ScreenshotBuilder {
        ScreenshotBuilder {
            is_default: None,
            caption: None,
            videos: vec![],
            images: vec![],
        }
    }

    pub fn caption(mut self, caption: TranslatableString) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    pub fn images(mut self, images: Vec<Image>) -> Self {
        self.images = images;
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

fn screenshot_image_deserialize<'de, D>(deserializer: D) -> Result<Vec<Image>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PImage {
        #[serde(rename = "type", default)]
        pub _type: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
        #[serde(rename = "$value")]
        url: Url,
    };

    let pimages: Vec<PImage> = Vec::deserialize(deserializer)?;
    Ok(pimages
        .into_iter()
        .map(
            |pi| match pi._type.unwrap_or_else(|| "source".to_string()).as_ref() {
                "thumbnail" => Image::Thumbnail {
                    url: pi.url,
                    width: pi.width.expect("screenshots thumbnails must have a width"),
                    height: pi
                        .height
                        .expect("screenshots thumbnails must have a height"),
                },
                _ => Image::Source {
                    url: pi.url,
                    width: pi.width,
                    height: pi.height,
                },
            },
        )
        .collect::<Vec<Image>>())
}

fn screenshot_type_deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(s == "default")
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Image {
    Source {
        url: Url,
        width: Option<u32>,
        height: Option<u32>,
    },
    Thumbnail {
        url: Url,
        width: u32,
        height: u32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Video {
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub codec: Option<String>,
    #[serde(default)]
    pub container: Option<String>,
    #[serde(rename = "$value")]
    pub url: Url,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml;
    use std::str::FromStr;

    #[test]
    fn default_screenshot() {
        let xml = r"
            <screenshot type='default'>
                <image type='source'>https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png</image>
            </screenshot>";
        let s1: Screenshot = quick_xml::de::from_str(&xml).unwrap();

        let s2 = ScreenshotBuilder::new().images(vec![Image::Source{
            url: Url::from_str("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png").unwrap(),
            width: None,
            height: None
        }]).build();

        assert_eq!(s1, s2);
    }

    #[test]
    fn name() {
        let xml = r"
        <screenshot type='default'>
            <caption>FooBar showing kitchen-sink functionality.</caption>
            <caption xml:lang='de'>FooBar beim Ausführen der Spühlbecken-Funktion.</caption>
            <image type='source' width='800' height='600'>https://www.example.org/en_US/main.png</image>
            <image type='thumbnail' width='752' height='423'>https://www.example.org/en_US/main-large.png</image>
            <image type='thumbnail' width='112' height='63'>https://www.example.org/en_US/main-small.png</image>
        </screenshot>";
        let s1: Screenshot = quick_xml::de::from_str(&xml).unwrap();


        let mut caption =
            TranslatableString::with_default("FooBar showing kitchen-sink functionality.");
        caption.add_for_locale(
            Some("de"),
            "FooBar beim Ausführen der Spühlbecken-Funktion.",
        );
        
        let s2 = ScreenshotBuilder::new().caption(caption).images(
            vec![
                Image::Source {
                    url: Url::from_str("https://www.example.org/en_US/main.png").unwrap(),
                    width: Some(800),
                    height: Some(600)
                },
                Image::Thumbnail {
                    url: Url::from_str("https://www.example.org/en_US/main-large.png").unwrap(),
                    width: 752,
                    height: 423
                },
                Image::Thumbnail {
                    url: Url::from_str("https://www.example.org/en_US/main-small.png").unwrap(),
                    width: 112,
                    height: 63
                }
            ]
        ).build();
        assert_eq!(s1, s2);
    }

    #[test]
    fn screenshot_video() {
        let xml = r"
            <screenshot>
                <video codec='av1' width='1600' height='900'>https://example.com/foobar/screencast.mkv</video>
            </screenshot>";
        let s: Screenshot = quick_xml::de::from_str(&xml).unwrap();

        assert_eq!(s.is_default, false);

        assert_eq!(
            s.videos,
            vec![Video {
                url: Url::from_str("https://example.com/foobar/screencast.mkv").unwrap(),
                width: Some(1600),
                height: Some(900),
                codec: Some("av1".to_string()),
                container: None,
            },]
        );
    }
}
