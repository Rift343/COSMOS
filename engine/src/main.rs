use std::fs::File;
use std::io::Read;
use serde_json::Value;


use syntaxic_parser::syntaxic_parser;
use runner_scheduler::scheduler;
use semantic_parser::semantic_parser;
use semantic_parser::structures::semantic_parser_file::SemanticParserFile;
use engine::csv_to_string;

fn main() {
    /*

    Basic plan :

    TODO : Call view for request

    TODO : Transfer request to syntaxic parser

    TODO : Verify output

    TODO : Transfer its result to semantic parser

    Verify output

    TODO : Transfer to runner / scheduler

    TODO : Transfer output to view for display

    ----
    Later make it loop, once everyone has contributed

     */

    // -----------------------------------------------------
    // ------------------ Syntaxic Parser ------------------
    // ----------------------- Start -----------------------
    // -----------------------------------------------------

    // Get query, static for now, should get from view later (request_receiver)
    let sql_query : String = "SELECT Id, Nom, Prenom FROM Personne;".to_string();

    // Call the syntaxic parser and get file handle for the syntaxic parsing file
    let mut syntaxic_parsing_handle : File = syntaxic_parser(sql_query);

    // Read the file and put its contents into a String
    let mut syntaxic_parsing_content = String::new();
    syntaxic_parsing_handle.read_to_string(&mut syntaxic_parsing_content).expect("Error: Unable to read syntaxic parsing file");

    // Convert to a serde_json Value type
    let parsing_value : Value = serde_json::from_str(&*syntaxic_parsing_content).expect("Error: Unable to turn JSON String into Value type");

    // Show "status" and "error" fields
    println!("Status : {}\nError : {}\n",parsing_value["status"], parsing_value["error"]);
    if parsing_value["satus"]=="false" {
        // Print for now, should send to the view later (result_printer)
        println!("{}",parsing_value["error"]);
    }
    else {
        // Print for now, should be given to the semantic parser later
        println!("{:?}",syntaxic_parsing_handle);
    }

    // -----------------------------------------------------
    // ------------------ Syntaxic Parser ------------------
    // ------------------------ End ------------------------
    // -----------------------------------------------------

    // -----------------------------------------------------
    // ------------------ Semantic Parser ------------------
    // ----------------------- Start -----------------------
    // -----------------------------------------------------

    // Mock syntaxic file, replace these variables when done
    let syntaxic_file_name = "data/SemanticTestData/FS_1.json".to_string();
    let syntaxic_file = File::options().read(true).open(syntaxic_file_name).expect("ENGINE :\tError occurred whilst attempting to open syntaxic file input");

    // Get the outputted semantic file.
    let mut semantic_file = semantic_parser(syntaxic_file);


    // Extract the file contents to a string first, then to a structure so that we may examine its fields.
    let semantic_file_content_as_struct: SemanticParserFile = {
        let mut semantic_file_contents_as_string = String::new();


        match semantic_file.read_to_string(&mut semantic_file_contents_as_string)
        {
            Ok(_) => (),
            Err(error) => panic!("ENGINE :\tError occurred whilst reading semantic parser file output\n{}", error)
        }

        match serde_json::from_str(semantic_file_contents_as_string.as_str()) {
            Ok(content) => {
                content
            }
            Err(error) => panic!("ENGINE :\tError occurred whilst parsing String to a structure\n{}", error)
        }
    };

    // Print the results for a feedback, may be removed when judged necessary
    println!("{:?}", semantic_file_content_as_struct);
    println!("{:?}\t{:?}", semantic_file_content_as_struct.status, semantic_file_content_as_struct.error);

    // -----------------------------------------------------
    // ------------------ Semantic Parser ------------------
    // ------------------------ End ------------------------
    // -----------------------------------------------------

    // -----------------------------------------------------
    // ------------------ Runner_scheduler ------------------
    // ----------------------- Start -----------------------
    // -----------------------------------------------------

    let csv_file_returned = scheduler(&semantic_file);
    match csv_file_returned {
        Ok(content) => {
            let printable_string;
            printable_string=csv_to_string(&content);
            match printable_string {
                Ok(content) => println!("{}",content),
                Err(_) => println!("
                    -----------------------------------------------------
                    ---------------------Engine--------------------------
                    ---------------------Error 1--------------------------
                    -----------------------------------------------------
                    Maybe CSV file is already used or not existe anymore.
                    Please check the data/CSV directory
                    "),
            }


        },//Case 1, we have a CSV file so CSV_to_string then result_printer
        Err(_) => {println!("
        -----------------------------------------------------
        -----------------Runner_scheduler--------------------
        ---------------------Error 2-------------------------
        -----------------------------------------------------
        Maybe CSV file is already used or not existe anymore.
        Please check the data/CSV directory
        ");},//Case2, print there is a error on a file for the runner_scheduler
    }

    // -----------------------------------------------------
    // ------------------ Runner_scheduler ------------------
    // ------------------------ End ------------------------
    // -----------------------------------------------------

}




