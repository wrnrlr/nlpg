mod nlp;
pub use nlp::get_model;
use pgx::{prelude::*, info, FATAL};

pg_module_magic!();

#[pg_extern]
fn translate(from:&str, to:&str, text:&str)->String {
    info!("welcome");
    let result = nlp::get_model(from, to);
    if result.is_err() { FATAL!("fatal model") }
    let (source, target, model) = result.unwrap();
    let output = model.translate(&[text], source, target);
    output.unwrap()[0].to_string()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    // use pgx::prelude::*;
    use pgx::{prelude::*, info, FATAL};

    #[test]
    fn test_get_model_without_pg() {
        assert!(crate::get_model("nl","en").is_ok());
    }

    #[pg_test]
    fn test_get_model_with_pg() {
        assert!(crate::get_model("nl","en").is_ok());
    }

    #[pg_test]
    fn test_translate() {
        info!("welcome");
        assert_eq!("hello", crate::translate("nl","en","hallo"));
        FATAL!("goodbye");
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
