
//use std::fs::File;
//use std::io::{Read, Seek};
//use serde_json::Value::String;
//use serde_json::Value;
#[allow(unused)]
use std::string::String;


//use syntaxic_parser::syntaxic_parser;
//use runner_scheduler::scheduler;
//use semantic_parser::semantic_parser;
//use semantic_parser::structures::semantic_parser_file::SemanticParserFile;
//use engine::csv_to_string;
use engine::process_request;

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
    //we call the process_request function to process the request
    let result = process_request(sql_query);
    println!("{}",result);
    // -----------------------------------------------------


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

}




