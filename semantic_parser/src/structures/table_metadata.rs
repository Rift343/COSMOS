use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnNameTypeCouple {
    pub column_name: String,
    pub column_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableAttributes {
    table_name: String,
    attribute_list: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constraint {
    pub constraint_name: String,
    pub constraint_type: String,
    pub attribute_list: Vec<String>,
    pub foreign_key: Option<Vec<TableAttributes>>,
    pub check: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableMetadata {
    pub table_name: String,
    pub columns: Vec<ColumnNameTypeCouple>,
    pub constraints: Vec<Constraint>
}

#[derive(Serialize, Deserialize, Debug)]
struct FichierMetadonnes {
    liste_metadonnees: Vec<TableMetadata>
}