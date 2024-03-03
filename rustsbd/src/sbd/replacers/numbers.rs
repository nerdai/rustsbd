use super::{FmtDisplay, Process};
use crate::utils::Rule;
use core::fmt;
use rustsbd_macros::{FmtDisplay, Process};

#[derive(Debug, FmtDisplay, Process)]
pub struct NumbersReplacer {
    rules: [Rule; 5],
}

impl Default for NumbersReplacer {
    fn default() -> Self {
        Self {
            rules: [
                Rule::new(r"\.(?=\d)", "∯", "PeriodBeforeNumberRule"),
                Rule::new(r"(?<=\d)\.(?=\S)", "∯", "NumberAfterPeriodBeforeLetterRule"),
                Rule::new(
                    r"(?<=\r\d)\.(?=(\s\S)|\))",
                    "∯",
                    "NewLineNumberPeriodSpaceLetterRule",
                ),
                Rule::new(r"(?<=^\d)\.(?=(\s\S)|\))", "∯", "StartLineNumberPeriodRule"),
                Rule::new(
                    r"(?<=^\d\d)\.(?=(\s\S)|\))",
                    "∯",
                    "StartLineTwoDigitNumberPeriodRule",
                ),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_period_before_number_rule() {
        let numbers_replacer = NumbersReplacer::default();
        let mut text = String::from("$.50 and .50");
        text = numbers_replacer.process(text);
        assert_eq!(text, "$∯50 and ∯50");
    }

    #[test]
    fn test_number_after_period_before_letter_rule() {
        let numbers_replacer = NumbersReplacer::default();
        let mut text = String::from("$3.test");
        text = numbers_replacer.process(text);
        assert_eq!(text, "$3∯test");
    }

    #[test]
    fn test_new_line_number_period_space_letter_rule() {
        let numbers_replacer = NumbersReplacer::default();
        let mut text = String::from("\r1. Hello");
        text = numbers_replacer.process(text);
        assert_eq!(text, "\r1∯ Hello");
    }

    #[test]
    fn test_start_line_number_period_rule() {
        let numbers_replacer = NumbersReplacer::default();
        let mut text = String::from("1. Hello");
        text = numbers_replacer.process(text);
        assert_eq!(text, "1∯ Hello");
    }

    #[test]
    fn test_start_line_two_digit_number_period_rule() {
        let numbers_replacer = NumbersReplacer::default();
        let mut text = String::from("01. Hello");
        text = numbers_replacer.process(text);
        assert_eq!(text, "01∯ Hello");
    }
}
