use std::result;
use csv::Reader;

fn main() {
    println!("Hello, world!");

    let filename = "engine/TestCosmosFichierResultat.csv";
    let result = csv_to_string(filename);
    println!("Result {}",result);
}




fn csv_to_string(file_name : &str) -> String {
    //string resultat
    let mut res = String::new();
    //ouverture du reader
    let mut rdr = Reader::from_path(file_name).expect("Error Main ");

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
    return res;
}




/*
fn csv_to_string(file : &str) -> Result<(), Box<dyn Error>> {
    //string resultat
    let mut res = String::new();
    //ouverture du reader
    let mut rdr = Reader::from_path("TestCosmosFichierResultat.csv")?;
    //recuperation du header
    let mut hd_print = rdr.headers();
    println!("{:?}", hd_print);

    //passage du header en string
    let hd_print_string = hd_print.unwrap();
    println!("hd_print_string {:?}", hd_print_string);
    let hd_print_string_index = &hd_print_string[0];
    println!("{:?}", hd_print_string_index);

    //longueur du header
    let nb_colonne = hd_print_string.len();
    println!("{:?}", nb_colonne);




    let data = rdr.records();
    let  mut nb_ligne = 0;


    for result in data {
        let record = result?;
        println!("{:?}", record);
        nb_ligne += 1;
    }
    println!("{}",nb_ligne);



    Ok(())
}


*/