mod structures;

use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use serde_json::Result;

use structures::fichier_parsage_syntaxique::FichierParsageSyntaxique;

use structures::fichier_parsage_semantique::FichierParsageSemantique;
use structures::fichier_parsage_semantique::DicTable;

use structures::table_metadonnee::TableMetadonnes;

use structures::couple_nom_colonne::CoupleNomColonne;

pub fn test() -> File {
    let fs1_filename = String::from("semantic_parser/TestData/FS_1.json");

    let fs1_fichier_str = {
        match fs::read_to_string(fs1_filename) {
            Ok(contenu) => {
                contenu
            }
            Err(erreur) => {
                panic!("Erreur : {}", erreur)
            }
        }
    };

    // println!("Contenu du fichier :\n{}\n---", fs1_fichier_str);

    let fps1_as_fps: FichierParsageSyntaxique = {
        match serde_json::from_str(fs1_fichier_str.as_str()) {
            Ok(contenu) => {
                contenu
            },
            Err(erreur) => panic!("Erreur : {}", erreur)
        }
    };

    println!("Temp : {:?}", fps1_as_fps);

    // ------------------------------------------------

    let fm1_filename = String::from("semantic_parser/TestData/FM_1.json");

    let fm1_fichier_str = {
        match fs::read_to_string(fm1_filename) {
            Ok(contenu) => {
                contenu
            }
            Err(erreur) => {
                panic!("Erreur : {}", erreur)
            }
        }
    };

    // println!("Contenu du fichier :\n{}\n---", fm1_fichier_str);

    let fm1: Result<Vec<TableMetadonnes>> = serde_json::from_str(fm1_fichier_str.as_str());

    let fm1_as_vec: Vec<TableMetadonnes> = {
        match fm1 {
            Ok(contenu) => {
                println!("{:?}", contenu);
                contenu
            },
            Err(erreur) => panic!("Erreur : {}", erreur)
        }
    };

    let mut res_printable = FichierParsageSemantique{
        tables: vec![],
        conditions: None,
        status: true,
        error: "".to_string(),
    };

    for table_fps_demandee in fps1_as_fps.table_name{
        let mut found_table = false;

        for table_metadonnee in &fm1_as_vec{
            if table_metadonnee.table_name == table_fps_demandee{
                found_table = true;
            }
        }

        println!("Tables demandée : {}\tTrouvées : {}", table_fps_demandee, found_table);

        let temp_dic_table = DicTable{
            table_name: table_fps_demandee,
            columns: vec![],
        };

        res_printable.tables.push(temp_dic_table);
    }

    for colonne_demandee in fps1_as_fps.columns{
        let mut nb_found = 0;
        // mut
        let mut table_correspondante: String = "".to_string();

        for table_metadonne in &fm1_as_vec{
            for couple_colonne in &table_metadonne.columns{
                if colonne_demandee.nom_variable == couple_colonne.column_name{
                    nb_found += 1;
                    if nb_found == 1 {
                        table_correspondante = table_metadonne.table_name.clone();
                    }
                }
            }
        }

        print!("Colonne demandée : {}.{}\t", colonne_demandee.nom_table, colonne_demandee.nom_variable);
        match nb_found{
            0 => println!("Non trouvée"),
            1 => {
                println!("Trouvée dans la table : {}", table_correspondante);

                for table in &mut res_printable.tables{
                    if table.table_name == table_correspondante{
                        let temp_couple = CoupleNomColonne{
                            nom_table: table_correspondante.clone(),
                            nom_variable: colonne_demandee.nom_variable.clone(),
                        };

                        // * ?
                        table.columns.push(temp_couple);
                    }
                }
            },
            _ => println!("Trouvée : {} fois", nb_found)
        }
    }

    println!("\n------------------------\n");
    println!("{:?}", res_printable);

    // let fps_filename = String::from("semantic_parser/TestData/FSE_1.json");

    let ress = serde_json::to_string(&res_printable).expect("Erreur lors de la sérialisation");

    // fs::write(fps_filename, ress).expect("Erreur lors de l'écriture dans le fichier");

    let mut out_file = File::options().read(true).write(true).create(true).open("semantic_parser/TestData/FSE_1.json").expect("Erreur lors de création de out_file");

    out_file.write_all(ress.as_bytes()).expect("Erreur lors de l'écriture dans le fichier");
    out_file.seek(SeekFrom::Start(0)).expect("Erreur lors du seek");

    /*
    let mut temp: String = String::new();

    let nb_read = out_file.read_to_string(&mut temp).expect("Erreur lors de lecture du fichier");

    out_file.seek(SeekFrom::Start(0)).expect("Erreur lors du seek");

    println!("Nb read : {}", nb_read);
    */

    out_file

}
