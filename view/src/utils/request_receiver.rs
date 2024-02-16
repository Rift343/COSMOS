use std::error::Error;
use std::io;

pub fn request_receiver() -> Result<String,Box<dyn Error>> {
    println!("Enter a SQL request :");

    let mut request = String::new();

    io::stdin()
        .read_line(&mut request)?;

    Ok(request)
}