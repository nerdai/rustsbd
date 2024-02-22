pub mod sbd;
pub mod utils;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::Rule;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn private_fields() {
        let rule = Rule::new(
            &String::from("Hello")[..],
            &String::from("World")[..]
        );
        assert_eq!(rule.pattern(), "Hello");
        assert_eq!(rule.replacement(), "World")
    }
}
