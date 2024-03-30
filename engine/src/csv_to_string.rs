use std::{fs::File, io::{BufReader, Read, Seek}};
use std::error::Error;
use serde_json::Value;
use std::string::String;


use syntaxic_parser::syntaxic_parser;
use runner_scheduler::scheduler;
use semantic_parser::lmd::semantic_parser;
use semantic_parser::ldd::semantic_parser_ldd;
//use engine::csv_to_string;

use std::fs;
//use csv::Reader;
/* 
pub fn engine_main(file_name : String) ->  Result<String, Box<dyn std::error::Error>> {

    //A CHANGER EN file_name DES QU ON PREND LE RES DE O/E
    let filename = "engine/TestCosmosFichierResultat.csv";

    //Convert csv to string
    let result = csv_to_string(filename);
    return result;
}*/

fn is_ldd(file_path: String) -> bool {
    // Read the file to a string.
    let table = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return false;
        }
    };

    // Parse the string as JSON.
    let content: Value = match serde_json::from_str(&table) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error parsing JSON: {}", error);
            return false;
        }
    };

    // Check if the "action" field is not select.
    match content.get("action") {
        Some(Value::String(action)) if action != "select" => true,
        _ => false,
    }
}

pub fn csv_to_string(mut file_name : &File) -> Result<String, Box<dyn std::error::Error>> {
    //string resultat
    file_name.rewind()?;
    
    let mut buf = String::new();
    let mut my_buffreader = BufReader::new(file_name);
    my_buffreader.read_to_string(&mut buf).expect("error)");
    /* 
    let mut res = String::new();
    //ouverture du reader
    let mut rdr = Reader::from_path(file_name)?;

    //recuperation du header
    let hd = rdr.headers();
    //passage du header en string
    let hd_string = hd.unwrap();

    //compteur nomnbre colonne
    let mut nb_colonne = 0;

    //ajout du header au resultat
    for column_name in hd_string{
        res = res + column_name + ";";
        nb_colonne += 1;
    }

    // on ajoute un \n a la fin de la ligne header
    res = res + "\n";

    //Pour la vue, on compte le nombre de ; avant le \n et on a le nb de colonne
    //ensuite affichage des mots tous les nb colonnes

    //ajout des donnees dans le fichier
    let data = rdr.records();
    //Boucle pour chaque ligne
    for line in data{
        let ldata = line.unwrap();
        //Boucle pour chaque colonne
        for index in 0..nb_colonne{
            res = res + &ldata[index] + ";";
        }
        //fin de ligne donc on rajoute \n
        res += "\n";
    }*/
    Ok(buf)
}


pub fn engine(request : String) ->Result<std::string::String, Box<(dyn std::error::Error + 'static)>> {

    // -----------------------------------------------------
    // ------------------ Syntaxic Parser ------------------
    // ----------------------- Start -----------------------
    // -----------------------------------------------------

    // Call the syntaxic parser and get file handle for the syntaxic parsing file
    let mut syntaxic_parsing_handle : File = match syntaxic_parser(request){
        Ok(file) => file,
        Err(error) => {
            println!("{}", error.to_string());
            return Err(error);
        }
    };

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
   let semantic_parser_res: Result<File, Box<dyn Error>> = Err("semantic parser not initialized".into());
    if (is_ldd("./data/transferFile/syntaxic_parsing.json".to_string())){

        let semantic_parser_res = semantic_parser_ldd(syntaxic_parsing_handle);
    }else{

        let semantic_parser_res = semantic_parser(syntaxic_parsing_handle);
    }
   

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