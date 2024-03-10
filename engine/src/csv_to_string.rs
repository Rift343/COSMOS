use std::{fs::File, io::{BufReader, Read, Seek}};

//use serde_json::Value::String;
use serde_json::Value;
#[allow(unused)]
use std::string::String;


use syntaxic_parser::syntaxic_parser;
use runner_scheduler::scheduler;
use semantic_parser::semantic_parser;
use semantic_parser::structures::semantic_parser_file::SemanticParserFile;


//use view::error_printer;
//use view::request_receiver;
#[allow(unused)]
use view::result_printer;

/* 
pub fn engine_main(file_name : String) ->  Result<String, Box<dyn std::error::Error>> {

    //A CHANGER EN file_name DES QU ON PREND LE RES DE O/E
    let filename = "engine/TestCosmosFichierResultat.csv";

    //Convert csv to string
    let result = csv_to_string(filename);
    return result;
}*/
pub fn csv_to_string(mut file_name : &File) -> Result<String, Box<dyn std::error::Error>> {
    //string resultat
    file_name.rewind()?;
    
    let mut buf = String::new();
    let mut my_buffreader = BufReader::new(file_name);
    my_buffreader.read_to_string(&mut buf).expect("eror)");
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


pub fn process_request(sql_query : String) -> String {


    println!("Main View : fini");
    /*
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
    let mut semantic_file = semantic_parser(syntaxic_parsing_handle);
    
    
    // Extract the file contents to a string first, then to a structure so that we may examine its fields.
    let semantic_file_content_as_struct: SemanticParserFile = {
        let mut semantic_file_contents_as_string :  std::string::String = Default::default();
    
    
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
    match csv_file_returned {//First match on the result of the runner_scheduler.
        Ok(content) => {
            let printable_string;
            printable_string = csv_to_string(&content);
            match printable_string {
                Ok(content) => content,
                Err(e) => {
                    format!(
                        "
                        -----------------------------------------------------
                        ---------------------Engine--------------------------
                        ---------------------Error---------------------------
                        -----------------------------------------------------
                        Maybe CSV file is already used or not existe anymore.
                        Please check the data/CSV directory
                        Error Message  : {}
    
                        ",
                        e.to_string()
                    )
                }
            }
        }, //Case 1, we have a CSV file so CSV_to_string then result_printer
        Err(e) => {
            format!("
            -----------------------------------------------------
            -----------------Runner_scheduler--------------------
            ---------------------Error---------------------------
            -----------------------------------------------------
            Maybe CSV file is already used or not existe anymore.
            Please check the data/CSV directory
            Error Message : {}
            ", e.to_string())
        },//Case2, print there is a error on a file for the runner_scheduler
    }
    
    // -----------------------------------------------------
    // ------------------ Runner_scheduler ------------------
    // ------------------------ End ------------------------
    // -----------------------------------------------------
    
    
    }