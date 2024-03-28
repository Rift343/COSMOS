pub mod structures;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use serde_json::to_string;
use structures::syntaxic_parser_file::SyntaxicParserFile;

use structures::semantic_parser_file::SemanticParserFile;
use structures::syntaxic_parser_file_ldd::SyntaxicParserFileLdd;
use structures::semantic_parser_file::TableDictionary;

use structures::table_metadata::{ColumnNameTypeCouple, Constraint, TableMetadata};

use crate::structures::semantic_parser_file::ColumnNameCouple;
use crate::structures::table_name_couple::TableNameCouple;

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
        Err(error) => {panic!("Error : {}", error)}
    }
}
fn semantic_parser_create(syntaxic_file_content_as_struct : SyntaxicParserFileLdd,table_metadata_as_struct : Vec<TableMetadata> ) -> Result<File, Box<dyn Error>> {
    println!("debut create");
    
    let mut new_table_name = syntaxic_file_content_as_struct.table_name;
    let mut table_metadata_as_struct = table_metadata_as_struct;
    //We are supposed to get only one table, we only take the first element
    let unwraped_new_table_new =new_table_name.remove(0).table_name;
    println!("on check primary key");
for table_metadata in &table_metadata_as_struct {
            if table_metadata.table_name.to_lowercase() == unwraped_new_table_new.to_lowercase() {
                println!("a table with this name already exist");
                return Err(Box::from("a table with this name already exist".to_string()));
            }
            let mut primary_key_present = false;
            for column in  &syntaxic_file_content_as_struct.columns{
                for constraint in &column.constraints{
                    if constraint=="PRIMARY KEY"{
                        primary_key_present= true;
                    }
                }}
            if ! primary_key_present{

                println!("No primary key specified");
                return Err(Box::from("No primary key specified".to_string()));
            }
                }
            

                println!("on create table meta data");
                
                let mut result = TableMetadata{
                    table_name : unwraped_new_table_new,
                    columns : vec![],
                    constraints: vec![], 
                };
                for column in &syntaxic_file_content_as_struct.columns {
                    result.columns.push(ColumnNameTypeCouple {
                        column_name: column.name.clone(),
                        column_type: column.datatype.clone(),
                    });
                    for constraint in &column.constraints{
                        let mut attribute_list = Vec::new();
                        attribute_list.push(column.name.clone());
                        // if we want to add a primary key
                        if constraint.clone() == "PRIMARY KEY".to_string() {
                        //we test if a constraint for the primary key already exist
                        let mut already_primary = false;
                        for c in &mut result.constraints {
                            if c.constraint_type.clone() == "PrimaryKey".to_string(){
                                already_primary = true;
                                c.attribute_list.push(column.name.clone())
                            }
                        }
                        if !already_primary{
                            result.constraints.push(Constraint {
                                constraint_name: column.name.clone() + &constraint,
                                constraint_type: "PrimaryKey".to_string(),
                                attribute_list: attribute_list.clone(),
                                foreign_key: None,
                                check: None,
                            });
                        }}else{
                    
                        result.constraints.push(Constraint {
                            constraint_name: column.name.clone() + &constraint,
                            constraint_type: constraint.clone(),
                            attribute_list:attribute_list.clone(),
                            foreign_key: None,
                            check: None,
                        });}
                        }
                    }

                    table_metadata_as_struct.push(result);
                    let json_string = match to_string(&table_metadata_as_struct) {
                        Ok(result) => result,
                        Err(error) => return Err(Box::from(format!("Unable to serialize struct to JSON: {}\n", error)))
                    };
            // Open or create a file called syntaxic_parsing.json placed in data/transferFile
            // The truncate(true) option allows for overwriting the entire file, needed when writing less bytes than already present
            let mut synt_parsing_file : File = match File::options().read(true).write(true).truncate(true).create(true).open("data/SemanticTestData/FM_1.json"){
                Ok(result) => result,
                Err(error) => return Err(Box::from(format!("Unable to open or create file : {}\n", error)))
            };
            println!("write all");
            // Write the contents of res_textx in the file
            match synt_parsing_file.write_all(json_string.as_bytes()){
                Ok(_) => (),
                Err(error) => {
                    println!("{}?",error);
                    return Err(Box::from(format!("Unable to write in file : {}\n", error)));}
            };
        
            // Set the offset to the beginning of the file
            match synt_parsing_file.seek(SeekFrom::Start(0)){
                Ok(_) => (),
                Err(error) => {
                    println!("{}?",error);
                return Err(Box::from(format!("Unable to seek from start : {}\n", error)));}
                //return Err(Box::from(&("Error, Unable to seek from start".to_string() + error_str))) to get rid of type str_err
            };
            println!("fin create");
            return Ok(synt_parsing_file);
            }


pub fn semantic_parser_ldd(mut syntaxic_file: File) -> Result<File, Box<dyn Error>> {
    // Extract the file contents to a structure
    let syntaxic_file_content_as_struct: SyntaxicParserFileLdd = {
        // File stores a str not structure, so we must first extract it before converting and put it
        // In a temporary variable
        let syntaxic_file_content_as_str = {
            let mut contents_of_file = String::new();

            match syntaxic_file.read_to_string(&mut contents_of_file) {
                Ok(_) => (),
                Err(error) => return Err(Box::try_from(error).unwrap())
            }

            contents_of_file
        };
        println!("read syntaxe");
        // Extract the string to a SyntaxicParserFile structure, and return it to allow
        // syntaxic_file_content_as_struct to receive the value
        println!("{}?",syntaxic_file_content_as_str.as_str());
        match serde_json::from_str(syntaxic_file_content_as_str.as_str()) {
            Ok(content) => {
                content
            }
            Err(error) => {
                println!("{}?", error);
                return Err(
                Box::try_from(error).unwrap());}
        }
    };
    println!("read metadata");
    let table_metadata_as_struct: Vec<TableMetadata> = get_metadata("data/SemanticTestData/FM_1.json".to_string());


    // Temporary variable to store what will be returned in the file
    // Done now due to the vector requiring allocating
    if (syntaxic_file_content_as_struct.action=="create"){
        println!("parser create");
        return semantic_parser_create(syntaxic_file_content_as_struct,table_metadata_as_struct);
        
    }else{
        return Err(Box::from("PAS IMPLEMENTER".to_string()));

    }
}
/// # Main function of the semantic parsing module
///
/// Takes a syntaxic parser file and returns a semantic parser file.
///
/// Panics for file or JSON errors
/// Does not panic for errors related to the syntaxic file, but as expected returns a semantic
/// parser file with the reason for failure filled out
pub fn semantic_parser(mut syntaxic_file: File) -> Result<File, Box<dyn Error>> {
    // Extract the file contents to a structure
    let syntaxic_file_content_as_struct: SyntaxicParserFile = {
        // File stores a str not structure, so we must first extract it before converting and put it
        // In a temporary variable
        let syntaxic_file_content_as_str = {
            let mut contents_of_file = "./data/transferFile/syntaxic_parsing.json".to_string();

            match syntaxic_file.read_to_string(&mut contents_of_file) {
                Ok(_) => (),
                Err(error) => return Err(Box::try_from(error).unwrap())
            }

            contents_of_file
        };
        // Extract the string to a SyntaxicParserFile structure, and return it to allow
        // syntaxic_file_content_as_struct to receive the value
        match serde_json::from_str(syntaxic_file_content_as_str.as_str()) {
            Ok(content) => {
                content
            }
            Err(error) => return Err(Box::try_from(error).unwrap())
        }
    };

    let table_metadata_as_struct: Vec<TableMetadata> = get_metadata("data/SemanticTestData/FM_1.json".to_string());

    let mut table_name_correspondent: HashMap<String, String> = HashMap::new();

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
    // And if they do then store them in a TableDictionaryy and add it to the tables vector for
    // res_printable
    for requested_table in syntaxic_file_content_as_struct.table_name {
        let mut found_table = false;
        let mut table_name = "".to_string();

        for table_metadata in &table_metadata_as_struct {
            if table_metadata.table_name.to_lowercase() == requested_table.table_name.to_lowercase() {
                found_table = true;
                table_name = table_metadata.table_name.clone();

                table_name_correspondent.insert(requested_table.use_name_table.clone(), table_name.clone());
            }
        }

        println!("Tables requested : {:?}\tActual name {}\tRenamed name : {}\tFound : {}", requested_table, table_name, requested_table.use_name_table.clone() ,found_table);

        if ! found_table {
            return Err(Box::from(format!("Requested table not found : {}\n", table_name)));

        }

        let temp_dic_table = TableDictionary {
            table: TableNameCouple{
                table_name,
                use_name_table: requested_table.use_name_table.clone(),
            },
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
        let mut corresponding_column: String = "".to_string();

        // Put check here inside of loop, doesn't matter as either we have '*' and nothing else, or a list of attributes, never a mix of the two
        if requested_column.attribute_name != "*" {
            for table_metadata in &table_metadata_as_struct {
                for column_couple in &table_metadata.columns {
                    let corresponding_table_name = {
                        let res;

                        match table_name_correspondent.get(&requested_column.use_name_table.clone()) {
                            None => return Err(Box::from(format!("Renamed table not found : {}\n", requested_column.use_name_table))),
                            Some(name) => res = name
                        };

                        res.to_lowercase()
                    };
                    // Both table and column name match
                    // OR if table name is empty then only match column name
                    if ((requested_column.attribute_name.to_lowercase() == column_couple.column_name.to_lowercase()) && (corresponding_table_name == table_metadata.table_name.to_lowercase()))
                        ||
                        (corresponding_table_name == "") {
                        nb_found += 1;
                        if nb_found == 1 {
                            corresponding_table = table_metadata.table_name.clone();
                            corresponding_column = column_couple.column_name.clone();
                        }
                    } else if (requested_column.attribute_name == "*") && (corresponding_table_name == table_metadata.table_name.to_lowercase()) {
                        corresponding_table = table_metadata.table_name.clone();
                        corresponding_column = column_couple.column_name.clone();
                        nb_found = 1;
                    }
                }
            }

            println!("Requested column : '{}.{}'\tActual : '{}.{}'", requested_column.use_name_table, requested_column.use_name_attribute, corresponding_table, corresponding_column);


            // React differently depending on how many occurrences for a better error messages
            match nb_found {
                0 => {
                    return Err(Box::from(format!("Column : '{}.{}'\tNon-Renamed : {}\nNot found", requested_column.use_name_table, requested_column.use_name_attribute, requested_column.attribute_name)))
                }
                1 => {
                    // If the column is correct, then go over every requested table in our return variable
                    // And once we found the table to which our column belongs, then we add it to it
                    for table in &mut res_printable.tables {
                        if table.table.table_name == corresponding_table {
                            let temp_couple = ColumnNameCouple {
                                attribute_name: corresponding_column.clone(),
                                use_name_attribute: requested_column.use_name_attribute.clone(),
                            };

                            table.columns.push(temp_couple);
                        }
                    }
                }
                // Any number of occurrences beyond 1 is handled identically, all are ambiguous
                _ => {
                    return Err(Box::from(format!("Column : '{}.{}'\nAmbiguous request, multiple occurrences in requested table list.", requested_column.use_name_table, requested_column.use_name_attribute)))
                }
            }
        }

        // Handle the situation where we have '*' and nothing else, where we need to add every attribute to the request
        // Will need some cleaning up but works.
        else {
            for table in &mut res_printable.tables {
                for table_metadata in &table_metadata_as_struct {
                    for column_couple in &table_metadata.columns {
                        if table_metadata.table_name == table.table.table_name {
                            let temp_couple = ColumnNameCouple {
                                attribute_name: column_couple.column_name.clone(),
                                // If * used, as we have no way of renaming individual attributes we just reuse the name
                                use_name_attribute: column_couple.column_name.clone(),
                            };

                            table.columns.push(temp_couple);
                        }
                    }
                }
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

    Ok(output_semantic_file)
}
