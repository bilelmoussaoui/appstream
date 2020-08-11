use serde::Serialize;
use std::collections::HashMap;

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
        t.add_for_lang("default", text);
        t
    }

    pub fn set_is_markup(&mut self, is_markup: bool) {
        self.1 = is_markup;
    }

    pub fn add_for_lang(&mut self, lang: &str, text: &str) {
        self.0.insert(lang.into(), text.to_string());
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Default)]
pub struct TranslatableVec(pub HashMap<String, Vec<String>>);

impl TranslatableVec {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_for_lang(&mut self, lang: &str, text: &str) {
        self.0
            .entry(lang.into())
            .and_modify(|sentenses| {
                sentenses.push(text.into());
            })
            .or_insert_with(|| vec![text.to_string()]);
    }
}
