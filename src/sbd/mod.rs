pub struct SBD {}

impl SBD {
    pub fn split(&self, text: String) -> Vec<String> {
        vec![text]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sbd = SBD {};
        let result = sbd.split(String::from("Hi!"));
        println!("{:?}", result)
    }
}
