use serde::{Deserialize, Serialize};

/// Corresponds to a couple which represents a column, by its name and its table name (whom it belongs to)
#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnTableNameCouple {
    pub table_name: String,
    pub column_name: String,
}
