

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use serde_json::to_string;
use crate::structures::syntaxic_parser_file_ldd::SyntaxicParserFileLdd;

use crate::structures::table_metadata::{ColumnNameTypeCouple, Constraint, TableMetadata};

//use crate::structures::column_table_name_triple_ldd::ColumnTableNameTripleLdd;
//use super::structures::table_name_ldd::TableNameLdd;



/// # Retrieves table metadata stored at a given path
///
/// Not a constant but a function for now so as to later allow caching and editing of metadata
/// (Not to be included soon, part of the Data Definition Language)
fn get_metadata(metadata_file_path: String) -> HashMap<String, TableMetadata> {
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
fn semantic_parser_create(syntaxic_file_content_as_struct : SyntaxicParserFileLdd,table_metadata_as_struct : HashMap<String, TableMetadata> ) -> Result<File, Box<dyn Error>> {
    println!("debut create");
    
    let mut new_table_name = syntaxic_file_content_as_struct.table_name;
    let mut table_metadata_as_struct = table_metadata_as_struct;
    //We are supposed to get only one table, we only take the first element
    let unwraped_new_table_new =new_table_name.remove(0).table_name;
    println!("on check primary key");
for (table_name,_table_metadata) in &table_metadata_as_struct {
            if table_name.to_lowercase() == unwraped_new_table_new.to_lowercase() {
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
                    //table_name : unwraped_new_table_new,
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

                    //table_metadata_as_struct.push(result);
                    table_metadata_as_struct.insert(unwraped_new_table_new,result);
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


fn create_row_data(syntaxic_file_content: &SyntaxicParserFileLdd, table_name: &str) -> Option<HashMap<String, String>> {
    let table_columns = syntaxic_file_content.columns.iter()
        .filter(|col| col.name == table_name)
        .map(|col| (col.name.clone(), col.datatype.clone(), col.data.clone()))
        .collect::<Vec<_>>();

    let mut row_data = HashMap::new();

    for (name, datatype, data) in table_columns {
        match datatype.as_str() {
            "INT" => {
                let value = data.as_ref()?.get(0).cloned()?;
                let int_value = value.parse::<i32>().ok()?;
                row_data.insert(name, int_value.to_string());
            }
            "VARCHAR" => {
                let value = data.as_ref()?.get(0).cloned()?;
                row_data.insert(name, value);
            }
            // AUTRES TYPES DE DONNEES
            _ => return None,
        }
    }

    Some(row_data)
}
            


fn check_types(
    row_data: &HashMap<String, String>,
    columns: &Vec<ColumnNameTypeCouple>,
) -> Result<(), Box<dyn Error>> {
    let r = "NULL".to_string();
    for column in columns {
        let value = row_data
            .get(column.column_name.as_str())
            .unwrap_or(&r);
        if column.column_type=="DATE"{
            return Err(Box::from("you tried inserting data in a table with a date type, this type is not implemented yet".to_string()));
        }
        if column.column_type=="INT"{

            if (! row_data.get(column.column_name.as_str()).unwrap_or(&r).parse::<i32>().is_ok()){

                return Err(Box::from(format!(
                    "Type mismatch for column '{}': expected INT', found '{}'",
                    column.column_name, value)));
            }
        
        
        }
        }
        /*
        if column.column_type != value.to_uppercase() {
            return Err(Box::from(format!(
                "Type mismatch for column '{}': expected '{}', found '{}'",
                column.column_name, column.column_type, value
            )));

        }
    }*/

    Ok(())
}

fn check_constraints(
    row_data: &HashMap<String, String>,
    constraints: &Vec<Constraint>,
) -> Result<(), Box<dyn Error>> {
    for constraint in constraints {
        let mut valid = false;
        for column_name in &constraint.attribute_list {
            let r = "NULL".to_string();
            let value = row_data
                .get(column_name.as_str())
                .unwrap_or(&r);

            match &constraint.constraint_type[..] {
                "NOT NULL" => {
                    if value == "NULL" {
                        return Err(Box::from(format!(
                            "NOT NULL constraint violated for column '{}'",
                            column_name
                        )));
                    }
                }
                "UNIQUE" => {
                    // Check if the value is unique
                    // ...
                }
                "PRIMARY KEY" => {
                    // Check if the value is not null
                    if value == "NULL" {
                        return Err(Box::from(format!(
                            "PRIMARY KEY constraint violated for column '{}'",
                            column_name
                        )));
                    }
                }
                _ => {
                    // Check other constraints
                    // ...
                }
            }

            // If the constraint is valid for this column, set the flag to true
            valid = true;
        }

        // If the constraint is not valid for any column, return an error
        if !valid {
            return Err(Box::from(format!(
                "Constraint '{}' violated",
                constraint.constraint_name
            )));
        }
    }

    Ok(())
}

fn semantic_parser_insert(
    table_metadata: HashMap<String, TableMetadata>,
    syntaxic_file_content: SyntaxicParserFileLdd,
    table_name: String,
) -> Result<File, Box<dyn Error>> {
    // Check if the table exists
    let table_meta = table_metadata.get(&table_name).ok_or("Table not found")?;

    // Create a new RowData instance based on the provided columns
    let row_data = create_row_data(&syntaxic_file_content, &table_name).ok_or("Primary key not filled")?;

    // Check if the primary key is filled in
    let primary_key_column = table_meta
        .constraints
        .iter()
        .find(|c| c.constraint_type == "PrimaryKey")
        .and_then(|c| c.attribute_list.get(0))
        .ok_or("Primary key not filled")?;
    if !row_data.contains_key(primary_key_column) {
        return Err(Box::from(format!("Primary key '{}' not filled", primary_key_column)));
    }

    // Check that the types are correct
    check_types(&row_data, &table_meta.columns)?;

    // Check that the constraints are valid
    check_constraints(&row_data, &table_meta.constraints)?;

    // Perform the actual insertion
    // ...

    let json_string = match to_string(&table_metadata) {
        Ok(result) => result,
        Err(error) => return Err(Box::from(format!("Unable to serialize struct to JSON: {}\n", error)))
    };
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
    let table_metadata_as_struct: HashMap<String,TableMetadata> = get_metadata("data/SemanticTestData/FM_1.json".to_string());


    // Temporary variable to store what will be returned in the file
    // Done now due to the vector requiring allocating
    if syntaxic_file_content_as_struct.action=="create"{
        println!("parser create");
        return semantic_parser_create(syntaxic_file_content_as_struct,table_metadata_as_struct);
        
    }else if syntaxic_file_content_as_struct.action=="insert"{
        println!("parser insert");
        // Extract the table name from the SyntaxicParserFileLdd struct
        let table_name = syntaxic_file_content_as_struct.table_name[0].table_name.clone();

        //faire le get du nom de la table voulue
        return semantic_parser_insert(table_metadata_as_struct, syntaxic_file_content_as_struct, table_name);

    } else {
        return Err(Box::from("NOT IMPLEMENTED".to_string()));
    }
}

