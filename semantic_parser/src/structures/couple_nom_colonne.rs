use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoupleNomColonne {
    pub nom_table: String,
    pub nom_variable: String,
}
