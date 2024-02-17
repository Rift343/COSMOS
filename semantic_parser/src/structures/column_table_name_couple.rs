use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnTableNameCouple {
    pub table_name: String,
    pub column_name: String,
}
