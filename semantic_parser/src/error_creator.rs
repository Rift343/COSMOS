use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

use crate::structures::fichier_parsage_semantique::FichierParsageSemantique;

pub fn create_semantic_error(error: String) -> File{
    let temp = FichierParsageSemantique{
        tables: vec![],
        conditions: None,
        status: false,
        error,
    };

    let mut error_file = File::options().read(true).write(true).create(true).open("semantic_parser/TestData/Erreur.json").expect("Erreur lors de création du fichier d'erreur");

    error_file.set_len(0).expect("Erreur lors de la réinitialisation du fichier");
    error_file.write_all(serde_json::to_string(&temp).as_bytes()).expect("Erreur lors de l'écriture dans le fichier");
    error_file.seek(SeekFrom::Start(0)).expect("Erreur lors du seek");

    error_file
}