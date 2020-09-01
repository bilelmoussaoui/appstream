use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const DEFAULT_LOCALE: &str = "C";

fn element_to_xml(e: &xmltree::Element) -> String {
    e.children
        .iter()
        .map(|node| match node {
            xmltree::XMLNode::Element(c) => {
                format!("<{}>{}</{}>", c.name, element_to_xml(c), c.name)
            }
            xmltree::XMLNode::Text(t) => t.clone(),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MarkupTranslatableString(pub BTreeMap<String, String>);

impl Default for MarkupTranslatableString {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl MarkupTranslatableString {
    pub fn with_default(text: &str) -> Self {
        let mut t = Self::default();
        t.add_for_locale(None, text);
        t
    }

    pub fn and_locale(mut self, locale: &str, text: &str) -> Self {
        self.add_for_locale(Some(locale), text);
        self
    }

    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        let locale = element.attributes.get("lang").map(|l| l.as_str());
        self.add_for_locale(locale, &element_to_xml(element));
    }

    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0.insert(
            locale.unwrap_or_else(|| DEFAULT_LOCALE).to_string(),
            text.to_string(),
        );
    }

    pub fn get_default(&self) -> Option<&String> {
        self.0.get(DEFAULT_LOCALE)
    }

    pub fn get_for_locale(&self, locale: &str) -> Option<&String> {
        self.0.get(locale)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TranslatableString(pub BTreeMap<String, String>);

impl Default for TranslatableString {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl TranslatableString {
    pub fn with_default(text: &str) -> Self {
        let mut t = Self::default();
        t.add_for_locale(None, text);
        t
    }

    pub fn and_locale(mut self, locale: &str, text: &str) -> Self {
        self.add_for_locale(Some(locale), text);
        self
    }

    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        let locale = element.attributes.get("lang").map(|l| l.as_str());
        if let Some(text) = element.get_text() {
            self.add_for_locale(locale, &text.into_owned());
        }
    }

    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0.insert(
            locale.unwrap_or_else(|| DEFAULT_LOCALE).to_string(),
            text.to_string(),
        );
    }

    pub fn get_default(&self) -> Option<&String> {
        self.0.get(DEFAULT_LOCALE)
    }

    pub fn get_for_locale(&self, locale: &str) -> Option<&String> {
        self.0.get(locale)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct TranslatableList(pub BTreeMap<String, Vec<String>>);

impl TranslatableList {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn with_default(words: Vec<&str>) -> Self {
        let mut t = Self::default();
        words.iter().for_each(|w| {
            t.add_for_locale(None, w);
        });
        t
    }

    pub fn and_locale(mut self, locale: &str, words: Vec<&str>) -> Self {
        words.iter().for_each(|w| {
            self.add_for_locale(Some(locale), w);
        });
        self
    }

    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        self.add_for_locale(
            element.attributes.get("lang").map(|l| l.as_str()),
            &element.get_text().unwrap().into_owned(),
        );
    }

    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0
            .entry(locale.unwrap_or_else(|| DEFAULT_LOCALE).into())
            .and_modify(|sentenses| {
                sentenses.push(text.into());
            })
            .or_insert_with(|| vec![text.to_string()]);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
