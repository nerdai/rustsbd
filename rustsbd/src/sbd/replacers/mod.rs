pub mod abbreviation;
pub mod ampm;
pub mod list_item;
pub mod numbers;
pub mod singleletterabb;

use crate::utils::Rule;
use core::fmt;
use fancy_regex::Regex;

pub trait Process {
    fn rules_vec(&self) -> Vec<Rule>;

    fn process(&self, mut text: String) -> String {
        for rule in self.rules_vec().iter() {
            let re = Regex::new(rule.pattern()).unwrap();
            let result = re.replace_all(&text[..], rule.replacement());
            text = String::from(result);
        }
        text
    }
}

pub trait FmtDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
