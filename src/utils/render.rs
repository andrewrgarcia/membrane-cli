use colored::{Colorize, ColoredString};

pub fn render_key_value(key: &str, value: &str) -> (ColoredString, ColoredString) {
    if key.starts_with('_') {
        (
            key.bright_black(),
            value.bright_black(),
        )
    } else {
        (key.normal(), value.normal())
    }
}
