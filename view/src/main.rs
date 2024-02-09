mod request_receiver;
mod error_printer;
mod result_printer;

use crate::request_receiver::request_receiver;

use crate::error_printer::error_printer;

use crate::result_printer::result_printer;
fn main() {

    let res_receiver = request_receiver();
    match res_receiver {
        Ok(req) => result_printer(req),
        Err(e) => error_printer(e)
    }
    println!("Main View : fini");








    /*
println!("Hello, world!");
let req = request_receiver();
println!("res {req}");
error_printer(req);
*/

    /*
        let res_test = "res final ID;Nom ;Prenom;Date de naissance;
    1;Ali;Jean;01/01/2001;
    2;Baba;Paul;02/02/2002;
    3;Coucou;Pierre;03/03/2003;
    4;Didi;Jacques;04/04/2004;";
        result_printer(res_test);

     */
}
