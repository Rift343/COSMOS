mod structures;
mod error_creator;

use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use structures::fichier_parsage_syntaxique::FichierParsageSyntaxique;

use structures::fichier_parsage_semantique::FichierParsageSemantique;
use structures::fichier_parsage_semantique::DicTable;

use structures::table_metadonnee::TableMetadonnes;

use structures::couple_nom_colonne::CoupleNomColonne;

use error_creator::create_semantic_error;

pub fn semantic_parser(mut syntaxic_file: File) -> File {
    let fs1_fichier_str = {
        let mut contenu_fichier_syntaxique = String::new();

        match syntaxic_file.read_to_string(& mut contenu_fichier_syntaxique) {
            Ok(_) => (),
            Err(erreur) => panic!("Erreur : {}", erreur)
        }

        contenu_fichier_syntaxique
    };

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

    let fm1_fichier_str = {
        match fs::read_to_string("semantic_parser/TestData/FM_1.json") {
            Ok(contenu) => {
                contenu
            }
            Err(erreur) => {
                panic!("Erreur : {}", erreur)
            }
        }
    };

    // println!("Contenu du fichier :\n{}\n---", fm1_fichier_str);

    let fm1_as_vec: Vec<TableMetadonnes> = {
        match serde_json::from_str(fm1_fichier_str.as_str()) {
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
            0 => {
                return create_semantic_error("".to_string())
            },
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


    out_file.set_len(0).expect("Erreur lors de la réinitialisation du fichier");
    out_file.write_all(ress.as_bytes()).expect("Erreur lors de l'écriture dans le fichier");
    out_file.seek(SeekFrom::Start(0)).expect("Erreur lors du seek");

    out_file

}
