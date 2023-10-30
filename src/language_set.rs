use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::Index;

use crate::Language;

/// A searchable set of language string instances
#[derive(Serialize, Deserialize, Clone)]
pub struct LanguageSet {
    current: String,
    fallback: String,
    languages: HashMap<String, Language>,
}

impl LanguageSet {
    /// Create a new language instance
    ///
    /// # Arguments
    /// * `fallback_language` - Language code for the fallback language
    /// * `languages` - Array of language instances to use
    pub fn new(fallback_language: &str, languages: &[Language]) -> Self {
        Self {
            current: fallback_language.to_string(),
            fallback: fallback_language.to_string(),
            languages: languages
                .iter()
                .map(|l| (l.short_name().to_string(), l.clone()))
                .collect(),
        }
    }

    /// List all supported languages
    pub fn all_languages(&self) -> Vec<Language> {
        self.languages.values().cloned().collect()
    }

    /// Return the set's fallback language
    pub fn fallback_language(&self) -> Option<&Language> {
        self.languages.get(&self.fallback)
    }

    /// Return the set's current language
    pub fn current_language(&self) -> Option<&Language> {
        self.languages.get(&self.current)
    }

    /// Add a language to the set
    ///
    /// # Arguments
    /// * `language` - New language
    pub fn add_language(&mut self, language: Language) {
        self.languages
            .insert(language.short_name().to_string(), language);
    }

    /// Add a language from a JSON file to the set
    ///
    /// # Arguments
    /// * `language` - New language
    pub fn load_language(
        &mut self,
        filename: &str,
        resources: HashMap<String, Vec<u8>>,
    ) -> Result<(), String> {
        match Language::new_from_file(filename, resources) {
            Ok(lang) => {
                self.add_language(lang);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Check the completeness of all language packs against the fallback
    /// Returns the list of missing strings for each language
    pub fn verify(&self) -> HashMap<String, Vec<String>> {
        if let Some(fallback) = self
            .fallback_language()
            .and_then(|l| Some(l.strings().keys().cloned().collect::<HashSet<String>>()))
        {
            self.languages
                .iter()
                .map(|l| {
                    (
                        l.0.clone(),
                        l.1.strings()
                            .keys()
                            .cloned()
                            .collect::<HashSet<String>>()
                            .difference(&fallback)
                            .cloned()
                            .collect(),
                    )
                })
                .collect::<HashMap<String, Vec<String>>>()
        } else {
            HashMap::default()
        }
    }

    /// Set the fallback language for lookups
    ///
    /// # Arguments
    /// * `language` - New language
    pub fn set_fallback_language(&mut self, language: &str) -> bool {
        if self.languages.contains_key(language) {
            self.fallback = language.to_string();
            true
        } else {
            false
        }
    }

    /// Set the current language for lookups
    ///
    /// # Arguments
    /// * `language` - New language
    ///
    /// returns false if the language code is not recognized
    pub fn set_language(&mut self, language: &str) -> bool {
        if self.languages.contains_key(language) {
            self.current = language.to_string();
            true
        } else {
            false
        }
    }

    /// Look up a string in a specific language
    ///
    /// # Arguments
    /// * `language` - Language to search
    /// * `name` - String to find
    pub fn get_from_lang(&self, language: &str, name: &str) -> Option<&str> {
        if let Some(lang) = self.languages.get(language) {
            lang.get(name)
        } else {
            None
        }
    }

    /// Look up a string
    ///
    /// # Arguments
    /// * `name` - String to find
    pub fn get(&self, name: &str) -> Option<&str> {
        self.current_language()
            .and_then(|l| l.get(name))
            .or(self.fallback_language().and_then(|l| l.get(name)))
    }

    /// Return an embedded resource as a utf8 string
    pub fn utf8_resource(&self, name: &str) -> Option<&str> {
        self.current_language()
            .and_then(|l| l.utf8_resource(name))
            .or(self.fallback_language().and_then(|l| l.utf8_resource(name)))
    }

    /// Return an embedded resource as a slice of bytes
    pub fn binary_resource(&self, name: &str) -> Option<&[u8]> {
        self.current_language()
            .and_then(|l| l.binary_resource(name))
            .or(self
                .fallback_language()
                .and_then(|l| l.binary_resource(name)))
    }
}

impl Index<&str> for LanguageSet {
    type Output = str;

    fn index(&self, name: &str) -> &Self::Output {
        self.get(name).unwrap_or_default()
    }
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate as embedded_lang;
    use crate::embedded_language;

    #[test]
    fn test_current_language() {
        let mut set = LanguageSet::new(
            "fr",
            &[
                embedded_language!("../examples/en.lang.json"),
                embedded_language!("../examples/fr.lang.json"),
            ],
        );

        assert_eq!(set.current_language().unwrap().short_name(), "fr");
        set.set_language("en");
        assert_eq!(set.current_language().unwrap().short_name(), "en");
    }

    #[test]
    fn test_fallback_language() {
        let mut set = LanguageSet::new(
            "fr",
            &[
                embedded_language!("../examples/en.lang.json"),
                embedded_language!("../examples/fr.lang.json"),
            ],
        );

        assert_eq!(set.fallback_language().unwrap().short_name(), "fr");
        set.set_fallback_language("en");
        assert_eq!(set.fallback_language().unwrap().short_name(), "en");
    }

    #[test]
    fn test_add_language() {
        let mut set = LanguageSet::new("fr", &[embedded_language!("../examples/fr.lang.json")]);

        set.add_language(embedded_language!("../examples/en.lang.json"));

        assert_eq!(set.set_language("en"), true);
    }

    #[test]
    fn test_load_language() {
        let mut set = LanguageSet::new("fr", &[embedded_language!("../examples/fr.lang.json")]);

        assert_eq!(
            set.load_language("examples/en.lang.json", HashMap::default())
                .is_ok(),
            true
        );
        assert_eq!(set.set_language("en"), true);
    }

    #[test]
    fn test_set_fallback_language() {
        let mut set = LanguageSet::new(
            "fr",
            &[
                embedded_language!("../examples/en.lang.json"),
                embedded_language!("../examples/fr.lang.json"),
            ],
        );

        assert_eq!(set.set_fallback_language("en"), true);
        assert_eq!(set.fallback_language().unwrap().short_name(), "en");

        assert_eq!(set.set_fallback_language("foo"), false);
        assert_eq!(set.fallback_language().unwrap().short_name(), "en");
    }

    #[test]
    fn test_set_language() {
        let mut set = LanguageSet::new(
            "fr",
            &[
                embedded_language!("../examples/en.lang.json"),
                embedded_language!("../examples/fr.lang.json"),
            ],
        );

        assert_eq!(set.set_language("en"), true);
        assert_eq!(set.current_language().unwrap().short_name(), "en");

        assert_eq!(set.set_language("foo"), false);
        assert_eq!(set.current_language().unwrap().short_name(), "en");
    }

    #[test]
    fn test_get_from_lang() {
        let mut set = LanguageSet::new(
            "fr",
            &[
                embedded_language!("../examples/en.lang.json"),
                embedded_language!("../examples/fr.lang.json"),
            ],
        );
        set.set_fallback_language("en");

        assert_eq!(set.get_from_lang("fr", "tree"), Some("arbre"));
        assert_eq!(set.get_from_lang("fr", "mustard"), None);
        assert_eq!(set.get_from_lang("en", "nope"), None);
    }

    #[test]
    fn test_get() {
        let mut set = LanguageSet::new(
            "fr",
            &[
                embedded_language!("../examples/en.lang.json"),
                embedded_language!("../examples/fr.lang.json"),
            ],
        );
        set.set_fallback_language("en");

        assert_eq!(set.get("tree"), Some("arbre"));
        assert_eq!(set.get("mustard"), Some("mustard"));
        assert_eq!(set.get("nope"), None);
    }

    #[test]
    fn test_index() {
        let mut set = LanguageSet::new(
            "fr",
            &[
                embedded_language!("../examples/en.lang.json"),
                embedded_language!("../examples/fr.lang.json"),
            ],
        );
        set.set_fallback_language("en");

        assert_eq!(set["tree"], "arbre".to_string());
        assert_eq!(set["mustard"], "mustard".to_string());
        assert_eq!(set["nope"], "".to_string());
    }
}
