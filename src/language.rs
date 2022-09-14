use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single language lookup instance
#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    name: String,
    short_name: String,
    strings: HashMap::<String, String>
}

impl Language {
	/// Create a new language instance
	/// 
	/// # Arguments
	/// * `name` - Full language name
	/// * `short_name` - Language code
	/// * `strings` - Language lookup table
    pub fn new(name: String, short_name: String, strings: HashMap::<String, String>) -> Self {
        Self { name, short_name, strings }
    }

	/// Read language from a JSON string
	/// 
	/// # Arguments
	/// * `path` - Path to the file
    pub fn new_from_string(json: &str) -> Result<Self, String> {
        match serde_json::from_str(json) {
			Ok(lang) => Ok(lang),
			Err(e) => Err(e.to_string())
		}
    }

	/// Read language from a file
	/// 
	/// # Arguments
	/// * `path` - Path to the file
    pub fn new_from_file(path: &str) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
			Ok(json) => {
                Self::new_from_string(&json)
			},
			Err(e) => {
				Err(e.to_string())
			}
		}
    }

    /// Get full language name 
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get language code 
    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    /// Get language lookup table 
    pub fn strings(&self) -> &HashMap::<String, String> {
        &self.strings
    }

    /// Look up a string in the given language
	/// 
	/// # Arguments
	/// * `name` - String to find
    pub fn get(&self, name: &str) -> Option<&str> {
        match self.strings.get(name) {
            None => None,
            Some(s) => Some(s.as_str())
        }
    }
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate as embedded_lang;
    use crate::embedded_language;

    #[test]
    fn test_new_from_string() {
        if let Ok(s) = std::fs::read_to_string("examples/en.lang.json") {
			let lang = Language::new_from_string(&s).unwrap();
            assert_eq!(lang.short_name(), "en");
		}
    }

    #[test]
    fn test_new_from_file() {
        let lang = Language::new_from_file("examples/en.lang.json").unwrap();
        assert_eq!(lang.short_name(), "en");
    }

    #[test]
    fn test_short_name() {
        let lang = embedded_language!("../examples/en.lang.json");
        assert_eq!(lang.short_name(), "en");
    }

    #[test]
    fn test_name() {
        let lang = embedded_language!("../examples/en.lang.json");
        assert_eq!(lang.name(), "English");
    }

    #[test]
    fn test_get() {
        let lang = embedded_language!("../examples/en.lang.json");
        
        assert_eq!(lang.get("hello_msg"), Some("hello world!"));
        assert_eq!(lang.get("goodbye_msg"), None);
    }
}