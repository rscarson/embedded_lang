use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Language;

/// A searchable set of language string instances
#[derive(Serialize, Deserialize, Clone)]
pub struct LanguageSet {
    current: String,
    fallback: String,
    languages: HashMap::<String, Language>
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
            languages: languages.iter().map(|l| (l.short_name().to_string(), l.clone())).collect()
        }
    }

    /// Return the set's fallback language
    pub fn fallback_language(&self) -> &str {
        &self.fallback
    }

    /// Return the set's current language
    pub fn current_language(&self) -> &str {
        &self.current
    }

    /// Add a language to the set
	/// 
	/// # Arguments
	/// * `language` - New language
    pub fn add_language(&mut self, language: Language) {
        self.languages.insert(language.short_name().to_string(), language);
    }

    /// Add a language from a JSON file to the set
	/// 
	/// # Arguments
	/// * `language` - New language
    pub fn load_language(&mut self, filename: &str) -> Result<(), String> {
        match Language::new_from_file(filename) {
            Ok(lang) => {
                self.add_language(lang);
                Ok(())
            },
            Err(e) => Err(e)
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
    pub fn get_from_lang(&self, language: &str, name: &str) -> Option<String> {
        if let Some(lang) = self.languages.get(language) {
            if let Some(s) = lang.get(name) {
                return Some(s)
            }
        }

        None
    }

    /// Look up a string
	/// 
	/// # Arguments
	/// * `name` - String to find
    pub fn get(&self, name: &str) -> Option<String> {
        if let Some(s) = self.get_from_lang(&self.current, name) {
            return Some(s)
        }
        
        if let Some(s) = self.get_from_lang(&self.fallback, name) {
            return Some(s)
        }

        None
    }
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate as embedded_lang;
    use crate::embedded_language;

    #[test]
    fn test_current_language() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ]);
        
        assert_eq!(set.current_language(), "fr");
        set.set_language("en");
        assert_eq!(set.current_language(), "en");
    }

    #[test]
    fn test_fallback_language() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ]);
        
        assert_eq!(set.fallback_language(), "fr");
        set.set_fallback_language("en");
        assert_eq!(set.fallback_language(), "en");
    }

    #[test]
    fn test_add_language() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/fr.lang.json"),
        ]);

        set.add_language(
            embedded_language!("../examples/en.lang.json")
        );

        assert_eq!(set.set_language("en"), true);
    }

    #[test]
    fn test_load_language() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/fr.lang.json"),
        ]);

        assert_eq!(set.load_language("examples/en.lang.json").is_ok(), true);
        assert_eq!(set.set_language("en"), true);
    }

    #[test]
    fn test_set_fallback_language() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ]);
        
        assert_eq!(set.set_fallback_language("en"), true);
        assert_eq!(set.fallback_language(), "en");
        
        assert_eq!(set.set_fallback_language("foo"), false);
        assert_eq!(set.fallback_language(), "en");
    }

    #[test]
    fn test_set_language() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ]);
        
        assert_eq!(set.set_language("en"), true);
        assert_eq!(set.current_language(), "en");
        
        assert_eq!(set.set_language("foo"), false);
        assert_eq!(set.current_language(), "en");
    }

    #[test]
    fn test_get_from_lang() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ]);
        set.set_fallback_language("en");

        assert_eq!(set.get_from_lang("fr", "tree"), Some("arbre".to_string()));
        assert_eq!(set.get_from_lang("fr", "mustard"), None);
        assert_eq!(set.get_from_lang("en", "nope"), None);
    }

    #[test]
    fn test_get() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ]);
        set.set_fallback_language("en");

        assert_eq!(set.get("tree"), Some("arbre".to_string()));
        assert_eq!(set.get("mustard"), Some("mustard".to_string()));
        assert_eq!(set.get("nope"), None);
    }
}