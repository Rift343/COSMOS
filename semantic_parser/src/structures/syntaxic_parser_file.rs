use serde::{Deserialize, Serialize};

use super::column_table_name_couple::ColumnTableNameCouple;


#[derive(Serialize, Deserialize, Debug)]
pub struct SyntaxicParserFile {
    pub table_name: Vec<String>,
    pub columns: Vec<ColumnTableNameCouple>,
    status: bool,
    error: Option<String>
}