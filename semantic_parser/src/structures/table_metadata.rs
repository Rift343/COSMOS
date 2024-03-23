use serde::{Deserialize, Serialize};

/// Represents a couple which stores columns, contains their name and their type
///
/// Does not store the table name to which it belongs, as it is expected to be nested inside a table
/// (Making the data already accessible)
#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnNameTypeCouple {
    pub column_name: String,
    pub column_type: String,
}

/// Represents a table and its attributes
#[derive(Serialize, Deserialize, Debug)]
pub struct TableAttributes {
    table_name: String,
    attribute_list: Vec<String>
}

/// Represents a constraint on a table of any kind
/// Fields may or may not be filled depending on constraint, constraint type checking is necessary.
#[derive(Serialize, Deserialize, Debug)]
pub struct Constraint {
    pub constraint_name: String,
    pub constraint_type: String,
    pub attribute_list: Vec<String>,
    pub foreign_key: Option<Vec<TableAttributes>>,
    pub check: Option<String>
}

/// Represents a tables metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct TableMetadata {
    pub columns: Vec<ColumnNameTypeCouple>,
    pub constraints: Vec<Constraint>
}

impl TableMetadata {
    pub fn has_attribute(&self, attribute_name: &String) -> bool{
        for table_attribute in &self.columns {
            if table_attribute.column_name == *attribute_name {
                return true
            }
        }

        false
    }
}
