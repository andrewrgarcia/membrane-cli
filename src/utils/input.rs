use std::io::{self, Read};
use colored::Colorize;

pub fn read_multiline(prompt: &str) -> anyhow::Result<String> {
    println!("{}", prompt.bright_cyan());
    println!("{}", "↪ Finish with Ctrl+D (Linux/macOS) or Ctrl+Z then Enter (Windows).".white());
    println!("{}", "↪ Press Ctrl+C to cancel.".white());

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() {
        anyhow::bail!("No content provided.");
    }

    Ok(buffer)
}
