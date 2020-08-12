use serde::Serialize;
use std::collections::HashMap;

pub const DEFAULT_LOCALE: &str = "C";

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct TranslatableString(pub HashMap<String, String>, bool);

impl Default for TranslatableString {
    fn default() -> Self {
        Self(HashMap::new(), false)
    }
}

impl TranslatableString {
    pub fn with_default(text: &str) -> Self {
        let mut t = Self::default();
        t.add_for_locale(None, text);
        t
    }

    pub fn set_is_markup(&mut self, is_markup: bool) {
        self.1 = is_markup;
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
}

#[derive(Clone, Debug, Serialize, PartialEq, Default)]
pub struct TranslatableVec(pub HashMap<String, Vec<String>>);

impl TranslatableVec {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_default(words: Vec<&str>) -> Self {
        let mut t = Self::default();
        words.iter().for_each(|w| {
            t.add_for_locale(None, w);
        });
        t
    }

    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0
            .entry(locale.unwrap_or_else(|| DEFAULT_LOCALE).into())
            .and_modify(|sentenses| {
                sentenses.push(text.into());
            })
            .or_insert_with(|| vec![text.to_string()]);
    }
}
