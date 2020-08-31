mod current;
use appstream::builders::*;
use appstream::types::{AppId, License, TranslatableString, TranslatableVec};
use appstream::{Collection, Component};
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use xmltree::{Element, XMLNode};

fn main() {
    let file = File::open("test.xml").unwrap();
    let file = BufReader::new(file);
    let component = Component::try_from(&Element::parse(file).unwrap());

    println!("{:#?}", component);
}
