use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const DEFAULT_LOCALE: &str = "C";

fn element_to_xml(e: &xmltree::Element) -> String {
    e.children
        .iter()
        .map(|node| match node {
            xmltree::XMLNode::Element(ref c) => {
                format!("<{}>{}</{}>", c.name, element_to_xml(c), c.name)
            }
            xmltree::XMLNode::Text(t) => t.clone(),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
/// A wrapper around a translable string that can contains markup.
///
///
/// # Example
/// ```
/// use appstream::MarkupTranslatableString;
/// let description = MarkupTranslatableString::with_default("<p>Contrast checks whether the contrast between two colors meet the WCAG requirements.</p>")
///                 .and_locale("cs", "<p>Kontroluje kontrast mezi dvěma zadanými barvami, jestli vyhovuje požadavkům pravidel pro bezbariérové weby (WCAG).</p>")
///                 .and_locale("es", "<p>Contraste comprueba la diferencia de contraste entre dos colores que cumplen los requisitos WCAG.</p>");
/// ```
pub struct MarkupTranslatableString(pub BTreeMap<String, String>);

impl MarkupTranslatableString {
    /// Create a new `MarkupTranslatableString` using the default locale.
    ///
    /// # Arguments
    ///
    /// * `text` - The string that corresponds to the default locale.
    pub fn with_default(text: &str) -> Self {
        let mut t = Self::default();
        t.add_for_locale(None, text);
        t
    }

    /// Adds a new translation for a specific locale.
    ///
    /// Very useful when constructing a `TranslatableString` manually.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use, use `with_default` if you want the default locale.
    /// * `text` - The corresponding translation.
    pub fn and_locale(mut self, locale: &str, text: &str) -> Self {
        self.add_for_locale(Some(locale), text);
        self
    }

    /// Adds a new string from a `xmltree.Element`
    ///
    /// XML elements containing a `lang` attribute are marked as translatable
    /// and can be used to feed the `MarkupTranslatableString`.
    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        let locale = element.attributes.get("lang").map(|l| l.as_str());
        self.add_for_locale(locale, &element_to_xml(&element));
    }

    /// Adds a new translation for a speicifc locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use, the default locale is used if `None` is set instead.
    /// * `text` - The translation corresponding to the locale.
    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0.insert(
            locale.unwrap_or_else(|| DEFAULT_LOCALE).to_string(),
            text.to_string(),
        );
    }

    /// Returns the text corresponding to the default locale `C`.
    pub fn get_default(&self) -> Option<&String> {
        self.0.get(DEFAULT_LOCALE)
    }

    /// Retrieve the corresponding text for a specific locale if available.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to retrieve the text for.  
    pub fn get_for_locale(&self, locale: &str) -> Option<&String> {
        self.0.get(locale)
    }

    /// Whether `self` contains any translatable strings.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
/// A wrapper around a translatable string.
///
/// # Example
/// ```
/// use appstream::TranslatableString;
/// let name = TranslatableString::with_default("Contrast")
///             .and_locale("cs", "Kontrast")
///             .and_locale("cs", "Kontrast");
/// ```
pub struct TranslatableString(pub BTreeMap<String, String>);

impl TranslatableString {
    /// Create a new `TranslatableString` using the default locale.
    ///
    /// # Arguments
    ///
    /// * `text` - The string that corresponds to the default locale.
    pub fn with_default(text: &str) -> Self {
        let mut t = Self::default();
        t.add_for_locale(None, text);
        t
    }

    /// Adds a new translation for a specific locale.
    ///
    /// Very useful when constructing a `TranslatableString` manually.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use, use `with_default` if you want the default locale.
    /// * `text` - The corresponding translation.
    pub fn and_locale(mut self, locale: &str, text: &str) -> Self {
        self.add_for_locale(Some(locale), text);
        self
    }

    /// Adds a new string from a `xmltree.Element`
    ///
    /// XML elements containing a `lang` attribute are marked as translatable
    /// and can be used to feed the `TranslatableString`.
    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        self.add_for_locale(
            element.attributes.get("lang").map(|l| l.as_str()),
            &element.get_text().unwrap_or_default(), // for some reason some description tags contains empty strings.
        );
    }

    /// Adds a new translation for a speicifc locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use, the default locale is used if `None` is set instead.
    /// * `text` - The translation corresponding to the locale.
    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0.insert(
            locale.unwrap_or_else(|| DEFAULT_LOCALE).to_string(),
            text.to_string(),
        );
    }

    /// Returns the text corresponding to the default locale `C`.
    pub fn get_default(&self) -> Option<&String> {
        self.0.get(DEFAULT_LOCALE)
    }

    /// Retrieve the corresponding text for a specific locale if available.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to retrieve the text for.    
    pub fn get_for_locale(&self, locale: &str) -> Option<&String> {
        self.0.get(locale)
    }

    /// Whether `self` contains any translatable strings.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
/// A wrapper around a list of strings that are translatable.
///
/// It's mostly used for the list of keywords a component can have
/// # Example
///
/// ```
/// use appstream::TranslatableList;
/// let keywords = TranslatableList::with_default(vec!["Color", "Contrast", "GNOME", "GTK"])
///                         .and_locale("cs", vec!["barva", "kontrast"])
///                         .and_locale("da", vec!["Farve", "Kontrast"]);
/// ```
pub struct TranslatableList(pub BTreeMap<String, Vec<String>>);

impl TranslatableList {
    /// Create a new `TranslatableList` using the default locale.
    ///
    /// # Arguments
    ///
    /// * `words` - List of strings to add to the default locale.
    pub fn with_default(words: Vec<&str>) -> Self {
        let mut t = Self::default();
        words.iter().for_each(|w| {
            t.add_for_locale(None, w);
        });
        t
    }

    /// Adds a list of strings for a specific locale.
    ///
    /// Very useful when constructing a `TranslatableList` manually.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use, use `with_default` if you want the default locale.
    /// * `words` - The list of strings to add to this specific locale.
    pub fn and_locale(mut self, locale: &str, words: Vec<&str>) -> Self {
        words.iter().for_each(|w| {
            self.add_for_locale(Some(locale), w);
        });
        self
    }

    /// Adds a new string from a `xmltree.Element`
    ///
    /// XML elements containing a `lang` attribute are marked as translatable
    /// and can be used to feed the `TranslatableList`.
    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        self.add_for_locale(
            element.attributes.get("lang").map(|l| l.as_str()),
            &element.get_text().unwrap_or_default(),
        );
    }

    /// Adds a new string for a specific locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use, `C` is used if `None` is provided.
    /// * `text` - The string to add.
    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0
            .entry(locale.unwrap_or_else(|| DEFAULT_LOCALE).into())
            .and_modify(|sentenses| {
                sentenses.push(text.into());
            })
            .or_insert_with(|| vec![text.to_string()]);
    }

    /// Whether `self` contains any translatable strings.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
