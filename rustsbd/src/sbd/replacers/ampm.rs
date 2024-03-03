use core::fmt;

use super::{FmtDisplay, Process};
use crate::utils::Rule;
use rustsbd_macros::{FmtDisplay, Process};

#[derive(Debug, FmtDisplay, Process)]
pub struct AmPmReplacer {
    rules: [Rule; 4],
}

impl Default for AmPmReplacer {
    fn default() -> Self {
        Self {
            rules: [
                Rule::new(r"(?<= P∯M)∯(?=\s[A-Z])", ".", "UpperCasePmRule"),
                Rule::new(r"(?<=A∯M)∯(?=\s[A-Z])", ".", "UpperCaseAmRule"),
                Rule::new(r"(?<=p∯m)∯(?=\s[A-Z])", ".", "LowerCasePmRule"),
                Rule::new(r"(?<=a∯m)∯(?=\s[A-Z])", ".", "LowerCaseAmRule"),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_case_pm_rule() {
        let ampm_replacer = AmPmReplacer::default();
        let mut text = String::from(
            "He left at 6 P∯M∯ Travelers who didn't get the warning at 5 p.m. left later.",
        );
        text = ampm_replacer.process(text);
        assert_eq!(
            text,
            "He left at 6 P∯M. Travelers who didn't get the warning at 5 p.m. left later."
        );
    }

    #[test]
    fn test_upper_case_am_rule() {
        let ampm_replacer = AmPmReplacer::default();
        let mut text = String::from(
            "He left at 6 A∯M∯ Travelers who didn't get the warning at 5 a.m. left later.",
        );
        text = ampm_replacer.process(text);
        assert_eq!(
            text,
            "He left at 6 A∯M. Travelers who didn't get the warning at 5 a.m. left later."
        );
    }

    #[test]
    fn test_lower_case_pm_rule() {
        let ampm_replacer = AmPmReplacer::default();
        let mut text = String::from(
            "He left at 6 p∯m∯ Travelers who didn't get the warning at 5 p.m. left later.",
        );
        text = ampm_replacer.process(text);
        assert_eq!(
            text,
            "He left at 6 p∯m. Travelers who didn't get the warning at 5 p.m. left later."
        );
    }

    #[test]
    fn test_lower_case_am_rule() {
        let ampm_replacer = AmPmReplacer::default();
        let mut text = String::from(
            "He left at 6 a∯m∯ Travelers who didn't get the warning at 5 a.m. left later.",
        );
        text = ampm_replacer.process(text);
        assert_eq!(
            text,
            "He left at 6 a∯m. Travelers who didn't get the warning at 5 a.m. left later."
        );
    }
}
