/* TO BE DELETED WHEN ENGINE IS DONE */

use std::fs::File;
use std::io::Read;
use syntaxic_parser;

pub fn main() {
    let mut syntaxic_parsing_handle : File = syntaxic_parser::syntaxic_parser("SELECT Id, Age, Salaire FROM Personne;".to_string());

    println!("Handle : {:?}",syntaxic_parsing_handle);

    let mut syntaxic_parsing_content = String::new();

    syntaxic_parsing_handle.read_to_string(&mut syntaxic_parsing_content).expect("Error: Unable to read syntaxic parsing file");

    println!("Read Result : {}",syntaxic_parsing_content);
}