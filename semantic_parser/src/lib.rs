pub mod structures;
mod error_creator;

use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use structures::syntaxic_parser_file::SyntaxicParserFile;

use structures::semantic_parser_file::SemanticParserFile;
use structures::semantic_parser_file::TableDictionary;

use structures::table_metadata::TableMetadata;

use structures::column_table_name_couple::ColumnTableNameCouple;

use error_creator::create_semantic_error;

/// # Retrieves table metadata stored at a given path
///
/// Not a constant but a function for now so as to later allow caching and editing of metadata
/// (Not to be included soon, part of the Data Definition Language)
fn get_metadata(metadata_file_path: String) -> Vec<TableMetadata> {
    // Store in temporary variable the file metadata but as a String
    let table_metadata_as_str = {
        match fs::read_to_string(metadata_file_path) {
            Ok(content) => {
                content
            }
            Err(error) => {
                panic!("Error : {}", error)
            }
        }
    };

    // Match to get content and extract it as a TableMetadata structure
    match serde_json::from_str(table_metadata_as_str.as_str()) {
        Ok(content) => {
            return content;
        }
        Err(error) => panic!("Error : {}", error)
    }
}


/// # Main function of the semantic parsing module
///
/// Takes a syntaxic parser file and returns a semantic parser file.
///
/// Panics for file or JSON errors
/// Does not panic for errors related to the syntaxic file, but as expected returns a semantic
/// parser file with the reason for failure filled out
pub fn semantic_parser(mut syntaxic_file: File) -> File {
    // Extract the file contents to a structure
    let syntaxic_file_content_as_struct: SyntaxicParserFile = {
        // File stores a str not structure, so we must first extract it before converting and put it
        // In a temporary variable
        let syntaxic_file_content_as_str = {
            let mut contents_of_file = String::new();

            match syntaxic_file.read_to_string(&mut contents_of_file) {
                Ok(_) => (),
                Err(error) => panic!("Error : {}", error)
            }

            contents_of_file
        };

        // Extract the string to a SyntaxicParserFile structure, and return it to allow
        // syntaxic_file_content_as_struct to receive the value
        match serde_json::from_str(syntaxic_file_content_as_str.as_str()) {
            Ok(content) => {
                content
            }
            Err(error) => panic!("Error : {}", error)
        }
    };

    let table_metadata_as_struct: Vec<TableMetadata> = get_metadata("data/SemanticTestData/FM_1.json".to_string());

    // Temporary variable to store what will be returned in the file
    // Done now due to the vector requiring allocating
    // TODO : Find a better name for this variable
    let mut res_printable = SemanticParserFile {
        tables: vec![],
        conditions: None,
        status: true,
        error: "".to_string(),
    };

    // Loops over every table amongst the requested tables,
    // Check if they exist by looping all tables in the metadata file,
    // And if they do then store them in a TableDictionary and add it to the tables vector for
    // res_printable
    for requested_table in syntaxic_file_content_as_struct.table_name {
        let mut found_table = false;

        for table_metadata in &table_metadata_as_struct {
            if table_metadata.table_name == requested_table {
                found_table = true;
            }
        }

        println!("Tables requested : {}\tFound : {}", requested_table, found_table);

        let temp_dic_table = TableDictionary {
            table_name: requested_table,
            columns: vec![],
        };

        res_printable.tables.push(temp_dic_table);
    }

    // Loops over every column amongst the request ones, as well as loop over every metadata column,
    // Count how many times they appear (Column and Table names must match, unless table name is
    // Empty, in which case only the column name needs to match
    for requested_column in syntaxic_file_content_as_struct.columns {
        let mut nb_found = 0;

        // Use corresponding_table variable rather than directly the requested table name because
        // We can give column names without a table name (if unambiguous), so this guarantees we can
        // Later properly add it to our return
        let mut corresponding_table: String = "".to_string();

        for table_metadata in &table_metadata_as_struct {
            for column_couple in &table_metadata.columns {
                // Both table and column name match
                // OR if table name is empty then only match column name
                if (requested_column.column_name == column_couple.column_name) && ((requested_column.table_name == table_metadata.table_name) || (requested_column.table_name == "")) {
                    nb_found += 1;
                    if nb_found == 1 {
                        corresponding_table = table_metadata.table_name.clone();
                    }
                }
            }
        }

        println!("Requested column : {}.{}\t", requested_column.table_name, requested_column.column_name);

        // React differently depending on how many occurrences for a better error messages
        match nb_found {
            0 => {
                return create_semantic_error(format!("Column : {}.{}\nNot found", requested_column.table_name, requested_column.column_name));
            }
            1 => {
                // If the column is correct, then go over every requested table in our return variable
                // And once we found the table to which our column belongs, then we add it to it
                for table in &mut res_printable.tables {
                    if table.table_name == corresponding_table {
                        let temp_couple = ColumnTableNameCouple {
                            table_name: corresponding_table.clone(),
                            column_name: requested_column.column_name.clone(),
                        };

                        table.columns.push(temp_couple);
                    }
                }
            }
            // Any number of occurrences beyond 1 is handled identically, all are ambiguous
            _ => {
                return create_semantic_error(format!("Column : {}.{}\nAmbiguous request, multiple occurrences in requested table list.", requested_column.table_name, requested_column.column_name));
            }
        }
    }

    // I/O operations to first convert the structure to a String using serde_json
    // Then to create an output file with read, write and creation permissions
    // We then wipe the files data (if it existed prior) by setting its length to 0
    // Then we write everything to the file, put the file cursor to the start to clean-up after ourselves
    // Before returning it
    let output_semantic_file_as_str = serde_json::to_string(&res_printable).expect("Error whilst serialising semantic file struct.");

    let mut output_semantic_file = File::options().read(true).write(true).create(true).open("data/SemanticTestData/FSE_1.json").expect("Error whilst creating semantic parser output file");


    output_semantic_file.set_len(0).expect("Error whilst reinitialising semantic output file.");
    output_semantic_file.write_all(output_semantic_file_as_str.as_bytes()).expect("Error occurred whilst writing to semantic output file.");
    output_semantic_file.seek(SeekFrom::Start(0)).expect("Error whilst seeking in semantic output file.");

    output_semantic_file
}
