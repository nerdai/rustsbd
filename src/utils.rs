use core::fmt;

#[derive(Default, Debug)]
pub struct Rule {
    pattern: String,
    replacement: String,
}

impl Rule {
    pub fn new(pattern: &str, replacement: &str) -> Self {
        Self {
            pattern: String::from(pattern),
            replacement: String::from(replacement),
        }
    }

    // Immutable access.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pattern, self.replacement)
    }
}
    
