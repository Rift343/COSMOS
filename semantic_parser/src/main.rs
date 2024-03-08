use std::fs::File;
use std::io::Read;
use semantic_parser::semantic_parser;

/// Main function of the semantic parser, allows testing of the library as a binary
fn main() {
    let fs1_filename = String::from("data/SemanticTestData/FS_1.json");
    let in_file = File::options().read(true).write(true).create(true).open(fs1_filename).expect("Erreur lors de création de out_file");


    let out_file = semantic_parser(in_file);

    let mut temp: String = String::new();

    println!("Pre-finished");

    out_file.expect("REASON").read_to_string(& mut temp).expect("");

    println!("{}", temp);

    println!("Finished !")
}