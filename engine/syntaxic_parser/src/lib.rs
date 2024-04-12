//! This syntaxic parser library has two main goals : <br>
//!     1. Given a String as input, it checks whether that String is a valid SQL query <br>
//!     2. It creates a JSON file and returns its handle. The JSON file contains either the elements of the query given, if its syntax is correct, or a description of the error if its syntax is incorrect

use std::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use pyo3::prelude::*;

/// This function's purpose is to be called by the Engine module with an SQL query, and return a file descriptor.
/// <br>
/// <br>
/// It takes a String as input and outputs a file descriptor referring to the `syntaxic_parsing.json` file it creates.
/// <br>
/// `pub fn syntaxic_parser(sql_request : String) -> File`
pub fn syntaxic_parser(sql_query : String) -> Result<File,Box<dyn Error>> {
    let mut res_textx :String = String::new();

    // A portion where we are going to use Python by calling our get_textx_result() function
    match Python::with_gil(|py| {
        res_textx = match get_textx_result(sql_query, py) {
            Ok(result) => result,
            Err(err) => return Err(err)
        };
        Ok(())
    }){
        Ok(()) => (),
        Err(err) => return Err(err)
    };

    // Open or create a file called syntaxic_parsing.json placed in data/transferFile
    // The truncate(true) option allows for overwriting the entire file, needed when writing less bytes than already present
    let mut synt_parsing_file : File = match File::options().read(true).write(true).truncate(true).create(true).open("data/transferFile/syntaxic_parsing.json"){
        Ok(result) => result,
        Err(error) => return Err(Box::from(format!("Unable to open or create file : {}\n", error)))
    };

    // Write the contents of res_textx in the file
    match synt_parsing_file.write_all(res_textx.as_bytes()){
        Ok(_) => (),
        Err(error) => return Err(Box::from(format!("Unable to write in file : {}\n", error)))
    };

    // Set the offset to the beginning of the file
    match synt_parsing_file.seek(SeekFrom::Start(0)){
        Ok(_) => (),
        Err(error) => return Err(Box::from(format!("Unable to seek from start : {}\n", error)))
        //return Err(Box::from(&("Error, Unable to seek from start".to_string() + error_str))) to get rid of type str_err
    };

    // Return file descriptor
    Ok(synt_parsing_file)
}


/// The function in which we fetch and use the TextX/Python code for the syntaxic parser
/// Takes an SQL request (String) and a Python token (cf. PyO3) as input, returns a String in JSON format or a Python error
fn get_textx_result(request: String, py: Python) -> Result<String,Box<dyn Error>> {//PyResult<String>
    // Path for the textX syntaxic parser
    let synt_parser_file = include_str!("../textX_grammar/syntaxic_parser.py");
    // Type &PyModule var containing the code of syntaxic_parser.py
    let textx_code = match PyModule::from_code(py,synt_parser_file,"syntaxic_parser.py",synt_parser_file){
        Ok(result) => result,
        Err(error) => return Err(Box::from(format!("Unable to fetch Python code : {}\n", error)))
    };

    // Extract is_valid_sql() function
    let func_is_valid_sql : &PyAny = match textx_code.getattr("is_valid_sql"){
        Ok(result) => result,
        Err(error) => return Err(Box::from(format!("Unable to get Python function : {}\n", error)))
    };

    // Call is_valid_sql() with request
    let res_is_valid_sql : &PyAny = match func_is_valid_sql.call1((request,)){
        Ok(result) => result,
        Err(error) => return Err(Box::from(format!("When calling the Python code : {}", error)))
    };

    // Extract result into a String
    let res_textx : String = match res_is_valid_sql.extract(){
        Ok(result) => result,
        Err(error) => return Err(Box::from(format!("Unable to extract Python result String : {}\n", error)))
    };

    // If the result is a syntaxic error, wrap it in an Err() and return it
    if res_textx.starts_with('S') {
        return Err(Box::from(res_textx))
    }

    // If the result is correct, return String with the contents of the syntaxic parsing file to be written
    Ok(res_textx)
}