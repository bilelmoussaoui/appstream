use super::types::AppId;
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Serialize, Serializer};
use std::cmp::{Ord, Ordering};
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString, ToString};
use url::Url;

#[derive(Clone, Debug, AsRefStr, EnumString, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ArtifactKind {
    Source,
    Binary,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Bundle {
    Limba(String),
    Flatpak {
        #[serde(skip_serializing_if = "Option::is_none")]
        runtime: Option<String>,
        sdk: String,
        reference: String,
    },
    AppImage(String),
    Snap(String),
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
pub enum Category {
    // Main categories
    // https://specifications.freedesktop.org/menu-spec/latest/apa.html#main-category-registry
    AudioVideo,
    Audio,
    Video,
    Development,
    Education,
    Game,
    Graphics,
    Network,
    Office,
    Science,
    Settings,
    System,
    Utility,
    // Additional categories
    // https://specifications.freedesktop.org/menu-spec/latest/apas02.html
    Building,
    Debugger,
    IDE,
    GUIDesigner,
    Profiling,
    RevisionControl,
    Translation,
    Calendar,
    ContactManagement,
    Database,
    Dictionnary,
    Chart,
    Email,
    Finance,
    FlowChart,
    PDA,
    ProjectManagement,
    Presentation,
    Spreadsheet,
    WordProcessor,
    TwoDGraphics,
    VectorGraphics,
    RasterGraphics,
    ThreeDGraphics,
    Scanning,
    OCR,
    Photography,
    Publishing,
    Viewer,
    TextTools,
    DesktopSettings,
    HardwareSettings,
    Printing,
    PackageManager,
    Dialup,
    InstantMessaging,
    Chat,
    IRCClient,
    Feed,
    HamRadio,
    News,
    P2P,
    RemoteAccess,
    Telephony,
    TelephonyTools,
    VideoConference,
    WebBrowser,
    WebDevelopment,
    Midi,
    Mixer,
    Sequencer,
    Tuner,
    TV,
    AudioVideoEditing,
    Player,
    Recorder,
    DiscBurning,
    ActionGame,
    AdventureGame,
    ArcadeGame,
    BoardGame,
    BlocksGame,
    CardGame,
    KidsGame,
    LogicGame,
    RolePlaying,
    Shooter,
    Simulation,
    SportsGame,
    StrategyGame,
    Art,
    Construction,
    Music,
    Languages,
    ArtificialIntelligence,
    Astronomy,
    Biology,
    Chemistry,
    ComputerScience,
    DataVisualization,
    Economy,
    Electricity,
    Geography,
    Geology,
    Geoscience,
    History,
    Humanities,
    ImageProcessing,
    Literature,
    Maps,
    Math,
    NumericalAnalysis,
    MedicalSoftware,
    Physics,
    Robotics,
    Spirituality,
    Sports,
    ParallelComputing,
    Amusement,
    Archiving,
    Compression,
    Electronics,
    Emulator,
    Engineering,
    FileTools,
    FileManager,
    TerminalEmulator,
    FileTransfer,
    Filesystem,
    Monitor,
    Security,
    Accessibility,
    Calculator,
    Clock,
    TextEditor,
    Documentation,
    Adult,
    Core,
    KDE,
    GNOME,
    XFCE,
    GTK,
    Qt,
    Motif,
    Java,
    ConsoleOnly,
    // Reserved categories
    // https://specifications.freedesktop.org/menu-spec/latest/apas03.html
    Screensaver,
    TrayIcon,
    Applet,
    Shell,
    #[strum(default = "true")]
    Unknown(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "$value")]
pub enum Checksum {
    Sha1(String),
    Sha256(String),
    Blake2b(String),
    Blake2s(String),
}

#[derive(Clone, Debug, AsRefStr, Serialize, ToString, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[strum(rename_all = "kebab-case")]
pub enum ComponentKind {
    Runtime,
    #[serde(alias = "console")]
    #[strum(serialize = "console")]
    ConsoleApplication,
    #[serde(alias = "desktop")]
    #[strum(serialize = "desktop")]
    DesktopApplication,
    #[serde(alias = "webapp")]
    #[strum(serialize = "web-application")]
    WebApplication,
    #[serde(rename = "inputmethod")]
    #[strum(serialize = "inputmethod")]
    InputMethod,
    #[serde(alias = "operating-system")]
    #[strum(serialize = "operating-system")]
    OS,
    Theme,
    Firmware,
    Addon,
    Font,
    Generic,
    IconTheme,
    Localization,
    Driver,
    Codec,
}

impl Default for ComponentKind {
    fn default() -> Self {
        ComponentKind::Generic
    }
}

impl FromStr for ComponentKind {
    type Err = anyhow::Error;

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
            _ => anyhow::bail!("invalid component type"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "$value")]
pub enum ContentAttribute {
    #[serde(rename = "violence-cartoon")]
    ViolenceCartoon(ContentState),
    #[serde(rename = "violence-fantasy")]
    ViolenceFantasy(ContentState),
    #[serde(rename = "violence-fealistic")]
    ViolenceFealistic(ContentState),
    #[serde(rename = "violence-bloodshed")]
    ViolenceBloodshed(ContentState),
    #[serde(rename = "violence-sexual")]
    ViolenceSexual(ContentState),
    #[serde(rename = "violence-desecration")]
    ViolenceDesecration(ContentState),
    #[serde(rename = "violence-slavery")]
    ViolenceSlavery(ContentState),
    #[serde(rename = "violence-realistic")]
    ViolenceRealistic(ContentState),
    #[serde(rename = "violence-worship")]
    ViolenceWorship(ContentState),
    #[serde(rename = "drugs-alcohol")]
    DrugsAlcohol(ContentState),
    #[serde(rename = "drugs-narcotics")]
    DrugsNarcotics(ContentState),
    #[serde(rename = "drugs-tobacco")]
    DrugsTobacco(ContentState),
    #[serde(rename = "sex-nudity")]
    SexNudity(ContentState),
    #[serde(rename = "sex-themes")]
    SexThemes(ContentState),
    #[serde(rename = "sex-homosexuality")]
    SexHomosexuality(ContentState),
    #[serde(rename = "sex-prostitution")]
    SexProstitution(ContentState),
    #[serde(rename = "sex-adultery")]
    SexAdultery(ContentState),
    #[serde(rename = "sex-appearance")]
    SexAppearance(ContentState),
    #[serde(rename = "language-profanity")]
    LanguageProfanity(ContentState),
    #[serde(rename = "language-humor")]
    LanguageHumor(ContentState),
    #[serde(rename = "language-discrimination")]
    LanguageDiscrimination(ContentState),
    #[serde(rename = "social-chat")]
    SocialChat(ContentState),
    #[serde(rename = "social-info")]
    SocialInfo(ContentState),
    #[serde(rename = "social-audio")]
    SocialAudio(ContentState),
    #[serde(rename = "social-location")]
    SocialLocation(ContentState),
    #[serde(rename = "social-contacts")]
    SocialContacts(ContentState),
    #[serde(rename = "money-advertising")]
    MoneyAdvertising(ContentState),
    #[serde(rename = "money-purchasing")]
    MoneyPurchasing(ContentState),
    #[serde(rename = "money-gambling")]
    MoneyGambling(ContentState),
}

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub enum ContentRatingVersion {
    #[serde(rename = "oars-1.0")]
    Oars1_0,
    #[serde(rename = "oars-1.1")]
    Oars1_1,
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

#[derive(Clone, Debug, AsRefStr, ToString, EnumString, Deserialize, Serialize, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ContentState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "mild")]
    Mild,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "intense")]
    Intense,
}

impl Default for ContentState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Icon {
    /// Icon loaded from the stock.
    Stock(String),
    /// Icon cached
    Cached {
        path: PathBuf,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
    },
    /// Icon loaded from a remote URL.
    Remote {
        url: Url,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
    },
    /// Icon loaded from a file.
    Local {
        path: PathBuf,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
    },
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

#[derive(Clone, Debug, ToString, AsRefStr, Serialize, Deserialize, PartialEq, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ImageKind {
    Source,
    Thumbnail,
}

impl Default for ImageKind {
    fn default() -> Self {
        Self::Source
    }
}

#[derive(Clone, Debug, Deserialize, AsRefStr, ToString, Serialize, PartialEq, EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum Kudo {
    AppMenu,
    HiDpiIcon,
    HighContrast,
    ModernToolkit,
    Notifications,
    SearchProvider,
    UserDocs,
    #[strum(default = "true")]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "name")]
pub enum Launchable {
    #[serde(alias = "desktop_id")]
    DesktopId(String),
    Service(String),
    Url(Url),
    CockpitManifest(String),
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

#[derive(Clone, Debug, ToString, EnumString, AsRefStr, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReleaseKind {
    Stable,
    Development,
}

impl Default for ReleaseKind {
    fn default() -> Self {
        ReleaseKind::Stable
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "$value", rename_all = "kebab-case")]
pub enum Size {
    Download(u64),
    Installed(u64),
}

#[derive(Clone, Debug, AsRefStr, EnumString, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReleaseUrgency {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for ReleaseUrgency {
    fn default() -> Self {
        ReleaseUrgency::Medium
    }
}

#[derive(Clone, Debug, AsRefStr, EnumString, ToString, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum FirmwareKind {
    Flashed,
    Runtime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
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
        kind: FirmwareKind,
        #[serde(rename(deserialize = "$value", serialize = "item"))]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type", content = "name")]
pub enum Translation {
    Gettext(String),
    Qt(String),
    Unknown,
}
