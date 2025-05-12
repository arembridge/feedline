use colored::{ColoredString, Colorize};

use crate::status::STATUS;

#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct FeedlineResult {
    pub status: STATUS,
    pub file: String,
    pub message: Option<String>,
}

impl FeedlineResult {
    pub fn get_message_parts(&self) -> Vec<ColoredString> {
        let mut parts: Vec<ColoredString> = Vec::new();
        parts.push(self.status.get_colored_string());
        parts.push(self.file.normal());
        if let Some(message) = &self.message {
            parts.push(message.dimmed());
        };
        parts
    }
}
