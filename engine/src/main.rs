use std::fs::File;
use std::io::{Read, Seek};
use serde_json::Value;


use syntaxic_parser::syntaxic_parser;
use runner_scheduler::scheduler;
use semantic_parser::semantic_parser;
use engine::csv_to_string;
use view::error_printer;
use view::request_receiver;
//use view::result_printer;

fn main() {

    // -----------------------------------------------------
    // ----------------------- View ------------------------
    // ----------------------- Start -----------------------
    // -----------------------------------------------------
    let req_receiver = request_receiver();
    let mut sql_query : std::string::String = Default::default();
    match req_receiver {
        Ok(s) => sql_query = s,
        Err(e) => error_printer(e)
    }


    /*
    //match resultat de request receiver
    match req_receiver {
        //Si on arrive a lire la requete dans l'entrée standart
        //On envoie la requete a l'engine
        Ok(req) => match engine::engine_main(req){
            Ok(res) => result_printer(res),
            Err(err) => error_printer(err)
        }
        Err(e) => error_printer(e)

    }


     */

    println!("Main View : fini");

    // -----------------------------------------------------
    // ------------------ Syntaxic Parser ------------------
    // ----------------------- Start -----------------------
    // -----------------------------------------------------

    // Get query, static for now, should get from view later (request_receiver)
    //let sql_query : std::string::String = "SELECT Id, Nom, Prenom FROM Personne;".to_string();

    // Call the syntaxic parser and get file handle for the syntaxic parsing file
    let mut syntaxic_parsing_handle : File = syntaxic_parser(sql_query);

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

    let semantic_file = match semantic_parser_res {
        Ok(contenu) => contenu,
        Err(err) => {
            error_printer(err);
            return
        }
    };

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
            let printable_string;
            printable_string=csv_to_string(&content);
            match printable_string {//Seconde math when the result string from the CSV File
                Ok(content) => println!("{}",content),
                Err(e) => println!("
                    -----------------------------------------------------
                    ---------------------Engine--------------------------
                    ---------------------Error---------------------------
                    -----------------------------------------------------
                    Maybe CSV file is already used or not existe anymore.
                    Please check the data/CSV directory
                    Error Message  : {}

                    ",e),//error message of csv_to_string return an error
            }


        },//Case 1, we have a CSV file so CSV_to_string then result_printer
        Err(e) => {println!("
        -----------------------------------------------------
        -----------------Runner_scheduler--------------------
        ---------------------Error---------------------------
        -----------------------------------------------------
        Maybe CSV file is already used or not existe anymore.
        Please check the data/CSV directory
        Error Message : {}
        ",e);},//Case2, print there is a error on a file for the runner_scheduler
    }

    // -----------------------------------------------------
    // ------------------ Runner_scheduler ------------------
    // ------------------------ End ------------------------
    // -----------------------------------------------------

}




