use core::fmt;

use super::Process;
use crate::utils::Rule;

#[derive(Debug)]
pub struct NumbersReplacer {
    rules: [Rule; 2]
}

impl Default for NumbersReplacer {
    fn default() -> Self {
        Self { rules: [
            Rule::new("Hello", "World!"),
            Rule::new("Marcus", "Nolan")
        ] }
    }
}

impl fmt::Display for NumbersReplacer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.rules)
    }
}

impl Process for NumbersReplacer {
    fn process(&self, mut text: String) -> String {
        todo!()
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
}