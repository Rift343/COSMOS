use serde::{Deserialize, Serialize};

use super::column_table_name_triple::ColumnTableNameTriple;
use super::table_name_couple::TableNameCouple;


/// Structure representing the Syntaxic Parser File, which is a dictionary with four keys : table_name, columns, status and error
#[derive(Serialize, Deserialize, Debug)]
pub struct SyntaxicParserFile {
    /// Vector of all the requested table names bundled together
    pub table_name: Vec<TableNameCouple>,
    /// Vector of all the requested columns, which we can't yet associate to their table name
    pub columns: Vec<ColumnTableNameTriple>,
}