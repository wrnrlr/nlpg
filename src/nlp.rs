use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::{OnceCell,Lazy};
use pgx::{warning};
use pgx::prelude::*;
use rust_bert::pipelines::translation::{Language, TranslationModel, TranslationModelBuilder};
use std::sync::mpsc;
use std::thread;

// type Translator = fn(text:&str)->Result<String, String>;

// lazy_static::lazy_static! {
//     static ref TRANSLATION_MODEL: Mutex<HashMap<(Language,Language),Arc<TranslationModel>>> = {
//         let mut m = HashMap::new();
//         m.insert((Language::Dutch,Language::English),Arc::new(TranslationModelBuilder::new().with_source_languages(vec![Language::Dutch]).with_target_languages(vec![Language::English]).create_model().unwrap()));
//         Mutex::new(m)
//     };
// }

// https://discuss.pytorch.org/t/is-inference-thread-safe/88583


pub struct WrappedTranslationModel(pub TranslationModel);

unsafe impl Send for WrappedTranslationModel {}
unsafe impl Sync for WrappedTranslationModel {}

static REPO: Lazy<Mutex<HashMap<(Language,Language),Arc<WrappedTranslationModel>>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert((Language::Dutch,Language::English),Arc::new(WrappedTranslationModel(TranslationModelBuilder::new().with_source_languages(vec![Language::Dutch]).with_target_languages(vec![Language::English]).create_model().unwrap())));
    Mutex::new(m)
});


// unsafe impl Sync for TranslationModel {}

#[allow(non_snake_case)] #[pg_guard]
pub unsafe extern "C" fn _PG_init() {
    println!("PG_init", );

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // do whatever i want
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

pub struct Translator {
    source:Language,
    target:Language,
    model:Arc<WrappedTranslationModel>,
}

impl Translator {
    pub fn translate(&self, text:&str)->String {
        let output = self.model.0.translate(&[text], self.source, self.target);
        output.unwrap()[0].to_string()
    }
}

pub fn get_model(from:&str, to:&str)->Result<Translator,String> {
    let source = string_to_language(from);
    if source.is_none() { return Err("source language unknown".to_string()) }
    let target = string_to_language(to);
    if target.is_none() { return Err("target language unknown".to_string()) }
    let repo = REPO.lock().unwrap();
    let model = repo.get(&(source.unwrap(),target.unwrap()));
    // This fails for some reason when runing inside a pgx function
    // let model = TranslationModelBuilder::new()
    //     .with_source_languages(vec![source.unwrap()])
    //     .with_target_languages(vec![target.unwrap()])
    //     .create_model();

    match model {
        Some(&ref model) => Ok(Translator{source:source.unwrap(), target:target.unwrap(), model: model.clone() }),
        None => Err("other error".to_string())
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
        let result = model.0.translate(&["hallo"], source, target);
        assert!(result.is_ok())
    }
}