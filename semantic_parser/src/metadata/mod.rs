use std::collections::HashMap;
use std::error::Error;
use std::fs;
use crate::structures::syntaxic_parser_file::TableNameCouple;
use crate::structures::table_metadata::TableMetadata;

pub fn get_metadata(metadata_file_path: String) -> HashMap<String, TableMetadata> {
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
        Err(error) => panic!("Error : {:?}", error)
    }
}

fn check_if_attribute_exist(table_metadata: &HashMap<String, TableMetadata>, attribute_name: &String, list_of_selected_tables: &Vec<TableNameCouple>) -> (Option<String>, u8) {
    let mut found_table_name: Option<String> = None;
    let mut nb_found = 0;

    for table in list_of_selected_tables {
        let table_name = &table.table_name;
        let table = table_metadata.get(table_name).expect("");

        if table.has_attribute(attribute_name) {
            println!("Found in table : {}.{}", table_name, attribute_name);
            found_table_name = Some(table_name.clone());
            nb_found += 1;
        }
    }
    return (found_table_name, nb_found);
}

pub fn check_if_attribute_is_valid(table_metadata_as_struct: &HashMap<String, TableMetadata>, attribute_name: &String, table_use_name: &String, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<TableNameCouple>) -> Result<String, Box<dyn Error>> {
    let table_name: String;

    if table_use_name == "" {
        let (optional_table_name, nb_found) = check_if_attribute_exist(&table_metadata_as_struct, attribute_name, selected_table_list);

        table_name = match optional_table_name {
            None => {
                return Err(Box::from(format!("No table containing requested attribute found : {}\n", attribute_name)));
            }
            Some(matching_table_name) => {
                matching_table_name
            }
        };

        match nb_found {
            0 => {
                return Err(Box::from(format!("Requested attribute not found : {}\n", attribute_name)));
            }
            1 => {
                ()
            }
            _ => {
                return Err(Box::from(format!("Multiple occurrences of requested attribute found ({}): {}\n", nb_found, attribute_name)));
            }
        }
    } else {
        table_name = match renamed_table_name_map.get(table_use_name) {
            None => {
                return Err(Box::from(format!("Unknown use of a table name : {}\n", table_use_name)));
            }
            Some(table_name) => {
                table_name.clone()
            }
        };

        let specified_table = match table_metadata_as_struct.get(&table_name) {
            None => {
                return Err(Box::from(format!("Unknown error : Table not found in metadata file despite validation : {}\t{}\n", table_use_name, table_name)));
            }
            Some(desired_table) => {
                desired_table
            }
        };

        if !specified_table.has_attribute(&attribute_name) {
            return Err(Box::from(format!("Attribute with specified table name not found : {}.{} (Alt table name : {})\n", table_name, attribute_name, table_use_name)));
        }
    }

    Ok(table_name)
}