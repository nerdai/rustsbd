use rustsbd::sbd::Sbd;

fn main() {
    let sbd = Sbd{pipeline: vec![]};

    let result = sbd.segment(String::from("Hi! Hello World"));
    println!("{:?}", result);

}