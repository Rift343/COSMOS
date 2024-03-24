use serde::{Deserialize, Serialize};

/// Corresponds to a couple which represents a column, by its name and its table name (whom it belongs to)
#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnTableNameTripleLdd {
    pub name: String,
    pub constraints: Vec<String>,
    pub datatype: String,
    pub data: Option<Vec<String>>

}