use serde::{Deserialize, Serialize};

use super::couple_nom_colonne::CoupleNomColonne;


#[derive(Serialize, Deserialize, Debug)]
pub struct FichierParsageSyntaxique {
    pub table_name: Vec<String>,
    pub columns: Vec<CoupleNomColonne>,
    status: bool,
    error: Option<String>
}