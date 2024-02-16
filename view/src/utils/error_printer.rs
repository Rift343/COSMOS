use std::error::Error;

pub fn error_printer(error: Box<dyn Error>){

    eprintln!("Error : {:?}",error);
}