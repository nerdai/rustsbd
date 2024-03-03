use core::fmt;

#[derive(Clone, Default, Debug)]
pub struct Rule {
    pattern: String,
    replacement: String,
    name: String,
}

impl Rule {
    pub fn new(pattern: &str, replacement: &str, name: &str) -> Self {
        Self {
            pattern: String::from(pattern),
            replacement: String::from(replacement),
            name: String::from(name),
        }
    }

    // Immutable access.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
    pub fn replacement(&self) -> &str {
        &self.replacement
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pattern, self.replacement)
    }
}
