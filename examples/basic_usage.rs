use embedded_lang::{embedded_language, get_string, LanguageSet};

fn main() {
    let mut translator = LanguageSet::new(
        "fr",
        &[
            embedded_language!("../examples/en.lang.json"),
            embedded_language!("../examples/fr.lang.json"),
        ],
    );
    translator.set_fallback_language("en");

    assert_eq!(get_string!(translator, "tree"), "arbre".to_string());

    assert_eq!(
        get_string!(translator, "category\\category2\\foo"),
        "bar".to_string()
    );
}
