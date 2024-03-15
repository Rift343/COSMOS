use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};
use std::process::exit;
use serde_json::Value;
use std::string::String;


use syntaxic_parser::syntaxic_parser;
use runner_scheduler::scheduler;
use semantic_parser::semantic_parser;
use engine::csv_to_string;
use view::error_printer;
use view::request_receiver;
use view::result_printer;

fn main() {
    let run = true;
    while run
    {
        let req_receiver = request_receiver();
        match req_receiver
            {
            Ok(request) =>
                {
                if request == "exit".to_string() {
                    //run = false;
                    result_printer("Another stellar performance! 🌟".to_string());
                    exit(0);
                }
                else{
                    match engine(request)
                        {
                        Ok(result) => result_printer(result),
                        Err(error_engine) => error_printer(error_engine)
                        }
                    }
                }
            Err(error_request_receiver) => error_printer(error_request_receiver)
            }
    }
    println!("Another stellar performance! 🌟");
}

fn engine(request : String) ->Result<std::string::String, Box<(dyn std::error::Error + 'static)>> {

     // -----------------------------------------------------
     // ------------------ Syntaxic Parser ------------------
     // ----------------------- Start -----------------------
     // -----------------------------------------------------

     // Call the syntaxic parser and get file handle for the syntaxic parsing file
     let mut syntaxic_parsing_handle : File = syntaxic_parser(request);

     // Read the file and put its contents into a String
     let mut syntaxic_parsing_content: std::string::String = Default::default();
     syntaxic_parsing_handle.read_to_string(&mut syntaxic_parsing_content).expect("Error: Unable to read syntaxic parsing file");

     // Convert to a serde_json Value type
     let parsing_value : Value = serde_json::from_str(&*syntaxic_parsing_content).expect("Error: Unable to turn JSON String into Value type");

     // Show "status" and "error" fields
     println!("Status : {}\nError : {}\n",parsing_value["status"], parsing_value["error"]);
     if parsing_value["status"]=="false" {
         // Print for now, should send to the view later (result_printer)
         println!("{}",parsing_value["error"]);
     }
     else {
         // Print for now, should be given to the semantic parser later
         println!("{:?}",syntaxic_parsing_handle);
     }

     syntaxic_parsing_handle.rewind().expect("Aled");

     // -----------------------------------------------------
     // ------------------ Syntaxic Parser ------------------
     // ------------------------ End ------------------------
     // -----------------------------------------------------

     // -----------------------------------------------------
     // ------------------ Semantic Parser ------------------
     // ----------------------- Start -----------------------
     // -----------------------------------------------------

    // Mock syntaxic file, replace these variables when done
    //let syntaxic_file_name = "data/SemanticTestData/FS_1.json".to_string();
    //let syntaxic_file = File::options().read(true).open(syntaxic_file_name).expect("ENGINE :\tError occurred whilst attempting to open syntaxic file input");

    // Get the outputted semantic file.
    let semantic_parser_res = semantic_parser(syntaxic_parsing_handle);

    let semantic_file : File;
    
    match semantic_parser_res {
        Ok(contenu) => semantic_file = contenu,
        Err(err) => {
            println!("{}",err.to_string());
            return Ok("Erreur semantic parser".to_string());
        }
    }

    // -----------------------------------------------------
     // ------------------ Semantic Parser ------------------
     // ------------------------ End ------------------------
     // -----------------------------------------------------

     // -----------------------------------------------------
     // ------------------ Runner_scheduler ------------------
     // ----------------------- Start -----------------------
     // -----------------------------------------------------

     let csv_file_returned = scheduler(&semantic_file);
     match csv_file_returned {//First match on the result of the runner_scheduler.
         Ok(content) => {
             let printable_string : Result<String, Box<dyn Error>>;
             printable_string=csv_to_string(&content);
             match printable_string {//Seconde math when the result string from the CSV File
                 Ok(content) => {
                     //println!("{}",content);
                     return Ok(content);
                 },


                 Err(e) => {
                     println!("
                     -----------------------------------------------------
                     ---------------------Engine--------------------------
                     ---------------------Error 1--------------------------
                     -----------------------------------------------------
                     Maybe CSV file is already used or not existe anymore.
                     Please check the data/CSV directory
                     Error Message  : {}

                    ",e);
                     return Ok("
                     -----------------------------------------------------
                     ---------------------Engine--------------------------
                     ---------------------Error 1--------------------------
                     -----------------------------------------------------
                     Maybe CSV file is already used or not existe anymore.
                     Please check the data/CSV directory
                     ".to_string());
                 }
                    //error message of csv_to_string return an error
             }


         },//Case 1, we have a CSV file so CSV_to_string then result_printer
         Err(e) => {
             println!("
         -----------------------------------------------------
         -----------------Runner_scheduler--------------------
         ---------------------Error 2-------------------------
         -----------------------------------------------------
         Maybe CSV file is already used or not existe anymore.
         Please check the data/CSV directory
         Error Message : {}
        ",e);
             return Ok("
         -----------------------------------------------------
         -----------------Runner_scheduler--------------------
         ---------------------Error 2-------------------------
         -----------------------------------------------------
         Maybe CSV file is already used or not existe anymore.
         Please check the data/CSV directory
         ".to_string());},//Case2, print there is a error on a file for the runner_scheduler
     }
     // -----------------------------------------------------
     // ------------------ Runner_scheduler ------------------
     // ------------------------ End ------------------------
     // -----------------------------------------------------
}




