use pgx::prelude::*;
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};

pg_module_magic!();

fn string_to_language(s:&str)->Option<Language> {
    match s {
        "en" => Some(Language::English),
        "es" => Some(Language::Spanish),
        "pt" => Some(Language::Portuguese),
        "it" => Some(Language::Italian),
        "ca" => Some(Language::Catalan),
        "de" => Some(Language::German),
        "ru" => Some(Language::Russian),
        "zh" => Some(Language::ChineseMandarin),
        "nl" => Some(Language::Dutch),
        "sv" => Some(Language::Swedish),
        "ar" => Some(Language::Arabic),
        "he" => Some(Language::Hebrew),
        "hi" => Some(Language::Hindi),
        _ => None
    }
}

#[pg_extern]
fn translate(from:&str, to:&str, text:&str)->String {
    let source = string_to_language(from).unwrap();
    let target = string_to_language(to).unwrap();
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![source])
        .with_target_languages(vec![target])
        .create_model().unwrap();
    let output = model.translate(&[text], None, Language::Spanish).unwrap();
    if output.len() == 0 { "".to_string() } else { output[0].to_string() }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_kvk() {
        assert_eq!("hello", crate::translate("nl","en","hallo"));
    }

}

/// This module is required by `cargo pgx test` invocations. 
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
