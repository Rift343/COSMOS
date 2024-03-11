use std::error::Error;
use std::io;
pub fn error_printer(error: Box<dyn Error>){

    eprintln!("Error : {:?}",error);
}


pub fn request_receiver() -> Result<String,Box<dyn Error>> {
    println!("Enter a SQL request :");

    let mut request = String::new();

    io::stdin()
        .read_line(&mut request)?;

    let request_without_n = request.trim_end_matches('\n').to_string();
    let request_without_nr = request_without_n.trim_end_matches('\r').to_string();

    Ok(request_without_nr)
}

pub fn result_printer(result: String){
    println!("{}",result);
}