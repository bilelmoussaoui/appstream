use super::enums::ImageKind;
use super::TranslatableString;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
/// Defines a visual representation of the `Component`.
/// See [\<screenshots\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-screenshots).
pub struct Screenshot {
    #[serde(default, alias = "default")]
    /// Whether the current screenshot is the default one.
    pub is_default: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// A translatable small description of the current screenshot.
    pub caption: Option<TranslatableString>,

    #[serde(
        default,
        rename(deserialize = "image", serialize = "images"),
        alias = "images",
        skip_serializing_if = "Vec::is_empty"
    )]
    /// The list of images the current screenshot has.
    /// It contains one image of kind `ImageKind::Source`, the rest are `ImageKind::Thumbnail`
    pub images: Vec<Image>,

    #[serde(rename = "video", default, skip_serializing_if = "Vec::is_empty")]
    /// The list of videos the current screenshot has.
    pub videos: Vec<Video>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// A screenshot video.
/// See [\<screenshots\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-screenshots).
pub struct Video {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The video width.
    pub width: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The video height.
    pub height: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The video codec. Possible values are `vp9` or `av1`.
    pub codec: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The video container. Possible values are Matroska(.mkv) or WebM.
    pub container: Option<String>,

    /// The video url.
    pub url: Url,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// A screenshot image.
/// See [\<screenshots\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-screenshots).
pub struct Image {
    #[serde(rename = "type")]
    /// The image type, either a source or a thumbnail.
    pub kind: ImageKind,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The image width.
    pub width: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The image height.
    pub height: Option<u32>,

    /// The image url.
    pub url: Url,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::{ImageBuilder, ScreenshotBuilder, VideoBuilder};
    use std::convert::TryFrom;
    use std::error::Error;

    #[test]
    fn default_screenshot() -> Result<(), Box<dyn Error>> {
        let xml = r"
            <screenshot type='default'>
                <image type='source'>https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png</image>
            </screenshot>";

        let element = xmltree::Element::parse(xml.as_bytes())?;
        let s1 = Screenshot::try_from(&element)?;

        let s2 = ScreenshotBuilder::default().image(
                ImageBuilder::new(Url::parse("https://raw.githubusercontent.com/PapirusDevelopmentTeam/papirus-icon-theme/master/preview.png")?)
                .build()
            )
            .build();

        assert_eq!(s1, s2);
        Ok(())
    }

    #[test]
    fn screenshot_caption() -> Result<(), Box<dyn Error>> {
        let xml = r"
        <screenshot type='default'>
            <caption>FooBar showing kitchen-sink functionality.</caption>
            <caption xml:lang='de'>FooBar beim Ausführen der Spühlbecken-Funktion.</caption>
            <image type='source' width='800' height='600'>https://www.example.org/en_US/main.png</image>
            <image type='thumbnail' width='752' height='423'>https://www.example.org/en_US/main-large.png</image>
            <image type='thumbnail' width='112' height='63'>https://www.example.org/en_US/main-small.png</image>
        </screenshot>";

        let element = xmltree::Element::parse(xml.as_bytes())?;
        let s1 = Screenshot::try_from(&element)?;

        let s2 = ScreenshotBuilder::default()
            .caption(
                TranslatableString::with_default("FooBar showing kitchen-sink functionality.")
                    .and_locale("de", "FooBar beim Ausführen der Spühlbecken-Funktion."),
            )
            .image(
                ImageBuilder::new(Url::parse("https://www.example.org/en_US/main.png")?)
                    .width(800)
                    .height(600)
                    .build(),
            )
            .image(
                ImageBuilder::new(Url::parse("https://www.example.org/en_US/main-large.png")?)
                    .width(752)
                    .height(423)
                    .kind(ImageKind::Thumbnail)
                    .build(),
            )
            .image(
                ImageBuilder::new(Url::parse("https://www.example.org/en_US/main-small.png")?)
                    .width(112)
                    .height(63)
                    .kind(ImageKind::Thumbnail)
                    .build(),
            )
            .build();
        assert_eq!(s1, s2);
        Ok(())
    }

    #[test]
    fn screenshot_video() -> Result<(), Box<dyn Error>> {
        let xml = r"
            <screenshot>
                <video codec='av1' width='1600' height='900'>https://example.com/foobar/screencast.mkv</video>
            </screenshot>";
        let element = xmltree::Element::parse(xml.as_bytes())?;
        let s1 = Screenshot::try_from(&element)?;

        let s2 = ScreenshotBuilder::default()
            .set_default(false)
            .video(
                VideoBuilder::new(Url::parse("https://example.com/foobar/screencast.mkv")?)
                    .width(1600)
                    .height(900)
                    .codec("av1")
                    .build(),
            )
            .build();
        assert_eq!(s1, s2);
        Ok(())
    }
}
