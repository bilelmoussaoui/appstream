use serde::{Deserialize, Serialize};

use super::ParseError;
use crate::app_id::AppId;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// A requrirement. See [\<requires\>, \<recommends\>, &
/// \<supports\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-relations)
pub enum Requirement {
    /// A display length requirement.
    DisplayLength(DisplayLength),
    /// Indicates support for a certain kind of input.
    Control(Control),
    /// A requirement relation with another software component.
    AppId(AppId),
    // TODO Add the remaining requirements: hardware, firmware, memory, kernel,
    // and modalias. The Other kind is added so that parsing does not crash.
    #[doc(hidden)]
    Other,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Rel {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Default for Rel {
    fn default() -> Self {
        Self::Ge
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Longest,
    Shortest,
}

impl Default for Side {
    fn default() -> Self {
        Self::Shortest
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// A display length requirement
pub struct DisplayLength {
    #[serde(default)]
    /// Allow depending on a certain minimal version.
    pub compare: Rel,
    /// A relation to the display length defined as an integer value in logical
    /// pixels.
    pub value: DisplayLengthValue,
    #[serde(default)]
    /// Which side is being measure.
    pub side: Side,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "compare", content = "$value", rename_all = "lowercase")]
/// A value for a display length
pub enum DisplayLengthValue {
    /// Very small screens, as used in watches, wearables and other
    /// small-display devices (about <= 360px).
    Xsmall,
    /// Small screens often used in handheld devices, such as phone screens,
    /// small phablets (about < 768px).
    Small,
    /// Screens in laptops, tablets (about >= 768px).
    Medium,
    /// Bigger computer monitors (about >= 1024px).
    Large,
    /// Television screens, large projected images (about >= 3840px).
    Xlarge,
    /// A specific value in pixels.
    Value(u32),
}

impl TryFrom<&str> for DisplayLengthValue {
    type Error = ParseError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "xsmall" => Ok(Self::Xsmall),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "xlarge" => Ok(Self::Xlarge),
            string => {
                if let Ok(value) = string.parse::<u32>() {
                    Ok(Self::Value(value))
                } else {
                    Err(ParseError::invalid_value(
                        string,
                        "$value",
                        "display_length",
                    ))
                }
            }
        }
    }
}

impl TryFrom<&str> for Rel {
    type Error = ParseError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "eq" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "lt" => Ok(Self::Lt),
            "gt" => Ok(Self::Gt),
            "le" => Ok(Self::Le),
            "ge" => Ok(Self::Ge),
            _ => Err(ParseError::invalid_value(
                string,
                "compare",
                "display_length",
            )),
        }
    }
}

impl TryFrom<&str> for Side {
    type Error = ParseError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "shortest" => Ok(Self::Shortest),
            "longest" => Ok(Self::Longest),
            _ => Err(ParseError::invalid_value(string, "side", "display_length")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
/// Indicates support for or require certain ways a user can control the
/// software
pub enum Control {
    /// Input via mouse/cursors/other pointing devices is possible
    Pointing,
    /// Keyboard input is possible
    Keyboard,
    /// Control via a console / command-line interface
    Console,
    /// Graphics tablet input
    Tablet,
    /// Input by touching a surface with fingers is possible
    Touch,
    /// The component supports gamepads (any game controller with
    /// wheels/buttons/joysticks)
    Gamepad,
    /// Input via a TV remote (with arrow keys, number pad, other basic inputs)
    /// is supported.
    TvRemote,
    /// The software can be controlled via voice recognition/activation
    Voice,
    /// The software can be controlled by computer vision / visual object and
    /// sign detection
    Vision,
}

impl TryFrom<&str> for Control {
    type Error = ParseError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "pointing" => Ok(Self::Pointing),
            "keyboard" => Ok(Self::Keyboard),
            "console" => Ok(Self::Console),
            "tablet" => Ok(Self::Tablet),
            "touch" => Ok(Self::Touch),
            "gamepad" => Ok(Self::Gamepad),
            "tv-remove" => Ok(Self::TvRemote),
            "voice" => Ok(Self::Voice),
            "vision" => Ok(Self::Vision),
            _ => Err(ParseError::invalid_value(string, "type", "compare")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, error::Error};

    use super::*;

    #[test]
    fn test_display_length() -> Result<(), Box<dyn Error>> {
        let xml = r"<display_length compare='ge'>360</display_length>";

        let element = xmltree::Element::parse(xml.as_bytes())?;
        let s1 = Requirement::try_from(&element)?;

        let s2 = Requirement::DisplayLength(DisplayLength {
            compare: Rel::Ge,
            value: DisplayLengthValue::Value(360),
            side: Side::Shortest,
        });

        assert_eq!(s1, s2);

        Ok(())
    }

    #[test]
    fn test_empty_display_length() -> Result<(), Box<dyn Error>> {
        let xml = r"<control>console</control>";

        let element = xmltree::Element::parse(xml.as_bytes())?;
        let s1 = Requirement::try_from(&element)?;

        let s2 = Requirement::Control(Control::Console);

        assert_eq!(s1, s2);

        Ok(())
    }

    #[test]
    fn test_small_display_length() -> Result<(), Box<dyn Error>> {
        let xml = r"<display_length compare='eq'>small</display_length>";

        let element = xmltree::Element::parse(xml.as_bytes())?;
        let s1 = Requirement::try_from(&element)?;

        let display_length = DisplayLength {
            compare: Rel::Eq,
            value: DisplayLengthValue::Small,
            side: Side::Shortest,
        };
        let s2 = Requirement::DisplayLength(display_length);

        assert_eq!(s1, s2);

        Ok(())
    }
}
