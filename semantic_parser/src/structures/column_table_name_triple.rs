use serde::{Deserialize, Serialize};

/// Corresponds to a couple which represents a column, by its name and its table name (whom it belongs to)
#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnTableNameTriple {
    pub use_name_table: String,
    pub attribute_name: String,
    pub use_name_attribute: String
}
