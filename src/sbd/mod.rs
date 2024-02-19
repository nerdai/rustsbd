pub mod replacers;

use replacers::Process;

pub struct Sbd {
    pub pipeline: Vec<Box<dyn Process>>,
}

impl Sbd {
    fn process(&self, text: String) -> String {
        // self.pipeline loop thru all processors and run .process()
        todo!()
    }

    pub fn segment(&self, text: String) -> Vec<String> {
        text.split(" ").map(|s| {s.to_string()}).collect()
    }

    pub fn get_sentences(&self, text: String) -> Vec<String> {
        let process_text = self.process(text);
        self.segment(process_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::replacers::{AbbreviationReplacer, ListItemReplacer};

    #[test]
    fn it_works() {
        let sbd = Sbd {
            pipeline: vec![
                Box::new(ListItemReplacer {}),
                Box::new(AbbreviationReplacer {}),
            ],
        };

        assert!(true);
    }

    #[test]
    fn segment_function_works() {
        let sbd = Sbd {
            pipeline: vec![
                Box::new(ListItemReplacer {}),
                Box::new(AbbreviationReplacer {}),
            ],
        };

        let result = sbd.segment(String::from("Hi! Hello World"));
        assert_eq!(result, vec!["Hi!", "Hello", "World"]);
    }
}
