use std::fs;
use pyo3::prelude::*;

pub fn parseur_syntaxique(sql_request : String) {
    let request : String = sql_request.to_string();
    let mut res_textx :String = "".to_string();
    Python::with_gil(|py| {
        res_textx = get_textx_result(request,py).map_err(|e|{
            e.print_and_set_sys_last_vars(py);
        }).expect("Erreur Python with gil");
    });
    fs::write("data/transferFile/fichierParsageSyntaxique.json",res_textx).expect("Erreur ecriture fichier parsage syntaxique");
}

fn get_textx_result(request: String, py: Python) -> PyResult<String> {
    // Le chemin du fichier correspondant au parseur syntaxique textX
    let fichier_parseur_syntaxique = include_str!("../textX_grammar/syntaxic_parser.py");
    // Var de type &PyModule contenant le code du fichier TextX
    let code_textx = PyModule::from_code(py,fichier_parseur_syntaxique,"syntaxic_parser.py",fichier_parseur_syntaxique).expect("Erreur recuperation code Python");
    // Cherche la fonction "is_valid_sql", l'appelle avec l'argument "select..." et extrait le résultat sous chaine de caractere
    let func_is_valid_sql : &PyAny = code_textx.getattr("is_valid_sql").expect("Erreur recuperation is_valid_sql");
    let res_is_valid_sql : &PyAny = func_is_valid_sql.call1((request,)).expect("Erreur appel func_is_valid_sql");
    let res_textx : String = res_is_valid_sql.extract().expect("Erreur extraction String de res_is_valid_sql");
    // Returns error or String
    Ok(res_textx)
}


/*
*/