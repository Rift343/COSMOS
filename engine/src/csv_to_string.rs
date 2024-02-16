use csv::Reader;

pub fn engine_main(file_name : String) ->  Result<String, Box<dyn std::error::Error>> {

    //A CHANGER EN file_name DES QU ON PREND LE RES DE O/E
    let filename = "engine/TestCosmosFichierResultat.csv";

    //Convert csv to string
    let result = csv_to_string(filename);
    return result;
}
fn csv_to_string(file_name : &str) -> Result<String, Box<dyn std::error::Error>> {
    //string resultat
    let mut res = String::new();
    //ouverture du reader
    let mut rdr = Reader::from_path(file_name)?;

    //recuperation du header
    let hd = rdr.headers();
    //passage du header en string
    let hd_string = hd.unwrap();

    //compteur nomnbre colonne
    let mut nb_colonne = 0;

    //ajout du header au resultat
    for column_name in hd_string{
        res = res + column_name + ";";
        nb_colonne += 1;
    }

    // on ajoute un \n a la fin de la ligne header
    res = res + "\n";

    //Pour la vue, on compte le nombre de ; avant le \n et on a le nb de colonne
    //ensuite affichage des mots tous les nb colonnes

    //ajout des donnees dans le fichier
    let data = rdr.records();
    //Boucle pour chaque ligne
    for line in data{
        let ldata = line.unwrap();
        //Boucle pour chaque colonne
        for index in 0..nb_colonne{
            res = res + &ldata[index] + ";";
        }
        //fin de ligne donc on rajoute \n
        res += "\n";
    }
    Ok(res)
}