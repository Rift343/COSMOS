use serde::{Deserialize, Serialize};

use super::couple_nom_colonne::CoupleNomColonne;

#[derive(Serialize, Deserialize, Debug)]
pub struct DicTable{
    pub table_name: String,
    pub columns: Vec<CoupleNomColonne>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FichierParsageSemantique{
    pub tables: Vec<DicTable>,
    pub conditions: Option<String>,
    pub status: bool,
    pub error: String
}