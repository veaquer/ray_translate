use core::str;
use std::process::{Command, Output};

use egui::{RichText, Ui};
const TEXT_SIZE: f32 = 24.;

#[derive(Debug)]
pub struct Translate<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub text: &'a str,
}

pub fn parse_args(text: &String) -> Result<Translate, &str> {
    let parts: Vec<&str> = text.split_whitespace().collect();
    let to_index = match parts.iter().position(|&x| x == "to") {
        Some(index) => index,
        None => return Err("Please, use 'lang to lang text' format."),
    };
    if parts.len() < to_index + 3 {
        return Err("Please, use 'lang to lang text' format.");
    }
    let from = parts[0];
    let to = parts[to_index + 1];
    let text = &text[text.find(parts[to_index + 2]).unwrap()..];
    Ok(Translate { from, to, text })
}

pub fn translate(tr: Translate) -> Result<String, String> {
    // Construct the command to run `translate-shell`
    let output: Output = Command::new("trans")
        .arg(format!("{}:{}", tr.from, tr.to))
        .arg(tr.text)
        .output()
        .map_err(|e| format!("Failed to execute process: {}.\nPlease, make sure that you have installed translate-shell.", e))?;

    // Check if the command was successful
    if output.status.success() {
        // Convert the output to a string
        let translated_text = str::from_utf8(&output.stdout)
            .map_err(|e| format!("Failed to convert output to string: {}", e))?;
        Ok(translated_text.to_string())
    } else {
        // Capture the error message
        let error_message = str::from_utf8(&output.stderr)
            .map_err(|e| format!("Failed to convert error message to string: {}", e))?;
        Err(error_message.to_string())
    }
}
pub fn render_ansi_text(ui: &mut Ui, text: &str) {
    if !text.contains('\x1b') {
        // If no ANSI codes are detected, display the plain text
        ui.label(RichText::new(text).size(TEXT_SIZE));
        return;
    }

    let mut is_bold = false;
    let mut is_underline = false;
    let mut buffer = String::new();

    let mut parts = text.split('\x1b').peekable();

    while let Some(part) = parts.next() {
        if part.is_empty() {
            continue;
        }

        if let Some((code, rest)) = part.split_once('m') {
            match code {
                "[1" => is_bold = true,
                "[22" => is_bold = false,
                "[4" => is_underline = true,
                "[24" => is_underline = false,
                _ => {}
            }

            buffer.push_str(rest);
        } else {
            buffer.push_str(part);
        }

        if parts.peek().is_none() {
            let mut rich_text = RichText::new(&buffer).size(TEXT_SIZE);

            if is_bold {
                rich_text = rich_text.strong();
            }

            if is_underline {
                rich_text = rich_text.underline();
            }

            ui.label(rich_text);
            buffer.clear();
        }
    }
}
