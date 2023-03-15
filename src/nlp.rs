use pgx::{warning};
use rust_bert::pipelines::translation::{Language, TranslationModel, TranslationModelBuilder};

// type Translator = fn(text:&str)->Result<String, String>;

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
    match model {
        Ok(model) => Ok((source.unwrap(), target.unwrap(), model)),
        Err(e) => Err(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_model() {
        assert!(crate::get_model("nl", "en").is_ok());
    }

    #[test]
    fn tranlation() {
        let (source, target, model) = crate::get_model("nl", "en").unwrap();
        let result = model.translate(&["hallo"], source, target);
        assert!(result.is_ok())
    }
}