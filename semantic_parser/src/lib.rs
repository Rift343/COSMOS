mod structures;
mod error_creator;

use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use structures::syntaxic_parser_file::SyntaxicParserFile;

use structures::semantic_parser_file::SemanticParserFile;
use structures::semantic_parser_file::TableDictionnary;

use structures::table_metadata::TableMetadata;

use structures::column_table_name_couple::ColumnTableNameCouple;

use error_creator::create_semantic_error;


fn get_metadata(metadata_file_path: String) -> Vec<TableMetadata> {
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

    match serde_json::from_str(table_metadata_as_str.as_str()) {
        Ok(content) => {
            return content;
        }
        Err(error) => panic!("Error : {}", error)
    }
}


pub fn semantic_parser(mut syntaxic_file: File) -> File {
    let syntaxic_file_content_as_struct: SyntaxicParserFile = {
        let syntaxic_file_content_as_str = {
            let mut contents_of_file = String::new();

            match syntaxic_file.read_to_string(&mut contents_of_file) {
                Ok(_) => (),
                Err(error) => panic!("Error : {}", error)
            }

            contents_of_file
        };

        match serde_json::from_str(syntaxic_file_content_as_str.as_str()) {
            Ok(content) => {
                content
            }
            Err(error) => panic!("Error : {}", error)
        }
    };

    let table_metadata_as_struct: Vec<TableMetadata> = get_metadata("semantic_parser/TestData/FM_1.json".to_string());

    let mut res_printable = SemanticParserFile {
        tables: vec![],
        conditions: None,
        status: true,
        error: "".to_string(),
    };

    for requested_table in syntaxic_file_content_as_struct.table_name {
        let mut found_table = false;

        for table_metadata in &table_metadata_as_struct {
            if table_metadata.table_name == requested_table {
                found_table = true;
            }
        }

        println!("Tables requested : {}\tFound : {}", requested_table, found_table);

        let temp_dic_table = TableDictionnary {
            table_name: requested_table,
            columns: vec![],
        };

        res_printable.tables.push(temp_dic_table);
    }

    for requested_column in syntaxic_file_content_as_struct.columns {
        let mut nb_found = 0;
        let mut corresponding_table: String = "".to_string();

        for table_metadata in &table_metadata_as_struct {
            for column_couple in &table_metadata.columns {
                if (requested_column.column_name == column_couple.column_name) && (requested_column.table_name == table_metadata.table_name) {
                    nb_found += 1;
                    if nb_found == 1 {
                        corresponding_table = table_metadata.table_name.clone();
                    }
                }
            }
        }

        println!("Requested column : {}.{}\t", requested_column.table_name, requested_column.column_name);

        match nb_found {
            0 => {
                return create_semantic_error(format!("Column : {}.{}\nNot found", requested_column.table_name, requested_column.column_name));
            }
            1 => {
                for table in &mut res_printable.tables {
                    if table.table_name == corresponding_table {
                        let temp_couple = ColumnTableNameCouple {
                            table_name: corresponding_table.clone(),
                            column_name: requested_column.column_name.clone(),
                        };

                        // * ?
                        table.columns.push(temp_couple);
                    }
                }
            }
            _ => {
                return create_semantic_error(format!("Column : {}.{}\nAmbiguous request, multiple occurrences in requested table list.", requested_column.table_name, requested_column.column_name));
            }
        }
    }

    let output_semantic_file_as_str = serde_json::to_string(&res_printable).expect("Error whilst serialising semantic file struct.");

    let mut output_semantic_file = File::options().read(true).write(true).create(true).open("semantic_parser/TestData/FSE_1.json").expect("Error whilst creating semantic parser output file");


    output_semantic_file.set_len(0).expect("Error whilst reinitialising semantic output file.");
    output_semantic_file.write_all(output_semantic_file_as_str.as_bytes()).expect("Error occurred whilst writing to semantic output file.");
    output_semantic_file.seek(SeekFrom::Start(0)).expect("Error whilst seeking in semantic output file.");

    output_semantic_file
}
