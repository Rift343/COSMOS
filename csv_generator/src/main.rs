use std::io;
use rnglib::{RNG, Language};
use rand::Rng;
use csv::Writer;

fn main() {

    println!("Entrer nombre de ligne souhaitées : ");

    let mut res_input = String::new();

    io::stdin()
        .read_line(&mut res_input)
        .expect("Error : Failed to read line");

    //on enlève le \n
    let len = res_input.len();
    res_input.truncate(len - 1);

    //conversion int
    let nb_line : i32 = res_input.parse().expect("Error nbline");

    //ouverture du reader
    let mut wtr = Writer::from_path("data_set.csv").expect("Error : writer");
    //definition random name
    let rng_s = RNG::try_from(&Language::Elven).unwrap();

    wtr.write_record(&["ID","First_name","Last_name","Age"]).expect("Error : write record");
    //ajout données
    for x in 0..nb_line{
        let id = x;
        let id_string = id.to_string();
        let first_name = rng_s.generate_name();
        let last_name = rng_s.generate_name();
        //def random age

        let age: i32 = rand::thread_rng().gen_range(0..100);
        let age_str = age.to_string();

        //Ecriture dans le fichier
        wtr.write_record(&[id_string,first_name,last_name,age_str]).expect("Error : write record");
        wtr.flush().expect("Error : flush");

    }
}
