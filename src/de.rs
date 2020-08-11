use super::enums::{Bundle, Category, Icon, Kudo, Launchable, ProjectUrl, Provide, Translation};
use super::translatable_string::{TranslatableString, TranslatableVec};
use super::{AppId, ContentRating, Language, License, Release, Screenshot};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::de;
use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;
use url::Url;

pub(crate) fn icon_deserialize<'de, D>(deserializer: D) -> Result<Vec<Icon>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PIcon {
        #[serde(rename = "type", default)]
        pub _type: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
        #[serde(rename = "$value")]
        path: String,
    };

    let picons: Vec<PIcon> = Vec::deserialize(deserializer)?;
    Ok(picons
        .into_iter()
        .map(
            |pi| match pi._type.unwrap_or_else(|| "cached".to_string()).as_ref() {
                "stock" => Icon::Stock(pi.path),
                "local" => Icon::Local {
                    path: pi.path.into(),
                    width: pi.width,
                    height: pi.height,
                },
                "remote" => Icon::Remote {
                    url: Url::from_str(&pi.path).unwrap(),
                    width: pi.width,
                    height: pi.height,
                },
                _ => Icon::Cached(pi.path),
            },
        )
        .collect::<Vec<Icon>>())
}

pub(crate) fn timestamp_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer);
    match s {
        Ok(timestamp) => Ok(Some(
            chrono::Utc
                .datetime_from_str(&timestamp, "%s")
                .or_else(
                    |_: chrono::ParseError| -> Result<DateTime<Utc>, chrono::ParseError> {
                        let date: NaiveDateTime =
                            NaiveDate::parse_from_str(&timestamp, "%Y-%m-%d")?.and_hms(0, 0, 0);
                        Ok(DateTime::<Utc>::from_utc(date, chrono::Utc))
                    },
                )
                .map_err(serde::de::Error::custom)?,
        )),
        Err(_) => Ok(None),
    }
}

pub(crate) fn app_id_deserialize<'de, D>(deserializer: D) -> Result<AppId, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(AppId { 0: s })
}

pub(crate) fn license_deserialize<'de, D>(deserializer: D) -> Result<Option<License>, D::Error>
where
    D: de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(s) => Ok(Some(License(s))),
        _ => Ok(None),
    }
}

pub(crate) fn bundle_deserialize<'de, D>(deserializer: D) -> Result<Vec<Bundle>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PBundle {
        #[serde(rename = "type")]
        _type: String,
        runtime: Option<String>,
        sdk: String,
        #[serde(rename = "$value", default)]
        name: String,
    };

    let bundles: Vec<PBundle> = Vec::deserialize(deserializer)?;
    Ok(bundles
        .into_iter()
        .map(|b| match b._type.as_ref() {
            "flatpak" => Bundle::Flatpak {
                name: b.name,
                sdk: b.sdk,
                runtime: b.runtime,
            },
            "limba" => Bundle::Limba(b.name),
            "snap" => Bundle::Snap(b.name),
            "appimage" => Bundle::AppImage(b.name),
            _ => Bundle::Tarball(b.name),
        })
        .collect::<Vec<Bundle>>())
}

pub(crate) fn extends_deserialize<'de, D>(deserializer: D) -> Result<Vec<AppId>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let extends: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(extends
        .into_iter()
        .map(|e| AppId::try_from(e.as_ref()).expect("Invalid AppId"))
        .collect::<Vec<AppId>>())
}

pub(crate) fn provides_deserialize<'de, D>(deserializer: D) -> Result<Vec<Provide>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PProvides {
        #[serde(rename = "$value", default)]
        pub val: Vec<Provide>,
    };

    let provides = PProvides::deserialize(deserializer)?;
    Ok(provides.val)
}

pub(crate) fn content_rating_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<ContentRating>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut contents: Vec<ContentRating> = Vec::deserialize(deserializer)?;
    contents.sort_by(|a, b| b.version.cmp(&a.version));

    Ok(contents.into_iter().next())
}

pub(crate) fn keywords_deserialize<'de, D>(deserializer: D) -> Result<TranslatableVec, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PKeywords {
        #[serde(rename = "keyword")]
        keywords: Vec<PKeyword>,
    };
    #[derive(Debug, Deserialize)]
    struct PKeyword {
        #[serde(rename = "xml:lang", default)]
        pub lang: Option<String>,
        #[serde(rename = "$value")]
        text: String,
    };

    let s: PKeywords = PKeywords::deserialize(deserializer)?;

    let mut translatable = TranslatableVec::new();
    s.keywords.into_iter().for_each(|t| {
        translatable.add_for_lang(&t.lang.unwrap_or_else(|| "default".to_string()), &t.text);
    });
    Ok(translatable)
}

pub(crate) fn kudos_deserialize<'de, D>(deserializer: D) -> Result<Vec<Kudo>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize, PartialEq, Default)]
    pub struct Kudos {
        #[serde(rename = "$value", default)]
        kudos: Vec<String>,
    }

    let k: Kudos = Kudos::deserialize(deserializer)?;
    Ok(k.kudos
        .into_iter()
        .map(|k| Kudo::from_str(&k).unwrap())
        .collect::<Vec<Kudo>>())
}

pub(crate) fn mimetypes_deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    pub struct Mimetypes {
        #[serde(rename = "$value", default)]
        mimes: Vec<String>,
    }

    let m: Mimetypes = Mimetypes::deserialize(deserializer)?;
    Ok(m.mimes)
}

pub(crate) fn releases_deserialize<'de, D>(deserializer: D) -> Result<Vec<Release>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PReleases {
        #[serde(rename = "release")]
        releases: Vec<Release>,
    };

    let r: PReleases = PReleases::deserialize(deserializer)?;
    Ok(r.releases)
}

pub(crate) fn languages_deserialize<'de, D>(deserializer: D) -> Result<Vec<Language>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PLanguages {
        #[serde(rename = "lang")]
        languages: Vec<Language>,
    };

    let l: PLanguages = PLanguages::deserialize(deserializer)?;
    Ok(l.languages)
}

pub(crate) fn translatable_deserialize<'de, D>(
    deserializer: D,
) -> Result<TranslatableString, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PTranslatable {
        #[serde(rename = "xml:lang", default)]
        pub lang: Option<String>,
        #[serde(rename = "$value", default)]
        pub val: String,
    };

    let s: Vec<PTranslatable> = Vec::deserialize(deserializer)?;

    let mut translatable = TranslatableString::default();
    s.into_iter().for_each(|t| {
        translatable
            .0
            .insert(t.lang.unwrap_or_else(|| "default".to_string()), t.val);
    });
    Ok(translatable)
}

pub(crate) fn some_translatable_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<TranslatableString>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PTranslatable {
        #[serde(rename = "xml:lang", default)]
        pub lang: Option<String>,
        #[serde(rename = "$value", default)]
        pub val: String,
    };

    let s: Option<Vec<PTranslatable>> = Option::deserialize(deserializer)?;

    let mut translatable = TranslatableString::default();
    match s {
        Some(a) => {
            a.into_iter().for_each(|t| {
                translatable
                    .0
                    .insert(t.lang.unwrap_or_else(|| "default".to_string()), t.val);
            });
            Ok(Some(translatable))
        }
        None => Ok(None),
    }
}

pub(crate) fn screenshots_deserialize<'de, D>(deserializer: D) -> Result<Vec<Screenshot>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PScreenshot {
        #[serde(rename = "screenshot", default)]
        pub screenshots: Vec<Screenshot>,
    };

    let s: PScreenshot = PScreenshot::deserialize(deserializer)?;
    Ok(s.screenshots)
}

pub(crate) fn launchable_deserialize<'de, D>(deserializer: D) -> Result<Vec<Launchable>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PLaunchable {
        #[serde(rename = "type", default)]
        pub _type: String,
        #[serde(rename = "$value", default)]
        pub val: String,
    };

    let launchables: Vec<PLaunchable> = Vec::deserialize(deserializer)?;

    Ok(launchables
        .into_iter()
        .map(|l| match l._type.as_ref() {
            "desktop-id" => Launchable::DesktopId(l.val),
            "service" => Launchable::Service(l.val),
            "url" => Launchable::Url(Url::from_str(&l.val).unwrap()),
            "cockpit-manifest" => Launchable::CockpitManifest(l.val),
            _ => Launchable::Unknown(l.val),
        })
        .collect::<Vec<Launchable>>())
}

pub(crate) fn category_deserialize<'de, D>(deserializer: D) -> Result<Vec<Category>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PCategories {
        #[serde(rename = "$value")]
        pub categories: Vec<PCategory>,
    };
    #[derive(Debug, Deserialize)]
    struct PCategory {
        #[serde(rename = "$value", default)]
        pub categories: Vec<String>,
    };

    let c: PCategories = PCategories::deserialize(deserializer)?;

    let mut categories = Vec::new();
    c.categories.into_iter().for_each(|c| {
        c.categories.into_iter().for_each(|category: String| {
            categories.push(Category::from_str(&category).unwrap_or(Category::Unknown(category)))
        })
    });

    Ok(categories)
}

pub(crate) fn urls_deserialize<'de, D>(deserializer: D) -> Result<Vec<ProjectUrl>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PUrl {
        #[serde(rename = "type", default)]
        pub _type: String,
        #[serde(rename = "$value", default)]
        pub url: String,
    };

    let urls: Vec<PUrl> = Vec::deserialize(deserializer)?;

    Ok(urls
        .into_iter()
        .map(|u| {
            let url = Url::from_str(&u.url).expect("Failed to parse url, invalid");
            match u._type.as_str() {
                "homepage" => ProjectUrl::Homepage(url),
                "help" => ProjectUrl::Help(url),
                "donation" => ProjectUrl::Donation(url),
                "bugtracker" => ProjectUrl::BugTracker(url),
                "translate" => ProjectUrl::Translate(url),
                "faq" => ProjectUrl::Faq(url),
                "contact" => ProjectUrl::Contact(url),
                _ => ProjectUrl::Unknown(url),
            }
        })
        .collect::<Vec<ProjectUrl>>())
}

pub(crate) fn translation_deserialize<'de, D>(deserializer: D) -> Result<Vec<Translation>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PTranslate {
        #[serde(rename = "type", default)]
        pub _type: String,
        #[serde(rename = "$value", default)]
        pub name: String,
    };

    let translations: Vec<PTranslate> = Vec::deserialize(deserializer)?;

    Ok(translations
        .into_iter()
        .map(|t| match t._type.as_str() {
            "qt" => Translation::Qt(t.name),
            "gettext" => Translation::Gettext(t.name),
            _ => Translation::Unknown,
        })
        .collect::<Vec<Translation>>())
}
