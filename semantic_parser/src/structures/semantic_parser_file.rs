use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnNameCouple {
    pub attribute_name: String,
    pub use_name_attribute: String
}


/// Represents a dictionary where the key is the table_name, and the value being a vector of a ColumnTableNameCouple structure
#[derive(Serialize, Deserialize, Debug)]
pub struct TableHashmap {
    pub use_name_table: String,
    pub columns: Vec<ColumnNameCouple>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AggregateHashmap{
    pub use_name_table: String,
    pub attribute_name: String,
    pub use_name_attribute: String,
    pub aggregate_type: String,
    pub attribute_type: String
}

/// Structure representing the contents of the Semantic Parser File, being a dictionary with four keys : tables, conditions, status and error
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticParserFile {
    pub tables: HashMap<String, TableHashmap>,
    pub aggregates: Vec<AggregateHashmap>,
    pub conditions: Option<String>,
}