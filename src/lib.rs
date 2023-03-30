mod models;

use pgx::prelude::*;

pg_module_magic!();

#[pg_schema]
pub mod nlp {
    use super::{models,serialize_vector};
    use pgx::{prelude::*, warning};
    #[pg_extern]
    pub fn translate(from:&str, to:&str, text:&str)->String {
        let result = models::get_translation_model(from, to);
        if result.is_err() { warning!("can't find model: {:?}", result.as_ref().err()); }
        let translator = result.unwrap();
        translator.translate(text)
    }

    #[pg_extern]
    pub fn sentence_embeddings(sentence:&str)->String {
        let model = models::get_sentence_embeddings_model();
        if model.is_err() { warning!("can't find model: {:?}", model.as_ref().err()); }
        serialize_vector(model.unwrap().encode(sentence))
    }
}

pub fn serialize_vector(v:Vec<f32>)->String {
    let mut buffer = ryu::Buffer::new();
    let mut s = String::from('[');
    for (i,f) in v.iter().enumerate() {
        if i != 0 { s.push(','); }
        s.push_str(buffer.format(*f));
    }
    s.push(']');
    s
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::{prelude::*};

    #[pg_test]
    fn bert_translate() { assert_eq!("hello", crate::nlp::translate("nl", "en", "hallo")); }

    #[test]
    fn serialize_vector_test() {
        assert_eq!(crate::serialize_vector(vec![]), "[]");
        assert_eq!(crate::serialize_vector(vec![0.12345678907]), "[0.12345679]");
        assert_eq!(crate::serialize_vector(vec![1.0, 2.0]), "[1.0,2.0]");
    }
}

/// This module is required by `cargo pgx test` invocations. 
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> { vec![] }
}
