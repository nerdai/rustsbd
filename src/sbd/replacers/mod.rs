pub trait Process {
    fn process(&self, text: String) -> String;
}

pub struct ListItemReplacer {}

impl Process for ListItemReplacer {
    fn process(&self, mut text: String) -> String {
        todo!()
    }
}

pub struct AbbreviationReplacer {}

impl Process for AbbreviationReplacer {
    fn process(&self, text: String) -> String {
        todo!()
    }
}