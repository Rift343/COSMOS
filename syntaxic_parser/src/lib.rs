use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use pyo3::prelude::*;

pub fn syntaxic_parser(sql_request : String) -> File {
    let request : String = sql_request.to_string();
    let mut res_textx :String = String::new();

    Python::with_gil(|py| {
        res_textx = get_textx_result(request,py).map_err(|e|{
            e.print_and_set_sys_last_vars(py);
        }).expect("Error at Python with gil");
    });

    // The truncate(true) option allows for overwriting the entire file, needed when writing less bytes than already present
    let mut synt_parsing_file : File = File::options().read(true).write(true).truncate(true).create(true).open("data/transferFile/syntaxic_parsing.json").expect("Error: Unable to open or create file");
    synt_parsing_file.write_all(res_textx.as_bytes()).expect("Error: Unable to write in file");
    synt_parsing_file.seek(SeekFrom::Start(0)).expect("Error: Unable to seek from start");

    synt_parsing_file
}

fn get_textx_result(request: String, py: Python) -> PyResult<String> {
    // Path for the textX syntaxic parser
    let synt_parser_file = include_str!("../textX_grammar/syntaxic_parser.py");
    // Type &PyModule var containing the code of syntaxic_parser.py
    let textx_code = PyModule::from_code(py,synt_parser_file,"syntaxic_parser.py",synt_parser_file).expect("Error: Unable to fetch Python code");

    // Extracts the is_valid_sql() function
    let func_is_valid_sql : &PyAny = textx_code.getattr("is_valid_sql").expect("Error: Unable to get is_valid_sql() function");
    // Calls is_valid_sql() with request
    let res_is_valid_sql : &PyAny = func_is_valid_sql.call1((request,)).expect("Error: Unable to call func_is_valid_sql");
    // Extracts result into a String
    let res_textx : String = res_is_valid_sql.extract().expect("Error: Unable to extract res_is_valid_sql String");

    // Returns String or Error
    Ok(res_textx)
}