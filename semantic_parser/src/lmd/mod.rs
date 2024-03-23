use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::structures::semantic_parser_file::{AggregateHashmap, ColumnNameCouple, SemanticParserFile, TableHashmap};
use crate::structures::syntaxic_parser_file::{SyntaxicParserFile, TableNameCouple};
use crate::structures::table_metadata::{ColumnNameTypeCouple, TableMetadata};

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

fn check_if_attribute_is_valid(table_metadata_as_struct: &HashMap<String, TableMetadata>, attribute_name: &String, table_use_name: &String, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<TableNameCouple>) -> Result<String, Box<dyn Error>>{
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

        if specified_table.has_attribute(&attribute_name) {
            ()
        } else {
            return Err(Box::from(format!("Attribute with specified table name not found : {}.{} (Alt table name : {})\n", table_name, attribute_name, table_use_name)));
        }
    }

    Ok(table_name)
}

fn get_all_attributes_of_table(table_attribute_list: &Vec<ColumnNameTypeCouple>, to_fill_attribute_list: &mut Vec<ColumnNameCouple>){
    for couple in table_attribute_list{
        let temp_attribute_couple = ColumnNameCouple{
            attribute_name: couple.column_name.clone(),
            use_name_attribute: couple.column_name.clone(),
        };

        to_fill_attribute_list.push(temp_attribute_couple);
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
            let mut contents_of_file = String::new();

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

    let table_metadata_as_struct: HashMap<String, TableMetadata> = get_metadata("data/SemanticTestData/FM_1.json".to_string());
    let mut renamed_table_name_map: HashMap<String, String> = HashMap::new();

    let mut semantic_parser_file_as_struct = SemanticParserFile {
        tables: HashMap::new(),
        aggregates: vec![],
        conditions: None,
    };


    for requested_table in &syntaxic_file_content_as_struct.table_name {
        match table_metadata_as_struct.get(&requested_table.table_name) {
            None => {
                return Err(Box::from(format!("Requested table not found : {}\n", requested_table.table_name)));
            }
            Some(_) => {
                println!("Table {} found", requested_table.table_name);
                renamed_table_name_map.insert(requested_table.use_name_table.clone(), requested_table.table_name.clone());

                let temp = TableHashmap {
                    use_name_table: requested_table.use_name_table.clone(),
                    columns: vec![],
                };

                semantic_parser_file_as_struct.tables.insert(requested_table.table_name.clone(), temp);
            }
        }
    }

    for requested_attribute in &syntaxic_file_content_as_struct.columns {
        let attribute_name = requested_attribute.attribute_name.clone();
        let use_name_attribute = requested_attribute.use_name_attribute.clone();
        let mut use_name_table = requested_attribute.use_name_table.clone();

        if attribute_name == "*"{
            let vec_lifetime_extender: Vec<TableNameCouple>;
            let vector_of_table_names: &Vec<TableNameCouple>;

            if use_name_table == "" {
                vector_of_table_names = &syntaxic_file_content_as_struct.table_name;
            }
            else {
                let table_name = {
                    match renamed_table_name_map.get(&use_name_table){
                        None => {
                            return Err(Box::from(format!("Attribute claims to belong to a non existent table : {}.{}\n", use_name_table, attribute_name)));
                        }
                        Some(table_name) => {
                            table_name
                        }
                    }
                };

                let requested_table_attributes = TableNameCouple {
                    table_name: table_name.clone(),
                    use_name_table,
                };

                vec_lifetime_extender = vec![requested_table_attributes];
                vector_of_table_names = &vec_lifetime_extender;
            }

            for requested_table in vector_of_table_names {
                let attribute_list_for_requested_table = {
                    match table_metadata_as_struct.get(&requested_table.table_name){
                        None => {
                            return Err(Box::from(format!("Requested table not found in metadata despite validation : {}\n", requested_table.table_name)));
                        }
                        Some(requested_table_metadata) => {
                            &requested_table_metadata.columns
                        }
                    }
                };

                let attribute_list_for_requested_tables = {
                    &mut semantic_parser_file_as_struct.tables.get_mut(&requested_table.table_name).unwrap().columns
                };

                get_all_attributes_of_table(attribute_list_for_requested_table, attribute_list_for_requested_tables);
            }
        }

        else if attribute_name.contains(","){
            let (aggregate_type, attribute_name) = {
                let split_attribute_name: Vec<&str> = attribute_name.split(",").collect();

                (split_attribute_name[0].to_string(), split_attribute_name[1].to_string())
            };

            let table_name = check_if_attribute_is_valid(&table_metadata_as_struct, &attribute_name, &use_name_table, &renamed_table_name_map, &syntaxic_file_content_as_struct.table_name)?;

            if use_name_table == "" {
                use_name_table = table_name.clone();
            }

            let table_metadata_column_vec = &table_metadata_as_struct.get(&table_name).unwrap();

            let temp_aggregate_struct = AggregateHashmap {
                use_name_table,
                use_name_attribute,
                aggregate_type,
                attribute_type: table_metadata_column_vec.get_type_of_attribute(&attribute_name)?,
                attribute_name,
            };

            semantic_parser_file_as_struct.aggregates.push(temp_aggregate_struct);

        }
        else {
            let table_name = check_if_attribute_is_valid(&table_metadata_as_struct, &attribute_name, &use_name_table, &renamed_table_name_map, &syntaxic_file_content_as_struct.table_name)?;

            let temp_column_name_couple = ColumnNameCouple {
                attribute_name,
                use_name_attribute,
            };

            match semantic_parser_file_as_struct.tables.get_mut(&table_name){
                None => {
                    return Err(Box::from(format!("Table not found despite validation : {}\n", table_name)));
                }
                Some(current_table) => {
                    current_table.columns.push(temp_column_name_couple);
                }
            }
        }
    }

    let output_semantic_file_as_str = serde_json::to_string(&semantic_parser_file_as_struct).expect("Error whilst serialising semantic file struct.");

    let mut output_semantic_file = File::options().read(true).write(true).create(true).open("data/SemanticTestData/FSE_1.json").expect("Error whilst creating semantic parser output file");

    output_semantic_file.set_len(0).expect("Error whilst reinitialising semantic output file.");
    output_semantic_file.write_all(output_semantic_file_as_str.as_bytes()).expect("Error occurred whilst writing to semantic output file.");
    output_semantic_file.seek(SeekFrom::Start(0)).expect("Error whilst seeking in semantic output file.");

    Ok(output_semantic_file)
}