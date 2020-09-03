//! # Appstream
//!
//! AppStream is a cross-distro effort for enhancing the metadata available about software components in the Linux and free-software ecosystem.
//! One of the project's goals is to make building software-center applications possible, and make interaction with the package sources of
//! a distribution smarter. AppStream provides specifications for meta-information which is shipped by upstream projects
//! and can be consumed by other software. The meta-information includes data which is interesting to display in software centers
//! and is mainly useful for end-users, as well as descriptions about the public interfaces a software component provides,
//!  which is mainly useful for developers, 3rd-party software installers and for automatically installing missing components
//!  on a distribution, for example missing firmware or mimetype-handlers.
//!
//! Specifications: [https://www.freedesktop.org/software/appstream/docs/](https://www.freedesktop.org/software/appstream/docs/)
//!
//! This crate aimes to provide an easy and sane Rust parser of Appstream using [xmltree](https://docs.rs/xmltree/)
//!
//! # Examples
//! ```
//! use appstream::Component;
//! use appstream::builders::{ComponentBuilder, ReleaseBuilder};
//! use appstream::TranslatableString;
//! use appstream::enums::{Provide, ProjectUrl};
//! use url::Url;
//! use chrono::{Utc, TimeZone};
//! use std::convert::TryFrom;
//!
//! let xml = r"<?xml version='1.0' encoding='UTF-8'?>
//!                 <component>
//!                     <id>com.example.foobar</id>
//!                     <name>Foo Bar</name>
//!                     <summary>A foo-ish bar</summary>
//!                     <url type='homepage'>http://www.example.org</url>
//!                     <metadata_license>CC0-1.0</metadata_license>
//!                     
//!                     <provides>
//!                       <library>libfoobar.so.2</library>
//!                       <font>foo.ttf</font>
//!                       <binary>foobar</binary>
//!                     </provides>
//!                     <releases>
//!                       <release version='1.2' date='2015-02-16'/>
//!                     </releases>
//!                     <developer_name>FooBar Team</developer_name>
//!                 </component>";
//! let element = xmltree::Element::parse(xml.as_bytes()).unwrap();
//! let c1 = Component::try_from(&element).unwrap();
//!
//! let c2 = ComponentBuilder::default()
//!     .id("com.example.foobar".into())
//!     .name(TranslatableString::with_default("Foo Bar"))
//!     .metadata_license("CC0-1.0".into())
//!     .summary(TranslatableString::with_default("A foo-ish bar"))
//!     .url(ProjectUrl::Homepage(
//!         Url::parse("http://www.example.org").unwrap(),
//!     ))
//!     .developer_name(TranslatableString::with_default("FooBar Team"))
//!     .provide(Provide::Library("libfoobar.so.2".into()))
//!     .provide(Provide::Font("foo.ttf".into()))
//!     .provide(Provide::Binary("foobar".into()))
//!     .release(
//!         ReleaseBuilder::new("1.2")
//!             .date(Utc.ymd(2015, 2, 16).and_hms_milli(0, 0, 0, 0))
//!             .build(),
//!     )
//!     .build();
//!
//! assert_eq!(c1, c2);
//! ```
//!
//! The library can parse a collection of components as well
//! ```no_run
//! use appstream::{Collection, Component};
//!
//! let collection = Collection::from_path("/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml".into()).unwrap();
//! #[cfg(feature="gzip")]
//! let collection = Collection::from_gzipped("/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml.gz".into()).unwrap();
//! // Find a specific application by id
//! println!("{:#?}", collection.find_by_id("org.gnome.design.Contrast".into()));
//!
//! // Find the list of gedit plugins
//! collection.components.iter()
//!     .filter(|c| c.extends.contains(&"org.gnome.gedit".into()))
//!     .collect::<Vec<&Component>>();
//! ```
//!
#![deny(missing_docs)]

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
mod screenshot;
mod translatable_string;
mod xml;

pub use app_id::AppId;
pub use collection::Collection;
pub use component::Component;
pub use content_rating::ContentRating;
pub use error::ParseError;
pub use language::Language;
pub use license::License;
pub use release::{Artifact, Release};
pub use screenshot::{Image, Screenshot, Video};
pub use translatable_string::{MarkupTranslatableString, TranslatableList, TranslatableString};
pub use url;
pub use xmltree;
