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

    if request.len() > 0 {
        request.pop();
    }
    Ok(request)
}

pub fn result_printer(result: String){
    println!("{}",result);
}