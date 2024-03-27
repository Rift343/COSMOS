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

#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
    pub etype: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub etype: String,
    pub use_name_table: String,
    pub attribute_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubQuery {
    pub etype: String,
    pub query: Box<SemanticParserFile>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ConditionAllowType {
    Attribute(Attribute),
    Constant(Constant),
    SubQuery(SubQuery)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition{
    pub etype: String,
    pub condition: String,
    pub datatype: String,
    pub left: ConditionAllowType,
    pub right: ConditionAllowType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataList {
    pub etype: String,
    pub value: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CheckerAllowType {
    DataList(DataList),
    SubQuery(SubQuery)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Checker {
    pub etype: String,
    pub check_type: String,
    pub datatype: String,
    pub left: SubQuery,
    pub right: CheckerAllowType
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LogicalAllowType {
    Condition(Condition),
    Logical(Box<Logical>),
    Checker(Checker)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logical {
    pub etype: String,
    pub operator: String,
    pub left: LogicalAllowType,
    pub right: LogicalAllowType
}

/// Structure representing the contents of the Semantic Parser File, being a dictionary with four keys : tables, conditions, status and error
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticParserFile {
    pub tables: HashMap<String, TableHashmap>,
    pub aggregates: Vec<AggregateHashmap>,
    pub conditions: Logical,
}