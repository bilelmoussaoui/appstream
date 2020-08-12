use super::de::*;
use super::enums::Image;
use super::TranslatableString;
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
    use crate::builders::{ScreenshotBuilder, VideoBuilder};
    use quick_xml;
    use std::str::FromStr;

    #[test]
    fn default_screenshot() {
        let xml = r"
            <screenshot type='default'>
                <image type='source'>https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png</image>
            </screenshot>";
        let s1: Screenshot = quick_xml::de::from_str(&xml).unwrap();

        let s2 = ScreenshotBuilder::new().image(Image::Source{
            url: Url::from_str("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png").unwrap(),
            width: None,
            height: None
        }).build();

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

        let s2 = ScreenshotBuilder::new()
            .caption(caption)
            .images(vec![
                Image::Source {
                    url: Url::from_str("https://www.example.org/en_US/main.png").unwrap(),
                    width: Some(800),
                    height: Some(600),
                },
                Image::Thumbnail {
                    url: Url::from_str("https://www.example.org/en_US/main-large.png").unwrap(),
                    width: 752,
                    height: 423,
                },
                Image::Thumbnail {
                    url: Url::from_str("https://www.example.org/en_US/main-small.png").unwrap(),
                    width: 112,
                    height: 63,
                },
            ])
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
                VideoBuilder::new(
                    Url::from_str("https://example.com/foobar/screencast.mkv").unwrap(),
                )
                .width(1600)
                .height(900)
                .codec("av1")
                .build(),
            )
            .build();
        assert_eq!(s1, s2);
    }
}
