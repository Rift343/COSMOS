use std::io::Read;
use semantic_parser::test;

fn main() {
    let mut out_file = test();

    let mut temp: String = String::new();

    println!("Pre-finished");

    out_file.read_to_string(& mut temp).expect("");

    println!("{}", temp);

    println!("Finished !")
}