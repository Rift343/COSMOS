use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
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
    pub query: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ConditionAllowType {
    Attr(Attribute),
    Const(Constant),
    SubQ(SubQuery)
}

impl Display for ConditionAllowType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionAllowType::Attr(inners) => {
                write!(f, "{}.{}", &inners.use_name_table, &inners.attribute_name)
            }
            ConditionAllowType::Const(inners) => {
                write!(f, "{}", inners.value)
            }
            ConditionAllowType::SubQ(_) => {
                // We shouldn't be trying to display / format a SubQ in our code,
                // If we do, we have reached an unexpected state.
                unimplemented!()
            }
        }
    }
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
    Cond(Rc<RefCell<Condition>>),
    Logi(Box<Logical>),
    Not(Box<LogicalNot>),
    Check(Checker)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logical {
    pub etype: String,
    pub operator: String,
    pub left: LogicalAllowType,
    pub right: LogicalAllowType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogicalNot {
    pub etype: String,
    pub operator: String,
    pub left: LogicalAllowType
}

/// Structure representing the contents of the Semantic Parser File, being a dictionary with four keys : tables, conditions, status and error
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticParserFile {
    pub tables: HashMap<String, TableHashmap>,
    pub aggregates: Vec<AggregateHashmap>,
    pub conditions: LogicalAllowType,
    pub subquery_hashmap: HashMap<String, SemanticParserFile>
}