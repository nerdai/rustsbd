use core::fmt;

use super::{FmtDisplay, Process};
use crate::utils::Rule;
use rustsbd_macros::{FmtDisplay, Process};

#[derive(Debug, FmtDisplay, Process)]
pub struct SingleLetterAbbReplacer {
    rules: [Rule; 2],
}

impl Default for SingleLetterAbbReplacer {
    fn default() -> Self {
        Self {
            rules: [
                Rule::new(
                    r"(?<=^[A-Z])\.(?=\s)",
                    "∯",
                    "SingleUpperCaseLetterAtStartOfLineRule",
                ),
                Rule::new(r"(?<=\s[A-Z])\.(?=,?\s)", "∯", "SingleUpperCaseLetterRule"),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_upper_case_letter_at_start_line_rule() {
        let single_upper_case_letter_at_start_replacer = SingleLetterAbbReplacer::default();
        let mut text = String::from("Q. What is his name?");
        text = single_upper_case_letter_at_start_replacer.process(text);
        assert_eq!(text, "Q∯ What is his name?");
    }

    #[test]
    fn test_single_upper_case_letter_rule() {
        let single_upper_case_letter_replacer = SingleLetterAbbReplacer::default();
        let mut text = String::from("My name is Jonas E. Smith.");
        text = single_upper_case_letter_replacer.process(text);
        assert_eq!(text, "My name is Jonas E∯ Smith.");
    }
}
