use super::de::*;
use super::enums::ImageKind;
use super::types::TranslatableString;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Serialize, Serializer};
use url::Url;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Screenshot {
    #[serde(
        rename(deserialize = "type", serialize = "default"),
        deserialize_with = "screenshot_type_deserialize",
        default
    )]
    pub is_default: bool,
    #[serde(
        deserialize_with = "some_translatable_deserialize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caption: Option<TranslatableString>,
    #[serde(rename = "image", default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Image>,
    #[serde(rename = "video", default, skip_serializing_if = "Vec::is_empty")]
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

impl Serialize for Screenshot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut screenshot_map = serializer.serialize_map(Some(1))?;
        let mut thumbnails = Vec::new();

        #[derive(Serialize)]
        struct ScreenshotObject {
            url: Url,
            #[serde(skip_serializing_if = "Option::is_none")]
            width: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            height: Option<u32>,
        }

        for image in &self.images {
            match image.kind {
                ImageKind::Thumbnail => {
                    let o = ScreenshotObject {
                        url: image.url.clone(),
                        width: image.width.clone(),
                        height: image.height.clone(),
                    };
                    thumbnails.insert(0, o);
                }
                ImageKind::Source => {
                    let o = ScreenshotObject {
                        url: image.url.clone(),
                        width: image.width.clone(),
                        height: image.height.clone(),
                    };
                    screenshot_map.serialize_entry("source-image", &o)?;
                }
            }
        }

        screenshot_map.serialize_entry("default", &self.is_default)?;
        screenshot_map.serialize_entry("thumbnails", &thumbnails)?;
        screenshot_map.end()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Video {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename(deserialize = "$value", serialize = "url"))]
    pub url: Url,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
    #[serde(rename = "type")]
    pub kind: ImageKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(rename(deserialize = "$value", serialize = "url"))]
    pub url: Url,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::{ImageBuilder, ScreenshotBuilder, VideoBuilder};
    use quick_xml;

    #[test]
    fn default_screenshot() {
        let xml = r"
            <screenshot type='default'>
                <image type='source'>https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png</image>
            </screenshot>";
        let s1: Screenshot = quick_xml::de::from_str(&xml).unwrap();

        let s2 = ScreenshotBuilder::new().image(
                ImageBuilder::new(Url::parse("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png").unwrap())
                .build()
            )
            .build();

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

        let s2 = ScreenshotBuilder::new()
            .caption(
                TranslatableString::with_default("FooBar showing kitchen-sink functionality.")
                    .and_locale("de", "FooBar beim Ausführen der Spühlbecken-Funktion."),
            )
            .image(
                ImageBuilder::new(Url::parse("https://www.example.org/en_US/main.png").unwrap())
                    .width(800)
                    .height(600)
                    .build(),
            )
            .image(
                ImageBuilder::new(
                    Url::parse("https://www.example.org/en_US/main-large.png").unwrap(),
                )
                .width(752)
                .height(423)
                .kind(ImageKind::Thumbnail)
                .build(),
            )
            .image(
                ImageBuilder::new(
                    Url::parse("https://www.example.org/en_US/main-small.png").unwrap(),
                )
                .width(112)
                .height(63)
                .kind(ImageKind::Thumbnail)
                .build(),
            )
            .build();
        assert_eq!(s1, s2);
    }

    #[test]
    fn screenshot_video() {
        let xml = r"
            <screenshot>
                <video codec='av1' width='1600' height='900'>https://example.com/foobar/screencast.mkv</video>
            </screenshot>";
        let s1: Screenshot = quick_xml::de::from_str(&xml).unwrap();

        let s2 = ScreenshotBuilder::new()
            .set_default(false)
            .video(
                VideoBuilder::new(Url::parse("https://example.com/foobar/screencast.mkv").unwrap())
                    .width(1600)
                    .height(900)
                    .codec("av1")
                    .build(),
            )
            .build();
        assert_eq!(s1, s2);
    }
}
