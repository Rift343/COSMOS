/*
mod utils;

use utils::error_printer::error_printer;
use utils::result_printer::result_printer;
use utils::request_receiver::request_receiver;
 */

fn main() {

    /*
    let req_receiver = request_receiver();


    //match resultat de request receiver
    match req_receiver {
        //Si on arrive a lire la requete dans l'entrée standart
        //On envoie la requete a l'engine
        Ok(req) => match engine::engine_main(req){
            Ok(res) => result_printer(res),
            Err(err) => error_printer(err)
        }
        Err(e) => error_printer(e)

    }


    println!("Main View : fini");

    /*
    let res_test = "res final ID;Nom ;Prenom;Date de naissance;
    1;Ali;Jean;01/01/2001;
    2;Baba;Paul;02/02/2002;
    3;Coucou;Pierre;03/03/2003;
    4;Didi;Jacques;04/04/2004;";
        result_printer(res_test);

     */ */
}
