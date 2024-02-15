use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoupleTypeColonne {
    pub column_name: String,
    pub column_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableAttributs {
    nom_table: String,
    liste_attributs: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constraint {
    pub constraint_name: String,
    pub constraint_type: String,
    pub attribut_list: Vec<String>,
    pub foreign_key: Option<Vec<TableAttributs>>,
    pub check: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableMetadonnes {
    pub table_name: String,
    pub columns: Vec<CoupleTypeColonne>,
    pub constraints: Vec<Constraint>
}

#[derive(Serialize, Deserialize, Debug)]
struct FichierMetadonnes {
    liste_metadonnees: Vec<TableMetadonnes>
}