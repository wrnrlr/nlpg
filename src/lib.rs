use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use pgx::{prelude::*, warning};
use rust_bert::pipelines::{ner::NERModel,summarization::SummarizationModel,zero_shot_classification::ZeroShotClassificationModel,question_answering::{QaInput, QuestionAnsweringModel}};
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType};
use rust_bert::pipelines::translation::{Language, TranslationModel, TranslationModelBuilder};

pg_module_magic!();

type Handle<M> = Lazy<Mutex<M>>;
static SUMMARIZATION_MODEL: Handle<SummarizationModel> = Lazy::new(||{Mutex::new(SummarizationModel::new(Default::default()).unwrap())});
static ZERO_SHOT_MODEL: Handle<ZeroShotClassificationModel> = Lazy::new(||{Mutex::new(ZeroShotClassificationModel::new(Default::default()).unwrap())});
static NER_MODEL: Handle<NERModel> = Lazy::new(||{Mutex::new(NERModel::new(Default::default()).unwrap())});
static QA_MODEL: Handle<QuestionAnsweringModel> = Lazy::new(||{Mutex::new(QuestionAnsweringModel::new(Default::default()).unwrap())});

/// Translate text from one language to the other
#[pg_extern] pub fn babel(from:&str, to:&str, text:&str) -> String {
    let result = models::get_translation_model(from, to);
    if result.is_err() { warning!("can't find babel model: {:?}", result.as_ref().err()); }
    let translator = result.unwrap();
    translator.translate(text)
}

/// Sentence embeddings
#[pg_extern] pub fn sbert(sentence:&str) -> String {
    let model = models::get_sentence_embeddings_model();
    if model.is_err() { warning!("can't find sbert model: {:?}", model.as_ref().err()); }
    serialize_vector(model.unwrap().encode(sentence))
}

/// Summarize statement
#[pg_extern] pub fn summary(s:&str) -> String { *SUMMARIZATION_MODEL.lock().unwrap().summarize(s).first() || "" }

/// Ask a question
#[pg_extern] pub fn ask(question:String,context:String) -> Answer { *QA_MODEL.lock().unwrap().predict(&[QaInput{question,context}],1,32).first() || "" }
#[derive(PostgresType)] struct Answer { score:f32, start:u32, end:u32, answer:String }

/// Classy a statement
#[pg_extern] pub fn  zero_shot(s:String,labels:Vec<String>) -> LabelWeight {
    *ZERO_SHOT_MODEL.lock().unwrap().predict(&[s],labels,None,128).map(|p|LabelWeight{label:p.label,score:p.score}) || ""
}
#[derive(PostgresType)] struct LabelWeight { label:String,score:f32 }

/// Named Entity Recognition
#[pg_extern] pub fn  ner(s:String) -> Vec<NamedEntity> {
    NER_MODEL.lock().unwrap().predict(&[s]).iter().map(|e|NamedEntity{label:e.label,score:e.score,word:e.word,offset:e.offset}).collect()
}
#[derive(PostgresType)] struct NamedEntity { word:String, score:f32, label:String, offset:u32 }

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
        None => Err("no sentence embedding model found".to_string())
    }
}

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

fn serialize_vector(v:Vec<f32>)->String {
    let mut buffer = ryu::Buffer::new();
    let mut s = String::from('[');
    for (i,f) in v.iter().enumerate() {
        if i != 0 { s.push(','); }
        s.push_str(buffer.format(*f));
    }
    s.push(']');
    s
}

#[cfg(any(test, feature = "pg_test"))] #[pg_schema] mod tests {
    use {pgx::{prelude::*},crate::*};
    #[pg_test] fn babel_translates() { assert_eq!("hello", babel("nl", "en", "hallo")); }
    #[pg_test] fn sbert_sentence_embedding() { assert_eq!("hello", sbert("Good morning")); }
    #[pg_test] fn summarize_text() { assert_eq!("hello", summary("Good morning")); }
    #[pg_test] fn ask_question() { assert_eq!("hello", ask("Good morning".to_string(),"context".to_string())); }
    #[pg_test] fn zero_shot_classification() { assert_eq!("hello", zero_shot("Good morning".to_string(),vec!["".to_string()])); }
    #[pg_test] fn named_entities() { assert_eq!("hello", ner("Good morning".to_string())); }
}

#[cfg(test)] mod tests {
    #[test] fn get_model() { assert!(super::get_translation_model("nl", "en").is_ok()); }
    #[test] fn translate() { let model = super::get_translation_model("nl", "en").unwrap();assert_eq!(model.translate("hallo"), "hello") }
    #[test] fn serialize_vector_test() {
        assert_eq!(super::serialize_vector(vec![]), "[]");
        assert_eq!(super::serialize_vector(vec![0.12345678907]), "[0.12345679]");
        assert_eq!(super::serialize_vector(vec![1.0, 2.0]), "[1.0,2.0]");
    }
}

// This module is required by `cargo pgx test` invocations.  It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> { vec![] }
}
