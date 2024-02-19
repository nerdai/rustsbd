struct Sbd;

impl Sbd {
    fn segment(&self, text: String) -> Vec<String> {
        text.split(" ").map(|s| {s.to_string()}).collect()
    }
}

fn main() {
    let sbd = Sbd;

    let result = sbd.segment(String::from("Hi! Hello World"));
    println!("{:?}", result);

}