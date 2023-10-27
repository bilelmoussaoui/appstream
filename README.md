# appstream
Appstream files parser using Rust & [xmltree](https://docs.rs/xmltree/)

[![](https://docs.rs/appstream/badge.svg)](https://docs.rs/appstream/) [![](https://img.shields.io/crates/v/appstream)](https://crates.io/crates/appstream) ![](https://github.com/bilelmoussaoui/appstream/workflows/CI/badge.svg)

Specifications: [https://www.freedesktop.org/software/appstream/docs/](https://www.freedesktop.org/software/appstream/docs/)

The `chrono` or `time` crates can be used to represent dates. `chrono` is the default. To use `time` instead, turn off default features and enable the `time` feature, like this:

```toml
[dependencies]
appstream = { version = "*", default-features = false, features = ["time"] }
```

How to use
```rust
use appstream::{Collection, Component, ParseError};

fn main() -> Result<(), ParseError> {
    let collection = Collection::from_path("/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml".into())?;
    // Find a specific application by id
    println!("{:#?}", collection.find_by_id("org.gnome.design.Contrast".into()));

    // Find the list of gedit plugins
    collection.components.iter()
        .filter(|c| c.extends.contains(&"org.gnome.gedit".into()))
        .collect::<Vec<&Component>>();

    Ok(())
}
```