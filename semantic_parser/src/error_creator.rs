use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

use crate::structures::semantic_parser_file::SemanticParserFile;

pub fn create_semantic_error(error: String) -> File{
    let temp = SemanticParserFile {
        tables: vec![],
        conditions: None,
        status: false,
        error,
    };

    let mut error_file = File::options().read(true).write(true).create(true).open("semantic_parser/TestData/Error.json").expect("Error occurred whilst creating semantic error file.");

    error_file.set_len(0).expect("Error occurred whilst wiping semantic error file.");
    error_file.write_all(serde_json::to_string(&temp).expect("Error during attempt to Stringify error structure.").as_bytes()).expect("Error occurred whilst writing to semantic error file.");
    error_file.seek(SeekFrom::Start(0)).expect("Error occurred whilst seeking semantic error file");

    error_file
}