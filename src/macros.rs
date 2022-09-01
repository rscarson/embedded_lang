/// Get a language as an embedded resource
/// 
/// # Arguments
/// * `filename` - Path to the file to embed
#[macro_export]
macro_rules! embedded_language {
    ($filename:literal) => {
        embedded_lang::Language::new_from_string( include_str!($filename) ).unwrap()
    };
}

/// Get a language string
/// 
/// # Arguments
/// * `filename` - Path to the file to embed
#[macro_export]
macro_rules! get_string {
    ($set:ident, $name:literal) => {
        $set.get($name).unwrap_or_default()
    };
}

#[cfg(test)]
mod test_token {
    use crate as embedded_lang;
    use crate::LanguageSet;

    #[test]
    fn test_embedded_language() {
        let lang = embedded_language!("../examples/en.lang.json");
        assert_eq!(lang.get("hello_msg"), Some("hello world!".to_string()));
    }

    #[test]
    fn test_get_string() {
        let mut set = LanguageSet::new("fr", &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ]);
        set.set_fallback_language("en");

        assert_eq!(get_string!(set, "foobar"), "".to_string());
        assert_eq!(get_string!(set, "mustard"), "mustard".to_string());
    }
}