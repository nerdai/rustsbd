use core::fmt;

use super::Process;
use crate::utils::Rule;

#[derive(Debug)]
pub struct NumbersReplacer {
    rules: [Rule; 1],
}

impl Default for NumbersReplacer {
    fn default() -> Self {
        Self {
            rules: [Rule::new(r"\.(?=\d)", "∯", "PeriodBeforeNumberRule")],
        }
    }
}

impl fmt::Display for NumbersReplacer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.rules)
    }
}

impl Process for NumbersReplacer {
    fn rules_vec(&self) -> Vec<Rule> {
        self.rules.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let numbers_replacer = NumbersReplacer::default();
        println!("{:?}", numbers_replacer);
    }

    #[test]
    fn test_period_before_number_rule() {
        let numbers_replacer = NumbersReplacer::default();
        let mut text = String::from("$.50 and .50");
        text = numbers_replacer.process(text);
        assert_eq!(text, "$∯50 and ∯50");
    }
}
