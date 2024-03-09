use serde::{Deserialize, Serialize};
use crate::structures::table_name_couple::TableNameCouple;

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnNameCouple {
    pub attribute_name: String,
    pub use_name_attribute: String
}


/// Represents a dictionary where the key is the table_name, and the value being a vector of a ColumnTableNameCouple structure
#[derive(Serialize, Deserialize, Debug)]
pub struct TableDictionary {
    pub table_name: TableNameCouple,
    pub columns: Vec<ColumnNameCouple>
}

/// Structure representing the contents of the Semantic Parser File, being a dictionary with four keys : tables, conditions, status and error
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticParserFile {
    pub tables: Vec<TableDictionary>,
    pub conditions: Option<String>,
    /// Status of the result, set to true if no errors occurred, else false, in which case the error field is filled out
    pub status: bool,
    /// If the status is set to false, contains the error which occurred to be displayed to the user
    pub error: String
}