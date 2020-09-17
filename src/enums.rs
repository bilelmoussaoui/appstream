use super::error::ParseError;
use super::AppId;
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize, Serialize, Serializer,
};
use std::cmp::{Ord, Ordering};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString, ToString};
use url::Url;

#[derive(Clone, Copy, Debug, AsRefStr, EnumString, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
/// The artifact type.
pub enum ArtifactKind {
    /// The artifact is distributed as source-code.
    Source,
    /// The artifact is distributed as binary.
    Binary,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
#[non_exhaustive]
/// Indicates that the software is available via a 3rd-party application installer.
/// See [\<bundle\/\>](https://www.freedesktop.org/software/appstream/docs/chap-CollectionData.html#tag-ct-bundle).
pub enum Bundle {
    /// A [Limba](https://people.freedesktop.org/~mak/limba/) bundle.
    Limba(String),
    /// A [Flatpak](https://flatpak.org/) bundle.
    Flatpak {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// The required runtime to run the application.
        runtime: Option<String>,
        /// The SDK used to build the application.
        sdk: String,
        /// The application reference.
        reference: String,
    },
    /// An [AppImage](https://appimage.org/) bundle.
    AppImage(String),
    /// A [Snap](https://snapcraft.io/) bundle.
    Snap(String),
    /// Plain and possibly compressed tarballs.
    Tarball(String),
}

impl Serialize for Bundle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bundle_map = serializer.serialize_map(None)?;

        match self {
            Bundle::Limba(id) => {
                bundle_map.serialize_entry("type", "limba")?;
                bundle_map.serialize_entry("id", id)?;
            }
            Bundle::Flatpak {
                runtime,
                sdk,
                reference,
            } => {
                bundle_map.serialize_entry("type", "flatpak")?;
                bundle_map.serialize_entry("reference", reference)?;
                bundle_map.serialize_entry("sdk", sdk)?;
                if runtime.is_some() {
                    bundle_map.serialize_entry("runtime", runtime.as_ref().unwrap())?;
                }
            }
            Bundle::AppImage(id) => {
                bundle_map.serialize_entry("type", "appimage")?;
                bundle_map.serialize_entry("id", id)?;
            }
            Bundle::Snap(id) => {
                bundle_map.serialize_entry("type", "snap")?;
                bundle_map.serialize_entry("id", id)?;
            }
            Bundle::Tarball(id) => {
                bundle_map.serialize_entry("type", "tarball")?;
                bundle_map.serialize_entry("id", id)?;
            }
        }

        bundle_map.end()
    }
}

#[derive(Clone, Debug, AsRefStr, EnumString, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[strum(serialize_all = "PascalCase")]
#[non_exhaustive]
/// Specifies a number of defined categories a component can be assigned to.
/// See the list of the [Registered Categories](https://specifications.freedesktop.org/menu-spec/latest/apa.html#main-category-registry).
pub enum Category {
    // Main categories
    // https://specifications.freedesktop.org/menu-spec/latest/apa.html#main-category-registry
    /// Application for presenting, creating, or processing multimedia (audio/video).
    AudioVideo,
    /// An audio application.
    Audio,
    /// A video application.
    Video,
    /// An application for development.
    Development,
    /// Educational software.
    Education,
    /// A game.
    Game,
    /// Application for viewing, creating, or processing graphics.
    Graphics,
    /// Network application such as a web browser.
    Network,
    /// An office type application.
    Office,
    /// Scientific software.
    Science,
    /// Settings applications.
    Settings,
    /// System application, "System Tools" such as say a log viewer or network monitor.
    System,
    /// Small utility application, "Accessories".
    Utility,
    // Additional categories
    // https://specifications.freedesktop.org/menu-spec/latest/apas02.html
    /// A tool to build applications.
    Building,
    /// A tool to debug applications.
    Debugger,
    /// IDE application.
    IDE,
    /// A GUI designer application.
    GUIDesigner,
    /// A profiling tool.
    Profiling,
    /// Applications like cvs or subversion.
    RevisionControl,
    /// A translation tool.
    Translation,
    /// Calendar application.
    Calendar,
    /// E.g. an address book.
    ContactManagement,
    /// Application to manage a database.
    Database,
    /// A dictionary.
    Dictionary,
    /// Chart application.
    Chart,
    /// Email application.
    Email,
    /// Application to manage your finance.
    Finance,
    /// A flowchart application.
    FlowChart,
    /// Tool to manage your PDA.
    PDA,
    /// Project management application.
    ProjectManagement,
    /// Presentation software.
    Presentation,
    /// A spreadsheet.
    Spreadsheet,
    /// A word processor.
    WordProcessor,
    /// 2D based graphical application.
    TwoDGraphics,
    /// Application for viewing, creating, or processing vector graphics.
    VectorGraphics,
    /// Application for viewing, creating, or processing raster (bitmap) graphics.
    RasterGraphics,
    /// Application for viewing, creating, or processing 3-D graphics.
    ThreeDGraphics,
    /// Tool to scan a file/text.
    Scanning,
    /// Optical character recognition application.
    OCR,
    /// Camera tools, etc.
    Photography,
    /// Desktop Publishing applications and Color Management tools.
    Publishing,
    /// Tool to view e.g. a graphic or pdf file.
    Viewer,
    /// A text tool utility.
    TextTools,
    /// Configuration tool for the GUI.
    DesktopSettings,
    /// A tool to manage hardware components, like sound cards, video cards or printers.
    HardwareSettings,
    /// A tool to manage printers.
    Printing,
    /// A package manager application.
    PackageManager,
    /// A dial-up program.
    Dialup,
    /// An instant messaging client.
    InstantMessaging,
    /// A chat client
    Chat,
    /// An IRC client.
    IRCClient,
    /// RSS, podcast and other subscription based contents.
    Feed,
    /// Tools like FTP or P2P programs.
    FileTransfer,
    /// HAM radio software.
    HamRadio,
    /// A news reader or a news ticker.
    News,
    /// A P2P program.
    P2P,
    /// A tool to remotely manage your PC.
    RemoteAccess,
    /// Telephony via PC.
    Telephony,
    /// Telephony tools, to dial a number, manage PBX, ...
    TelephonyTools,
    /// Video Conference software.
    VideoConference,
    /// A web browser.
    WebBrowser,
    /// A tool for web developers
    WebDevelopment,
    /// An app related to MIDI.
    Midi,
    /// Just a mixer.
    Mixer,
    /// A sequencer.
    Sequencer,
    /// A tuner.
    Tuner,
    /// A TV application.
    TV,
    /// Application to edit audio/video files.
    AudioVideoEditing,
    /// Application to play audio/video files.
    Player,
    /// Application to record audio/video files.
    Recorder,
    /// Application to burn a disc.
    DiscBurning,
    /// An action game.
    ActionGame,
    /// Adventure style game.
    AdventureGame,
    /// Arcade style game.
    ArcadeGame,
    /// A board game.
    BoardGame,
    /// Falling blocks game.
    BlocksGame,
    /// A card game.
    CardGame,
    /// A game for kids.
    KidsGame,
    /// Logic games like puzzles, etc.
    LogicGame,
    /// A role playing game.
    RolePlaying,
    /// A shooter game.
    Shooter,
    /// A simulation game.
    Simulation,
    /// A sports game.
    SportsGame,
    /// A strategy game.
    StrategyGame,
    /// Software to teach arts.
    Art,
    ///
    Construction,
    /// Musical software.
    Music,
    /// Software to learn foreign languages.
    Languages,
    /// Artificial Intelligence software.
    ArtificialIntelligence,
    /// Astronomy software.
    Astronomy,
    /// Biology software.
    Biology,
    /// Chemistry software.
    Chemistry,
    /// Computer sience software.
    ComputerScience,
    /// Data visualization software.
    DataVisualization,
    /// Economy software.
    Economy,
    /// Electricity software.
    Electricity,
    /// Geography software.
    Geography,
    /// Geology software.
    Geology,
    /// Geoscience software, GIS.
    Geoscience,
    /// History software.
    History,
    /// Software for philosophy, psychology and other humanities.
    Humanities,
    /// Image Processing software.
    ImageProcessing,
    /// Literature software.
    Literature,
    /// Software for viewing maps, navigation, mapping, GPS.
    Maps,
    /// Math software.
    Math,
    /// Numerical analysis software.
    NumericalAnalysis,
    /// Medical software.
    MedicalSoftware,
    /// Physics software.
    Physics,
    /// Robotics software.
    Robotics,
    /// Religious and spiritual software, theology.
    Spirituality,
    /// Sports software.
    Sports,
    /// Parallel computing software.
    ParallelComputing,
    /// A simple amusement.
    Amusement,
    /// A tool to archive/backup data.
    Archiving,
    /// A tool to manage compressed data/archives.
    Compression,
    /// Electronics software, e.g. a circuit designer.
    Electronics,
    /// Emulator of another platform, such as a DOS emulator.
    Emulator,
    /// Engineering software, e.g. CAD programs.
    Engineering,
    /// A file tool utility.
    FileTools,
    /// A file manager.
    FileManager,
    /// A terminal emulator application.
    TerminalEmulator,
    /// A file system tool.
    Filesystem,
    /// Monitor application/applet that monitors some resource or activity.
    Monitor,
    /// A security tool.
    Security,
    /// Accessibility.
    Accessibility,
    /// A calculator.
    Calculator,
    /// A clock application/applet.
    Clock,
    /// A text editor.
    TextEditor,
    /// Help or documentation.
    Documentation,
    /// Application handles adult or explicit material.
    Adult,
    /// Important application, core to the desktop such as a file manager or a help browser.
    Core,
    /// Application based on KDE libraries.
    KDE,
    /// Application based on GNOME libraries.
    GNOME,
    /// Application based on XFCE libraries.
    XFCE,
    /// Application based on GTK+ libraries.
    GTK,
    /// Application based on Qt libraries.
    Qt,
    /// Application based on Motif libraries.
    Motif,
    /// Application based on Java GUI libraries, such as AWT or Swing.
    Java,
    /// Application that only works inside a terminal (text-based or command line application).
    ConsoleOnly,
    // Reserved categories
    // https://specifications.freedesktop.org/menu-spec/latest/apas03.html
    /// A screen saver.
    Screensaver,
    /// An application that is primarily an icon for the "system tray" or "notification area".
    TrayIcon,
    /// An applet that will run inside a panel or another such application, likely desktop specific.
    Applet,
    /// A shell (an actual specific shell such as bash or tcsh, not a TerminalEmulator).
    Shell,
    #[strum(default)]
    #[doc(hidden)]
    Unknown(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "$value")]
#[non_exhaustive]
/// Defines a checksum to validate the integrity of an artifact.
pub enum Checksum {
    /// A checksum computed using `sha1`.
    Sha1(String),
    /// A checksum computed using `sha256`.
    Sha256(String),
    /// A checksum computed using `blake2b`.
    Blake2b(String),
    /// A checksum computed using `blake2s`.
    Blake2s(String),
}

#[derive(Clone, Copy, Debug, AsRefStr, Serialize, ToString, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[non_exhaustive]
/// Defines the various types of a `Component`.
pub enum ComponentKind {
    /// A runtime.
    Runtime,
    #[serde(alias = "console")]
    #[strum(serialize = "console")]
    /// A terminal application (CLI).
    ConsoleApplication,
    #[serde(alias = "desktop")]
    #[strum(serialize = "desktop")]
    /// A desktop application.
    DesktopApplication,
    #[serde(alias = "webapp")]
    #[strum(serialize = "web-application")]
    /// A web application.
    WebApplication,
    #[serde(rename = "inputmethod")]
    #[strum(serialize = "inputmethod")]
    /// An input method.
    InputMethod,
    #[serde(alias = "operating-system")]
    #[strum(serialize = "operating-system")]
    /// An operating system.
    OS,
    /// A theme.
    Theme,
    /// A firmware.
    Firmware,
    /// An addon of another `Component`.
    Addon,
    /// A font.
    Font,
    /// A generic component.
    Generic,
    /// An icon theme.
    IconTheme,
    /// A localization package.
    Localization,
    /// A driver.
    Driver,
    /// A codec.
    Codec,
}

impl Default for ComponentKind {
    fn default() -> Self {
        ComponentKind::Generic
    }
}

impl FromStr for ComponentKind {
    type Err = ParseError;

    fn from_str(c: &str) -> Result<Self, Self::Err> {
        match c {
            "runtime" => Ok(ComponentKind::Runtime),
            "console" | "console-application" => Ok(ComponentKind::ConsoleApplication),
            "desktop" | "desktop-application" => Ok(ComponentKind::DesktopApplication),
            "webapp" => Ok(ComponentKind::WebApplication),
            "inputmethod" => Ok(ComponentKind::InputMethod),
            "operating-system" => Ok(ComponentKind::OS),
            "theme" => Ok(ComponentKind::Theme),
            "firmware" => Ok(ComponentKind::Firmware),
            "addon" => Ok(ComponentKind::Addon),
            "font" => Ok(ComponentKind::Font),
            "icontheme" | "icon-theme" => Ok(ComponentKind::IconTheme),
            "driver" => Ok(ComponentKind::Driver),
            "codec" => Ok(ComponentKind::Codec),
            "localization" => Ok(ComponentKind::Localization),
            "" | "generic" => Ok(ComponentKind::default()),
            _ => Err(ParseError::InvalidValue(
                c.to_string(),
                "type".to_string(),
                "component".to_string(),
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "$value")]
#[non_exhaustive]
/// OARS attribute.
pub enum ContentAttribute {
    #[serde(rename = "violence-cartoon")]
    /// Defined as fictional characters depicted in an animated film or a comic strip which do not look human.
    ViolenceCartoon(ContentState),
    #[serde(rename = "violence-fantasy")]
    /// Defined as characters easily distinguishable from reality.
    ViolenceFantasy(ContentState),
    #[serde(rename = "violence-realistic")]
    /// Defined as characters not easily distinguishable from reality.
    ViolenceRealistic(ContentState),
    #[serde(rename = "violence-bloodshed")]
    /// Defined as the killing or wounding of people.
    ViolenceBloodshed(ContentState),
    #[serde(rename = "violence-sexual")]
    /// Defined as any unwanted sexual act or activity.
    ViolenceSexual(ContentState),
    #[serde(rename = "violence-desecration")]
    /// Defined as the action of desecrating something, typically a human body.
    ViolenceDesecration(ContentState),
    #[serde(rename = "violence-slavery")]
    /// Defined as working without proper remuneration or appreciation.
    ViolenceSlavery(ContentState),
    #[serde(rename = "violence-worship")]
    /// Defined as violence targeted to places of worship.
    ViolenceWorship(ContentState),
    #[serde(rename = "drugs-alcohol")]
    /// Defined as usage of alcohol or seeing a character consumes one.
    DrugsAlcohol(ContentState),
    #[serde(rename = "drugs-narcotics")]
    /// Defined as an addictive drug affecting mood or behaviour that is specifically illegal in at least one country.
    DrugsNarcotics(ContentState),
    #[serde(rename = "drugs-tobacco")]
    /// Defined as any nicotine-rich product.
    DrugsTobacco(ContentState),
    #[serde(rename = "sex-nudity")]
    /// Defined as a state of undress, and in this case specifically specifically nudity likely to cause offense.
    SexNudity(ContentState),
    #[serde(rename = "sex-themes")]
    /// Defined as in reference to a sexual act.
    SexThemes(ContentState),
    #[serde(rename = "sex-homosexuality")]
    /// Defined as sexual attraction to people of one's own sex.
    SexHomosexuality(ContentState),
    #[serde(rename = "sex-prostitution")]
    /// Defined as the practice or occupation of engaging in sexual activity with someone for payment.
    SexProstitution(ContentState),
    #[serde(rename = "sex-adultery")]
    /// Defined as voluntary interaction between a married person and a person who is not their spouse.
    SexAdultery(ContentState),
    #[serde(rename = "sex-appearance")]
    /// Defined as appearance of human or human-like characters that are sexualized in some way.
    SexAppearance(ContentState),
    #[serde(rename = "language-profanity")]
    /// Defined as blasphemous or obscene language.
    LanguageProfanity(ContentState),
    #[serde(rename = "language-humor")]
    /// Defined as the quality of being amusing.
    LanguageHumor(ContentState),
    #[serde(rename = "language-discrimination")]
    /// Defined as the unjust or prejudicial treatment of different categories of people, especially on the grounds of race, age, or sex.
    LanguageDiscrimination(ContentState),
    #[serde(rename = "social-chat")]
    /// Defined as any messaging system connected to the Internet.
    SocialChat(ContentState),
    #[serde(rename = "social-info")]
    /// Defined as sharing information with a legal entity typically used for advertising or for sending back diagnostic data.
    SocialInfo(ContentState),
    #[serde(rename = "social-audio")]
    /// Defined as any multimedia messaging system connected to the Internet.
    SocialAudio(ContentState),
    #[serde(rename = "social-location")]
    /// Defined as sharing your physical real-time location.
    SocialLocation(ContentState),
    #[serde(rename = "social-contacts")]
    /// Defined as sharing identifiable details with other users to allow out-of-band communication.
    SocialContacts(ContentState),
    #[serde(rename = "money-advertising")]
    /// Defined as the activity of producing advertisements for commercial products or services.
    MoneyAdvertising(ContentState),
    #[serde(rename = "money-purchasing")]
    /// Defined as items or points that a user can buy for use within a virtual world to improve a character or enhance the playing experience.
    MoneyPurchasing(ContentState),
    #[serde(rename = "money-gambling")]
    /// Defined as taking a risky action in the hope of a desired result.
    MoneyGambling(ContentState),
}

#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize, Debug)]
#[non_exhaustive]
/// Defines the version of the OARS specification.
pub enum ContentRatingVersion {
    #[serde(rename = "oars-1.0")]
    /// OARS v1.0.
    Oars1_0,
    #[serde(rename = "oars-1.1")]
    /// OARS v1.1.
    Oars1_1,
    /// Unknown version
    Unknown,
}

impl Default for ContentRatingVersion {
    fn default() -> Self {
        ContentRatingVersion::Unknown
    }
}

impl Ord for ContentRatingVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ContentRatingVersion::Oars1_0, ContentRatingVersion::Oars1_1)
            | (ContentRatingVersion::Unknown, _) => Ordering::Less,
            (ContentRatingVersion::Oars1_1, ContentRatingVersion::Oars1_0)
            | (_, ContentRatingVersion::Unknown) => Ordering::Greater,
            (ContentRatingVersion::Oars1_0, ContentRatingVersion::Oars1_0)
            | (ContentRatingVersion::Oars1_1, ContentRatingVersion::Oars1_1) => Ordering::Equal,
        }
    }
}

impl PartialOrd for ContentRatingVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, AsRefStr, ToString, EnumString, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Defines the state of a `ContentAttribute`
pub enum ContentState {
    /// No state is set.
    None,
    /// Mild state.
    Mild,
    /// Moderate state.
    Moderate,
    /// Intense state.
    Intense,
}

impl Default for ContentState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, AsRefStr, EnumString, ToString, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Defines the firmware type.
pub enum FirmwareKind {
    /// A flashed firmware.
    Flashed,
    /// A runtime firmware.
    Runtime,
}

#[derive(Clone, Debug, PartialEq)]
/// Defines a component icon.
/// See [\<icon\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-icon).
pub enum Icon {
    /// Icon loaded from the stock.
    Stock(String),
    /// Icon cached.
    Cached {
        /// The icon path.
        path: PathBuf,
        /// The icon width.
        width: Option<u32>,
        /// The icon height.
        height: Option<u32>,
    },
    /// Icon loaded from a remote URL.
    Remote {
        /// The icon URL.
        url: Url,
        /// The icon width.
        width: Option<u32>,
        /// The icon height.
        height: Option<u32>,
    },
    /// Icon loaded from a file.
    Local {
        /// The icon path.
        path: PathBuf,
        /// The icon width.
        width: Option<u32>,
        /// The icon height.
        height: Option<u32>,
    },
}

impl<'de> Deserialize<'de> for Icon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IconVisitor;

        impl<'de> Visitor<'de> for IconVisitor {
            type Value = Icon;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map representing an icon")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut kind = None;
                let mut width = None;
                let mut height = None;
                let mut path = None;

                while let Some((key, value)) = access.next_entry::<String, String>()? {
                    match &*key {
                        "type" => {
                            kind = Some(value.clone());
                        }
                        "width" => {
                            width = value.parse::<u32>().ok();
                        }
                        "height" => {
                            height = value.parse::<u32>().ok();
                        }
                        "path" | "name" | "url" => {
                            path = Some(value.clone());
                        }
                        _ => (),
                    }
                }

                let path = path.ok_or_else(|| de::Error::missing_field("path"))?;
                let kind = kind.ok_or_else(|| de::Error::missing_field("type"))?;

                match kind.as_ref() {
                    "remote" => Ok(Icon::Remote {
                        url: Url::parse(&path).map_err(|_| {
                            de::Error::invalid_value(
                                de::Unexpected::Str(&path),
                                &"expected a valid url",
                            )
                        })?,
                        width,
                        height,
                    }),
                    "stock" => Ok(Icon::Stock(path)),
                    "cached" => Ok(Icon::Cached {
                        path: path.into(),
                        width,
                        height,
                    }),
                    "local" => Ok(Icon::Local {
                        path: path.into(),
                        width,
                        height,
                    }),
                    e => Err(de::Error::invalid_value(
                        de::Unexpected::Str(e),
                        &"expected a type of local, remote, cached or stock",
                    )),
                }
            }
        }
        deserializer.deserialize_map(IconVisitor)
    }
}

impl Serialize for Icon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Icon::Stock(path) => {
                let mut s = serializer.serialize_struct("icon", 2)?;
                s.serialize_field("type", "stock")?;
                s.serialize_field("name", &path)?;
                s.end()
            }
            Icon::Remote { url, width, height } => {
                let mut s = serializer.serialize_struct("icon", 4)?;
                s.serialize_field("type", "remote")?;
                s.serialize_field("url", &url)?;
                s.serialize_field("width", &width)?;
                s.serialize_field("height", &height)?;
                s.end()
            }
            Icon::Cached {
                path,
                width,
                height,
            } => {
                let mut s = serializer.serialize_struct("icon", 4)?;
                s.serialize_field("type", "cached")?;
                s.serialize_field("path", &path)?;
                s.serialize_field("width", &width)?;
                s.serialize_field("height", &height)?;
                s.end()
            }
            Icon::Local {
                path,
                width,
                height,
            } => {
                let mut s = serializer.serialize_struct("icon", 4)?;
                s.serialize_field("type", "local")?;
                s.serialize_field("path", &path)?;
                s.serialize_field("width", &width)?;
                s.serialize_field("height", &height)?;
                s.end()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, ToString, AsRefStr, Serialize, Deserialize, PartialEq, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// The type of an image.
pub enum ImageKind {
    /// The source image.
    Source,
    /// A thumbnail image.
    Thumbnail,
}

impl Default for ImageKind {
    fn default() -> Self {
        Self::Source
    }
}

#[derive(Clone, Debug, Deserialize, AsRefStr, ToString, Serialize, PartialEq, EnumString)]
#[strum(serialize_all = "PascalCase")]
#[non_exhaustive]
/// Defines some metrics of awesomeness.
/// See [the specs](https://gitlab.gnome.org/GNOME/gnome-software/blob/master/doc/kudos.md) for more information.
pub enum Kudo {
    /// The application uses an AppMenu.
    /// Shouldn't be used anymore.
    #[doc(hidden)]
    AppMenu,
    /// The application installs a 128px icon or larger.
    HiDpiIcon,
    /// The application uses a high contrast icons for visually impaired users.
    HighContrast,
    /// Uses a modern toolkit like GTK 3 or Qt 5.
    ModernToolkit,
    /// Registers for sending desktop notifications.
    Notifications,
    /// Provides a search provider for GNOME Shell or KDE Plasma.
    SearchProvider,
    /// Provides user documentation/help.
    UserDocs,
    #[strum(default)]
    #[doc(hidden)]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "name")]
#[non_exhaustive]
/// Indicates possible methods to launch the application.
/// See [\<launchable\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-launchable).
pub enum Launchable {
    #[serde(alias = "desktop_id")]
    /// The application can be launched via a desktop file.
    /// See [Desktop File ID](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#desktop-file-id).
    DesktopId(String),
    /// The software can be started, stopped and monitored by the OS "init" such as systemd.
    Service(String),
    /// The application is a website viewed through a browser.
    Url(Url),
    /// The software can be launched from the menus of the [Cockpit](http://cockpit-project.org/) admin interface.
    CockpitManifest(String),
    #[doc(hidden)]
    Unknown(String),
}

impl Serialize for Launchable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("launchable", 2)?;
        match self {
            Launchable::DesktopId(app_id) => {
                s.serialize_field("type", "desktop_id")?;
                s.serialize_field("name", &app_id)?;
            }
            Launchable::Service(name) => {
                s.serialize_field("type", "service")?;
                s.serialize_field("name", &name)?;
            }
            Launchable::Url(url) => {
                s.serialize_field("type", "url")?;
                s.serialize_field("name", &url)?;
            }
            Launchable::CockpitManifest(manifest) => {
                s.serialize_field("type", "cockpit_manifest")?;
                s.serialize_field("name", &manifest)?;
            }
            Launchable::Unknown(name) => {
                s.serialize_field("type", "unknown")?;
                s.serialize_field("name", &name)?;
            }
        }
        s.end()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "url")]
#[non_exhaustive]
/// Defines a list of possible project URLs.
/// See [\<url\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-url).
pub enum ProjectUrl {
    /// Web page with information on how to donate.
    Donation(Url),
    /// To submit or modify translations.
    Translate(Url),
    /// Upstream homepage.
    Homepage(Url),
    /// Bug tracking system, to report new bugs.
    BugTracker(Url),
    /// An online user's reference.
    Help(Url),
    /// Web page with answers to frequently asked questions.
    Faq(Url),
    /// Web page that allows the user to contact the developer.
    Contact(Url),
    #[doc(hidden)]
    Unknown(Url),
}

impl Serialize for ProjectUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("url", 2)?;
        match self {
            ProjectUrl::Donation(url) => {
                s.serialize_field("type", "donation")?;
                s.serialize_field("url", &url)?;
            }
            ProjectUrl::Translate(url) => {
                s.serialize_field("type", "translate")?;
                s.serialize_field("url", &url)?;
            }
            ProjectUrl::Homepage(url) => {
                s.serialize_field("type", "homepage")?;
                s.serialize_field("url", &url)?;
            }
            ProjectUrl::BugTracker(url) => {
                s.serialize_field("type", "bugtracker")?;
                s.serialize_field("url", &url)?;
            }
            ProjectUrl::Help(url) => {
                s.serialize_field("type", "help")?;
                s.serialize_field("url", &url)?;
            }
            ProjectUrl::Faq(url) => {
                s.serialize_field("type", "faq")?;
                s.serialize_field("url", &url)?;
            }
            ProjectUrl::Contact(url) => {
                s.serialize_field("type", "contact")?;
                s.serialize_field("url", &url)?;
            }
            ProjectUrl::Unknown(url) => {
                s.serialize_field("type", "unknown")?;
                s.serialize_field("url", &url)?;
            }
        }
        s.end()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
/// Describes the public interfaces the component provides.
/// See [\<provide\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-provides).
#[non_exhaustive]
pub enum Provide {
    /// Shared library.
    Library(PathBuf),
    /// Name of a binary installed in `$PATH`.
    Binary(String),
    /// Full name of a font.
    Font(String),
    /// A modalias glob representing the hardware types the component handles.
    Modalias(String),
    /// Information needed to associate a firmware with a device.
    Firmware {
        #[serde(rename = "type")]
        /// The firmware type.
        kind: FirmwareKind,
        /// Value
        item: String,
    },
    /// Name of a Python 2 module.
    Python2(String),
    /// Name of a Python 2 module.
    Python3(String),
    /// FIXME: support dbus session type
    DBus(String),
    /// Useful when the component-id had to be renamed.
    Id(AppId),
    /// Required only for Codec components.
    Codec(String),
}

#[derive(Clone, Copy, Debug, ToString, EnumString, AsRefStr, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Classifies the release into stable/development.
/// See [\<releases\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-releases).
pub enum ReleaseKind {
    /// A stable release.
    Stable,
    /// A development release, not intended to be installed by users.
    Development,
}

impl Default for ReleaseKind {
    fn default() -> Self {
        ReleaseKind::Stable
    }
}

#[derive(Clone, Copy, Debug, AsRefStr, EnumString, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Defines how important is to install the new release as un update.
/// See [\<releases\/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-releases).
pub enum ReleaseUrgency {
    /// Low urgency.
    Low,
    /// Medium urgency.
    Medium,
    /// High urgency.
    High,
    /// Critical urgency.
    Critical,
}

impl Default for ReleaseUrgency {
    fn default() -> Self {
        ReleaseUrgency::Medium
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "$value", rename_all = "kebab-case")]
#[non_exhaustive]
/// Defines the download and installed size of a `Component` or `Artifact`.
pub enum Size {
    /// The downloaded size is bytes.
    Download(u64),
    /// The installed size in bytes.
    Installed(u64),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type", content = "name")]
#[non_exhaustive]
/// Defines the possible translation domains.
/// See [\<translation/\>](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-translation).
pub enum Translation {
    /// The component uses gettext for translations.
    Gettext(String),
    /// The component uses Qt for translations.
    Qt(String),
    #[doc(hidden)]
    Unknown,
}
