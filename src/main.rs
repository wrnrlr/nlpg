fn main() {
    println!("welcome");
    let text = "hallo";
    let from = "nl";
    let to = "en";
    let result = pg_nlp::get_model(from, to);
    let (source, target, model) = result.unwrap();
    println!("{} {}", source, target);
    let output = model.translate(&[text], source, target);
    println!("{}", output.unwrap()[0].to_string());
}