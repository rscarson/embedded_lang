# embedded_lang: Embedded language resources for rust applications
[![Crates.io](https://img.shields.io/crates/v/embedded_lang.svg)](https://crates.io/crates/embedded_lang)
[![Build Status](https://github.com/rscarson/embedded_lang/workflows/Rust/badge.svg)](https://github.com/rscarson/embedded_lang/actions?workflow=Rust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/embedded_lang/master/LICENSE)

A small library to provide translation strings as an embedded resource
Language files are in JSON format and will be embedded into the binary at compile time.

Please see the examples directory for language file samples

Usage example:
```rust
use embedded_lang::{ LanguageSet, embedded_language, get_string };

fn main() {
    let mut translator = LanguageSet::new("fr", &[
        embedded_language!("../examples/en.lang.json"),
        embedded_language!("../examples/fr.lang.json"),
    ]);
    translator.set_fallback_language("en");

    assert_eq!(get_string!(translator, "tree"), "arbre".to_string());
}
```

LanguageSets have a current language, and a fallback language from which strings will be fetched
if the current language is missing the requested string.