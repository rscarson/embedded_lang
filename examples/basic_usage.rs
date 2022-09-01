use embedded_lang::{ LanguageSet, embedded_language, get_string };

fn main() {
    let mut translator = LanguageSet::new("fr", &[
        embedded_language!("../examples/en.lang.json"),
        embedded_language!("../examples/fr.lang.json"),
    ]);
    translator.set_fallback_language("en");

    assert_eq!(get_string!(translator, "tree"), "arbre".to_string());
}