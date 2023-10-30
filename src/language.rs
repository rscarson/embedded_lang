use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

/// Part of a path to a string
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum LanguageStringObject {
    /// A string endpoint
    Direct(String),

    /// Part of a path to an endpoint
    Category(HashMap<String, LanguageStringObject>),
}

impl LanguageStringObject {
    /// Flatten a LanguageStringObject tree into a flat object
    pub fn flatten(&self, own_key: &str) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::default();
        match self {
            LanguageStringObject::Direct(s) => {
                map.insert(own_key.to_string(), s.clone());
            }
            LanguageStringObject::Category(c) => map.extend(Self::flatten_all(c, Some(own_key))),
        };
        map
    }

    fn flatten_all(
        c: &HashMap<String, LanguageStringObject>,
        root_key: Option<&str>,
    ) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::default();
        c.iter().for_each(|e| {
            let key = if root_key.is_some() {
                format!("{}\\{}", root_key.unwrap(), e.0)
            } else {
                e.0.clone()
            };
            map.extend(e.1.flatten(&key))
        });
        map
    }
}

/// Represents a single language lookup instance
#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    name: String,
    short_name: String,
    strings: HashMap<String, LanguageStringObject>,

    #[serde(default)]
    resources: HashMap<String, Vec<u8>>,

    #[serde(skip_serializing, default)]
    attachments: HashMap<String, serde_json::Value>,
}

impl Language {
    /// Create a new language instance
    ///
    /// # Arguments
    /// * `name` - Full language name
    /// * `short_name` - Language code
    /// * `strings` - Language lookup table
    pub fn new(
        name: String,
        short_name: String,
        strings: HashMap<String, LanguageStringObject>,
        resources: HashMap<String, Vec<u8>>,
    ) -> Self {
        Self {
            name,
            short_name,
            strings,
            resources,
            attachments: HashMap::default(),
        }
    }

    /// Attach a document to this language
    pub fn attach<T: Serialize + DeserializeOwned + for<'a> Deserialize<'a>>(
        &mut self,
        name: &str,
        attachment: T,
    ) -> Result<(), serde_json::Error> {
        self.attachments
            .insert(name.to_string(), serde_json::to_value(attachment)?);
        Ok(())
    }

    /// Get an attachment
    pub fn attachment<T: Serialize + DeserializeOwned + for<'a> Deserialize<'a>>(
        &self,
        name: &str,
    ) -> Option<T> {
        if let Some(v) = self.attachments.get(name) {
            serde_json::from_value(v.clone()).ok()
        } else {
            None
        }
    }

    /// Read language from a JSON string
    ///
    /// # Arguments
    /// * `path` - Path to the file
    pub fn new_from_string(
        json: &str,
        resources: HashMap<String, Vec<u8>>,
    ) -> Result<Self, String> {
        match serde_json::from_str::<Self>(json) {
            Ok(mut lang) => {
                lang.resources = resources;
                Ok(lang)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// Read language from a file
    ///
    /// # Arguments
    /// * `path` - Path to the file
    pub fn new_from_file(path: &str, resources: HashMap<String, Vec<u8>>) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(json) => Self::new_from_string(&json, resources),
            Err(e) => Err(e.to_string()),
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
    pub fn strings(&self) -> HashMap<String, String> {
        LanguageStringObject::flatten_all(&self.strings, None)
    }

    /// Look up a string in the given language
    ///
    /// # Arguments
    /// * `name` - String to find
    pub fn get(&self, name: &str) -> Option<&str> {
        let mut path = name.split("\\").peekable();
        if path.peek().is_none() {
            return None;
        }

        let mut pos = self.strings.get(path.next().unwrap());
        for item in path {
            if pos.is_none() {
                return None;
            }
            match pos.unwrap() {
                LanguageStringObject::Direct(s) => return Some(s),
                LanguageStringObject::Category(c) => pos = c.get(item),
            }
        }

        if let Some(pos) = pos {
            match pos {
                LanguageStringObject::Direct(s) => Some(s),
                LanguageStringObject::Category(_) => None,
            }
        } else {
            None
        }
    }

    /// Return an embedded resource as a utf8 string
    pub fn utf8_resource(&self, name: &str) -> Option<&str> {
        self.resources
            .get(name)
            .and_then(|bytes| std::str::from_utf8(&bytes.as_slice()).ok())
    }

    /// Return an embedded resource as a slice of bytes
    pub fn binary_resource(&self, name: &str) -> Option<&[u8]> {
        self.resources
            .get(name)
            .and_then(|bytes| Some(bytes.as_slice()))
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
            let lang = Language::new_from_string(&s, HashMap::default()).unwrap();
            assert_eq!(lang.short_name(), "en");
        }
    }

    #[test]
    fn test_new_from_file() {
        let lang = Language::new_from_file("examples/en.lang.json", HashMap::default()).unwrap();
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
