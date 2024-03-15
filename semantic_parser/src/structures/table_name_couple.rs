use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TableNameCouple {
    pub table_name: String,
    pub use_name_table: String,
}
