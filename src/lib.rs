mod nlp;
pub use nlp::get_model;
use pgx::prelude::*;

pg_module_magic!();

#[pg_schema]
pub mod bert {
    use super::nlp;
    use pgx::{prelude::*, warning};
    #[pg_extern]
    pub fn translate(from:&str, to:&str, text:&str)->String {
        let result = nlp::get_model(from, to);
        if result.is_err() { warning!("can't find model: {:?}", result.as_ref().err()); }
        let translator = result.unwrap();
        translator.translate(text)
        // "TODO".to_string()
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::{prelude::*};

    #[pg_test]
    fn bert_translate() { assert_eq!("hello", crate::bert::translate("nl","en","hallo")); }
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
