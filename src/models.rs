use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::{Lazy};
use rust_bert::pipelines::translation::{Language, TranslationModel, TranslationModelBuilder};
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder,SentenceEmbeddingsModel,SentenceEmbeddingsModelType};

// type Translator = fn(text:&str)->Result<String, String>;
// https://discuss.pytorch.org/t/is-inference-thread-safe/88583

pub struct WrappedTranslationModel(pub TranslationModel);
unsafe impl Send for WrappedTranslationModel {}
unsafe impl Sync for WrappedTranslationModel {}

static TRANSLATION_MODELS: Lazy<Mutex<HashMap<(Language, Language),Arc<WrappedTranslationModel>>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert((Language::Dutch,Language::English),Arc::new(WrappedTranslationModel(TranslationModelBuilder::new().with_source_languages(vec![Language::Dutch]).with_target_languages(vec![Language::English]).create_model().unwrap())));
    Mutex::new(m)
});

// #[allow(non_snake_case)] #[pg_guard]
// pub unsafe extern "C" fn _PG_init() {}

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

pub struct Translator {
    source:Language,
    target:Language,
    model:Arc<WrappedTranslationModel>,
}

impl Translator {
    pub fn translate(&self, text:&str)->String {
        let output = self.model.0.translate(&[text], self.source, self.target);
        output.unwrap()[0].trim().to_string()
    }
}

pub fn get_translation_model(from:&str, to:&str)->Result<Translator,String> {
    let source = string_to_language(from);
    if source.is_none() { return Err("source language unknown".to_string()) }
    let target = string_to_language(to);
    if target.is_none() { return Err("target language unknown".to_string()) }
    let repo = TRANSLATION_MODELS.lock().unwrap();
    match repo.get(&(source.unwrap(),target.unwrap())) {
        Some(&ref model) => Ok(Translator{source:source.unwrap(), target:target.unwrap(), model: model.clone() }),
        None => Err("other error".to_string())
    }
}

pub struct MySentenceEmbeddingsModel(pub SentenceEmbeddingsModel);
unsafe impl Send for MySentenceEmbeddingsModel {}
unsafe impl Sync for MySentenceEmbeddingsModel {}
impl MySentenceEmbeddingsModel {
    pub fn encode(&self, text:&str)->Vec<f32> {
        self.0.encode(&[text]).unwrap()[0].clone()
    }
}

static SENTENCE_EMBEDDINGS_MODELS: Lazy<Mutex<HashMap<String,Arc<MySentenceEmbeddingsModel>>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("AllMiniLmL12V2".to_string(),Arc::new(MySentenceEmbeddingsModel(SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model().unwrap())));
    Mutex::new(m)
});

pub fn get_sentence_embeddings_model()->Result<Arc<MySentenceEmbeddingsModel>,String> {
    let repo = SENTENCE_EMBEDDINGS_MODELS.lock().unwrap();
    match repo.get(&"AllMiniLmL12V2".to_string()) {
        Some(&ref model) => Ok(model.clone()),
        None => Err("no sentence embedding model error".to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_model() {
        assert!(super::get_translation_model("nl", "en").is_ok());
    }

    #[test]
    fn translate() {
        let model = super::get_translation_model("nl", "en").unwrap();
        assert_eq!(model.translate("hallo"), "hello")
    }
}