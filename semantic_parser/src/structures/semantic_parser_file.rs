use serde::{Deserialize, Serialize};

use super::column_table_name_couple::ColumnTableNameCouple;

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDictionnary {
    pub table_name: String,
    pub columns: Vec<ColumnTableNameCouple>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticParserFile {
    pub tables: Vec<TableDictionnary>,
    pub conditions: Option<String>,
    pub status: bool,
    pub error: String
}