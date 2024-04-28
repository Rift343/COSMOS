mod r#where;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::lmd::r#where::{Check, handle_where};

use crate::structures::semantic_parser_file::{AggregateHashmap, ColumnNameCouple, LogicalAllowType, SemanticParserFile, TableHashmap};

use crate::structures::syntaxic_parser_file::{SyntaxicParserFile, TableNameCouple};

use crate::structures::table_metadata::TableMetadata;

use crate::metadata::{get_metadata, check_if_attribute_is_valid};

/// # Retrieves table metadata stored at a given path
///
/// Not a constant but a function for now so as to later allow caching and editing of metadata
/// (Not to be included soon, part of the Data Definition Language)

pub fn match_verify_table_to_renamed(syntaxic_file_content_as_struct: &SyntaxicParserFile, table_metadata_as_struct: &HashMap<String, TableMetadata>, renamed_table_name_map: &mut HashMap<String, String>) -> Result<HashMap<String, TableHashmap>, Box<dyn Error>> {
    let mut semantic_parser_table_hashmap: HashMap<String, TableHashmap> = HashMap::new();

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

                semantic_parser_table_hashmap.insert(requested_table.table_name.clone(), temp);
            }
        }
    }

    Ok(semantic_parser_table_hashmap)
}

pub fn handle_select(syntaxic_file_content_as_struct: &SyntaxicParserFile, table_metadata_as_struct: &HashMap<String, TableMetadata>, renamed_table_name_map: &HashMap<String, String>, semantic_parser_table_hashmap: &mut HashMap<String, TableHashmap>, semantic_parser_aggregates: &mut Vec<AggregateHashmap>) -> Result<(), Box<dyn Error>> {
    for requested_attribute in &syntaxic_file_content_as_struct.columns {
        let attribute_name = requested_attribute.attribute_name.clone();
        let use_name_attribute = requested_attribute.use_name_attribute.clone();
        let mut use_name_table = requested_attribute.use_name_table.clone();

        if attribute_name == "*" {
            // * or table.*
            let vec_lifetime_extender: Vec<TableNameCouple>;
            let vector_of_table_names: &Vec<TableNameCouple>;

            if use_name_table == "" {
                vector_of_table_names = &syntaxic_file_content_as_struct.table_name;
            } else {
                let table_name = {
                    match renamed_table_name_map.get(&use_name_table) {
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
                let table_metadata_attribute_list_for_requested_table = {
                    match table_metadata_as_struct.get(&requested_table.table_name) {
                        None => {
                            return Err(Box::from(format!("Requested table not found in metadata despite validation : {}\n", requested_table.table_name)));
                        }
                        Some(requested_table_metadata) => {
                            requested_table_metadata
                        }
                    }
                };

                let semantic_parser_attribute_list_for_requested_table = {
                    &mut semantic_parser_table_hashmap.get_mut(&requested_table.table_name).unwrap().columns
                };

                table_metadata_attribute_list_for_requested_table.get_all_attributes_of_table(semantic_parser_attribute_list_for_requested_table);
            }
        } else if attribute_name.contains(",") {
            // Aggregate
            let (aggregate_type, attribute_name) = {
                let split_attribute_name: Vec<&str> = attribute_name.split(",").collect();

                (split_attribute_name[0].to_string(), split_attribute_name[1].to_string())
            };

            if attribute_name != "*" {
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

                semantic_parser_aggregates.push(temp_aggregate_struct);
            }

            else {
                if aggregate_type != "COUNT" {
                    return Err(Box::from(format!("'*' operator used with incompatible aggregate type : {}", aggregate_type)));
                }

                let temp_aggregate_struct = AggregateHashmap {
                    use_name_table,
                    use_name_attribute,
                    aggregate_type,
                    attribute_type: "INT".to_string(),
                    attribute_name,
                };

                semantic_parser_aggregates.push(temp_aggregate_struct);
            }

        } else {
            // attr or table.attr
            let table_name = check_if_attribute_is_valid(&table_metadata_as_struct, &attribute_name, &use_name_table, &renamed_table_name_map, &syntaxic_file_content_as_struct.table_name)?;

            let temp_column_name_couple = ColumnNameCouple {
                attribute_name,
                use_name_attribute,
            };

            match semantic_parser_table_hashmap.get_mut(&table_name) {
                None => {
                    return Err(Box::from(format!("Table not found despite validation : {}\n", table_name)));
                }
                Some(current_table) => {
                    current_table.columns.push(temp_column_name_couple);
                }
            }
        }
    }

    Ok(())
}

fn get_query_datatype(table_metadata_as_struct: &HashMap<String, TableMetadata>, t: &SemanticParserFile) -> Result<String, Box<dyn Error>> {
    if t.tables.len() != 1 {
        return Err(Box::from(format!("Subquery used with multiple tables selected ({} requested)\n", t.tables.len())));
    }

    for (key, value) in &t.tables {
        println!("Attributes : {:?}", &t.tables);
        println!("Attributes : {:?}", value);


        // &t.aggregates.len()
        if (value.columns.len()) != 1 {
            return Err(Box::from(format!("Subquery used with multiple attributes selected (Not implemented) ({} requested)\n", t.tables.len())));
        }

        let mut attribute_datatype = match table_metadata_as_struct.get(key) {
            Some(found_table) => {
                found_table
            }
            None => {
                return Err(Box::from("Unknown error : get_query_datatype : Table not found\n"));
            }
        }.get_type_of_attribute(&value.columns[0].attribute_name)?;

        attribute_datatype.truncate(7);

        return Ok(attribute_datatype);
    }

    unreachable!()
}

fn parse_syntaxic_struct(syntaxic_file_content_as_struct: &SyntaxicParserFile, table_metadata_as_struct: &HashMap<String, TableMetadata>) -> Result<SemanticParserFile, Box<dyn Error>> {
    let mut renamed_table_name_map: HashMap<String, String> = HashMap::new();

    let mut semantic_parser_table_hashmap: HashMap<String, TableHashmap> = match_verify_table_to_renamed(&syntaxic_file_content_as_struct, &table_metadata_as_struct, &mut renamed_table_name_map)?;
    let mut semantic_parser_aggregates: Vec<AggregateHashmap> = vec![];

    handle_select(&syntaxic_file_content_as_struct, &table_metadata_as_struct, &renamed_table_name_map, &mut semantic_parser_table_hashmap, &mut semantic_parser_aggregates)?;

    let mut requested_subqueries: HashMap<String, &SyntaxicParserFile> = HashMap::new();
    let mut subquery_hashmap: HashMap<String, SemanticParserFile> = HashMap::new();
    let mut subquery_checking: Vec<(String, Check, r#where::SubQHashMapAllowType)> = vec![];

    let where_clause_as_struct: Option<LogicalAllowType>;

    if (syntaxic_file_content_as_struct.where_clause.conditions.len() != 0) {
        let (_, _, t1): (isize, isize, LogicalAllowType) = handle_where(&syntaxic_file_content_as_struct.where_clause.conditions, &syntaxic_file_content_as_struct.where_clause.linkers, 0, syntaxic_file_content_as_struct.where_clause.linkers.len() as isize - 1, &table_metadata_as_struct, &renamed_table_name_map, &syntaxic_file_content_as_struct.table_name, &mut requested_subqueries, &mut subquery_checking)?;
        where_clause_as_struct = Some(t1);
    } else {
        where_clause_as_struct = None;
    }

    for (key, value) in requested_subqueries {
        subquery_hashmap.insert(key, parse_syntaxic_struct(&value, table_metadata_as_struct)?);
    }

    for (subquery_id, matched_against, to_edit_condition) in subquery_checking {
        let left_datatype = get_query_datatype(table_metadata_as_struct, subquery_hashmap.get(&subquery_id).unwrap())?;

        let right_datatype: String;

        match matched_against {
            Check::SubQ(right_sub_id) => {
                right_datatype = get_query_datatype(table_metadata_as_struct, subquery_hashmap.get(&right_sub_id).unwrap())?
            }
            Check::Val(right_type) => {
                right_datatype = right_type
            }
        }

        if left_datatype != right_datatype {
            return Err(Box::from(format!("parse_syntaxic_struct : mismatched datatypes (subquery involved): {} - {}", left_datatype, right_datatype)));
        }

        match to_edit_condition {
            r#where::SubQHashMapAllowType::Checker(check) => {
                (*check).borrow_mut().datatype = left_datatype;
            }

            r#where::SubQHashMapAllowType::Cond(cond) => {
                (*cond).borrow_mut().datatype = left_datatype;
            }
        }
    }

    let semantic_parser_file_as_struct = SemanticParserFile {
        tables: semantic_parser_table_hashmap,
        aggregates: semantic_parser_aggregates,
        conditions: where_clause_as_struct,
        subquery_hashmap,
    };

    Ok(semantic_parser_file_as_struct)
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

    let semantic_parser_file_as_struct = parse_syntaxic_struct(&syntaxic_file_content_as_struct, &table_metadata_as_struct)?;

    let output_semantic_file_as_str = serde_json::to_string(&semantic_parser_file_as_struct).expect("Error whilst serialising semantic file struct.");

    let mut output_semantic_file = File::options().read(true).write(true).create(true).open("data/SemanticTestData/FSE_1.json").expect("Error whilst creating semantic parser output file");

    output_semantic_file.set_len(0).expect("Error whilst reinitialising semantic output file.");
    output_semantic_file.write_all(output_semantic_file_as_str.as_bytes()).expect("Error occurred whilst writing to semantic output file.");
    output_semantic_file.seek(SeekFrom::Start(0)).expect("Error whilst seeking in semantic output file.");

    Ok(output_semantic_file)
}