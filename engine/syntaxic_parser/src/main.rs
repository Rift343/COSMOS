/* TO BE DELETED WHEN ENGINE IS DONE 

use serde_json::Value;
use std::fs::File;
use std::io::Read;
use syntaxic_parser;
*/
pub fn main() {
    // Call the syntaxic parser and get file handle for the syntaxic parsing file
    /*
        let mut syntaxic_parsing_handle : File = syntaxic_parser::syntaxic_parser("SELECT * FROM Personne;".to_string());

        println!("Handle : {:?}",syntaxic_parsing_handle);

        // Read the file and pour its contents into a String
        let mut syntaxic_parsing_content = String::new();

        syntaxic_parsing_handle.read_to_string(&mut syntaxic_parsing_content).expect("Error: Unable to read syntaxic parsing file");

        // Convert to a serde_json Value type
        let parsing_value : Value = serde_json::from_str(&*syntaxic_parsing_content).expect("Error: Unable to turn JSON file into Value type");

        // Show "status" and "error" fields
        println!("Status : {}\nError : {}\n",parsing_value["status"], parsing_value["error"]);
    */
}
