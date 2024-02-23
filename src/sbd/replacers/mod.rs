pub mod abbreviation;
pub mod list_item;
pub mod numbers;

use crate::utils::Rule;
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
