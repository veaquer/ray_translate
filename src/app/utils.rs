use std::error::Error;

use rust_translate::translate;

#[derive(Debug)]
pub struct Translate<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub text: &'a str,
}

pub fn parse_args(text: &String) -> Translate {
    let parts: Vec<&str> = text.split_whitespace().collect();
    let to_index = parts
        .iter()
        .position(|&x| x == "to")
        .expect("Expected 'to' keyword in the input text");

    let from = parts[0];
    let to = parts[to_index + 1];
    let text = &text[text.find(parts[to_index + 2]).unwrap()..];
    Translate { from, to, text }
}

pub async fn translate_from_args<'a>(args: Translate<'a>) -> Result<String, Box<dyn Error>> {
    return translate(args.from, args.to, args.text).await;
}
