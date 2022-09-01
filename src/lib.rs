//! A small library to provide translation strings as an embedded resource
//! Language files are in JSON format and will be embedded into the binary at compile time.
//! 
//! Please see the examples directory for language file samples
//! 
//! Usage example:
//! ```rust
//! use embedded_lang::{ LanguageSet, embedded_language, get_string };
//! 
//! fn main() {
//!     let mut translator = LanguageSet::new("fr", &[
//!         embedded_language!("../examples/en.lang.json"),
//!         embedded_language!("../examples/fr.lang.json"),
//!     ]);
//!     translator.set_fallback_language("en");
//! 
//!     assert_eq!(get_string!(translator, "tree"), "arbre".to_string());
//! }
//! ```
//! 
//! LanguageSets have a current language, and a fallback language from which strings will be fetched
//! if the current language is missing the requested string.
#![doc(html_root_url = "https://docs.rs/embedded-lang/0.5.0")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

mod language;
mod language_set;
mod macros;

pub use language::*;
pub use language_set::*;
pub use macros::*;

#[cfg(test)]
mod test_token {
    #[test]
    fn test_readme_deps() {
        version_sync::assert_markdown_deps_updated!("README.md");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }
}