//! # Appstream
//!
//! AppStream is a cross-distro effort for enhancing the metadata available
//! about software components in the Linux and free-software ecosystem.
//! One of the project's goals is to make building software-center applications
//! possible, and make interaction with the package sources of a distribution
//! smarter. AppStream provides specifications for meta-information which is
//! shipped by upstream projects and can be consumed by other software. The
//! meta-information includes data which is interesting to display in software
//! centers and is mainly useful for end-users, as well as descriptions about
//! the public interfaces a software component provides,  which is mainly useful
//! for developers, 3rd-party software installers and for automatically
//! installing missing components  on a distribution, for example missing
//! firmware or mimetype-handlers.
//!
//! Specifications: [https://www.freedesktop.org/software/appstream/docs/](https://www.freedesktop.org/software/appstream/docs/)
//!
//! This crate aimes to provide an easy and sane Rust parser of Appstream using [xmltree](https://docs.rs/xmltree/)
//!
//! The `chrono` or `time` crates can be used to represent dates. `chrono` is the default. To use `time` instead, turn off default features and enable the `time` feature, like this:
//! ```toml
//! [dependencies]
//! appstream = { version = "*", default-features = false, features = ["time"] }
//! ```
//!
//! # Examples
//!
//! ```
//! use std::convert::TryFrom;
//!
//! use appstream::{
//!     builders::{ComponentBuilder, ReleaseBuilder},
//!     enums::{ProjectUrl, Provide},
//!     Component, DateTime, ParseError, TranslatableString,
//! };
//! use url::Url;
//!
//! fn main() -> Result<(), ParseError> {
//!     let xml = r"<?xml version='1.0' encoding='UTF-8'?>
//!                     <component>
//!                         <id>com.example.foobar</id>
//!                         <name>Foo Bar</name>
//!                         <summary>A foo-ish bar</summary>
//!                         <url type='homepage'>http://www.example.org</url>
//!                         <metadata_license>CC0-1.0</metadata_license>
//!                         <provides>
//!                           <library>libfoobar.so.2</library>
//!                           <font>foo.ttf</font>
//!                           <binary>foobar</binary>
//!                         </provides>
//!                         <releases>
//!                           <release version='1.2'/>
//!                         </releases>
//!                         <developer_name>FooBar Team</developer_name>
//!                     </component>";
//!     let element = xmltree::Element::parse(xml.as_bytes())?;
//!     let c1 = Component::try_from(&element)?;
//!
//!     let c2 = ComponentBuilder::default()
//!         .id("com.example.foobar".into())
//!         .name(TranslatableString::with_default("Foo Bar"))
//!         .metadata_license("CC0-1.0".into())
//!         .summary(TranslatableString::with_default("A foo-ish bar"))
//!         .url(ProjectUrl::Homepage(Url::parse("http://www.example.org")?))
//!         .developer_name(TranslatableString::with_default("FooBar Team"))
//!         .provide(Provide::Library("libfoobar.so.2".into()))
//!         .provide(Provide::Font("foo.ttf".into()))
//!         .provide(Provide::Binary("foobar".into()))
//!         .release(
//!             ReleaseBuilder::new("1.2")
//!                 .build(),
//!         )
//!         .build();
//!
//!     assert_eq!(c1, c2);
//!
//!     Ok(())
//! }
//! ```
//!
//! The library can parse a collection of components as well
//! ```no_run
//! use appstream::{Collection, Component, ParseError};
//!
//! fn main() -> Result<(), ParseError> {
//!     let collection = Collection::from_path(
//!         "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml".into(),
//!     )?;
//!     #[cfg(feature = "gzip")]
//!     let collection = Collection::from_gzipped(
//!         "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml.gz".into(),
//!     )?;
//!     // Find a specific application by id
//!     println!(
//!         "{:#?}",
//!         collection.find_by_id("org.gnome.design.Contrast".into())
//!     );
//!
//!     // Find the list of gedit plugins
//!     collection
//!         .components
//!         .iter()
//!         .filter(|c| c.extends.contains(&"org.gnome.gedit".into()))
//!         .collect::<Vec<&Component>>();
//!
//!     Ok(())
//! }
//! ```
#![deny(missing_docs)]

#[macro_use]
extern crate cfg_if;

mod app_id;
/// Various helpers to build any appstream type.
pub mod builders;
mod collection;
mod component;
mod content_rating;
/// Various enumerations used in the appstream types.
pub mod enums;
mod error;
mod language;
mod license;
mod release;
mod requirements;
mod screenshot;
mod translatable_string;
mod xml;

pub use app_id::AppId;
pub use collection::Collection;
pub use component::Component;
pub use content_rating::ContentRating;
pub use error::{ContextParseError, ParseError};
pub use language::Language;
pub use license::License;
pub use release::{Artifact, Release};
pub use requirements::{Control, DisplayLength, DisplayLengthValue, Requirement};
pub use screenshot::{Image, Screenshot, Video};
pub use translatable_string::{MarkupTranslatableString, TranslatableList, TranslatableString};
pub use url;
pub use xmltree;

cfg_if! {
    if #[cfg(feature = "time")] {
        /// The time module DateTime re-export
        pub use time::OffsetDateTime as DateTime;
    } else {
        use chrono::{DateTime as ChronoDateTime, Utc};
        /// The chrono module DateTime re-export
        pub type DateTime = ChronoDateTime<Utc>;
    }
}

#[cfg(test)]
#[inline]
fn date(year: i32, month: u8, day: u8) -> DateTime {
    cfg_if! {
        if #[cfg(feature = "time")] {
            return time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day).unwrap().midnight().assume_utc();
        } else {
            use chrono::TimeZone;
            return Utc.with_ymd_and_hms(year, month.into(), day.into(), 0, 0, 0).unwrap();
        }
    }
}

#[cfg(test)]
#[inline]
fn timestamp(timestamp: &str) -> DateTime {
    cfg_if! {
        if #[cfg(feature = "time")] {
            use time::{macros::format_description};
            let format = format_description!("[unix_timestamp]");
            return DateTime::parse(timestamp, &format).unwrap()
        } else {
            use chrono::TimeZone;
            return Utc.datetime_from_str(timestamp, "%s").unwrap();
        }
    }
}
