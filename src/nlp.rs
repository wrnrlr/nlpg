use rust_bert::pipelines::translation::{Language, TranslationModel, TranslationModelBuilder};
use rust_bert::RustBertError;

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

pub fn get_model(from:&str, to:&str)->Result<(Language, Language, TranslationModel),String> {
    let source = string_to_language(from);
    if source.is_none() { return Err("source language unknown".to_string()) }
    let target = string_to_language(to);
    if target.is_none() { return Err("target language unknown".to_string()) }
    // This fails for some reason when runing inside a pgx function
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![source.unwrap()])
        .with_target_languages(vec![target.unwrap()])
        .create_model();
    if let Some(err) = &model.as_ref().err() { return Err("can't create model".to_string()) }
    Ok((source.unwrap(), target.unwrap(), model.unwrap()))
}