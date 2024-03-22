use serde::{Deserialize, Serialize};

use super::column_table_name_triple_ldd::ColumnTableNameTripleLdd;
use super::table_name_couple::TableNameCouple;


/// Structure representing the Syntaxic Parser File, which is a dictionary with four keys : table_name, columns, status and error
#[derive(Serialize, Deserialize, Debug)]
pub struct SyntaxicParserFileLdd {
    /// Vector of all the requested table names bundled together
    pub table_name: Vec<String>,
    /// Vector of all the requested columns, which we can't yet associate to their table name
    pub columns: Vec<ColumnTableNameTripleLdd>,
    /// Status of the result, set to true if no errors occurred, else false, in which case the error field is filled out
    /// Set to optional to later allow to easily get rid of it
    status: Option<bool>,
    /// If the status is set to false, contains the error which occurred to be displayed to the user
    /// Assumed that the error field may not be filled out (if we received the file, that is because
    /// the engine allowed it, aka no errors have occurred which we need to think about
    error: Option<String>,
    pub action : String,
}