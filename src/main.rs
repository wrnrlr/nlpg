fn main() {
    let (source, target, model) = pg_nlp::get_model("nl", "en").unwrap();
    let output = model.translate(&["hallo"], source, target);
    println!("{}", output.unwrap()[0].to_string());
}