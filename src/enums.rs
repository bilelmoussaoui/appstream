use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
use std::path::PathBuf;
use strum_macros::{EnumString, ToString};
use url::Url;

#[derive(Clone, Debug, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactKind {
    Source,
    Binary,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Bundle {
    Limba(String),
    Flatpak {
        #[serde(skip_serializing_if = "Option::is_none")]
        runtime: Option<String>,
        sdk: String,
        #[serde(rename = "$value", default)]
        id: String,
    },
    AppImage(String),
    Snap(String),
    Tarball(String),
}

#[derive(Clone, Debug, EnumString, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
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
    #[serde(alias = "network")]
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
    #[serde(alias = "webbrowser")]
    WebBrowser,
    #[serde(alias = "webdevelopment")]
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

#[derive(Clone, Debug, Serialize, ToString, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ComponentKind {
    Runtime,
    #[serde(alias = "console")]
    ConsoleApplication,
    #[serde(alias = "desktop")]
    DesktopApplication,
    #[serde(alias = "webapp")]
    WebApplication,
    #[serde(rename = "inputmethod")]
    InputMethod,
    #[serde(alias = "operating-system")]
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
            (ContentRatingVersion::Oars1_0, ContentRatingVersion::Oars1_1) => Ordering::Less,
            (ContentRatingVersion::Oars1_1, ContentRatingVersion::Oars1_0) => Ordering::Greater,
            (ContentRatingVersion::Oars1_0, ContentRatingVersion::Oars1_0)
            | (ContentRatingVersion::Oars1_1, ContentRatingVersion::Oars1_1) => Ordering::Equal,
            (ContentRatingVersion::Unknown, _) | (_, ContentRatingVersion::Unknown) => {
                Ordering::Less
            }
        }
    }
}

impl PartialOrd for ContentRatingVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Icon {
    Stock(String),
    Cached {
        path: PathBuf,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
    },
    Remote {
        url: Url,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
    },
    Local {
        path: PathBuf,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
    },
}

#[derive(Clone, Debug, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageKind {
    Source,
    Thumbnail,
}

#[derive(Clone, Debug, Deserialize, ToString, Serialize, PartialEq, EnumString)]
pub enum Kudo {
    AppMenu,
    HiDpiIcon,
    HighContrast,
    ModernToolkit,
    Notifications,
    SearchProvider,
    UserDocs,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Launchable {
    DesktopId(String),
    Service(String),
    Url(Url),
    CockpitManifest(String),
    Unknown(String),
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum ProjectUrl {
    Donation(Url),
    Translate(Url),
    Homepage(Url),
    BugTracker(Url),
    Help(Url),
    Faq(Url),
    Contact(Url),
    Unknown(Url),
}

#[derive(Clone, Debug, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
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

#[derive(Clone, Debug, ToString, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
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

#[derive(Clone, Debug, ToString, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FirmwareKind {
    Flashed,
    Runtime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Provide {
    Library(PathBuf),
    Binary(String),
    Font(String),
    Modalias(String),
    Firmware {
        #[serde(rename = "type")]
        kind: FirmwareKind,
        #[serde(rename = "$value")]
        item: String,
    },
    Python2(String),
    Python3(String),
    DBus(String),
    Id(String),
    Codec(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case", tag = "type", content = "$value")]
pub enum Translation {
    Gettext(String),
    Qt(String),
    Unknown,
}

#[test]
fn test_provide_firmware() {
    let x = r"<firmware type='runtime'>ipw2200-bss.fw</firmware>";
    let p: Provide = quick_xml::de::from_str(&x).unwrap();
    assert_eq!(
        p,
        Provide::Firmware {
            kind: FirmwareKind::Runtime,
            item: "ipw2200-bss.fw".into()
        }
    );
}
