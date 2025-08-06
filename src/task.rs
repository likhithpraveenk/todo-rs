use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub done: bool,
    pub created_at: DateTime<Local>,
    pub modified_at: DateTime<Local>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done {
            "✔".green()
        } else {
            "•".red()
        };
        let id = format!("{}.", self.id).bold().blue();
        let text = if self.done {
            self.text.strikethrough().dimmed()
        } else {
            self.text.normal()
        };

        let created = self.created_at.format("%b %d, %Y at %H:%M").to_string();
        let modifed = self.modified_at.format("%b %d, %Y at %H:%M").to_string();

        write!(
            f,
            "{} [{}] {}\n   {} {}\n   {} {}",
            id,
            status,
            text,
            "Created  :".bright_black(),
            created.bright_white(),
            "Modified :".bright_black(),
            modifed.bright_white(),
        )
    }
}
