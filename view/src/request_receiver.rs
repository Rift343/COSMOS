use std::io;

pub fn request_receiver() -> String {
    println!("Enter a SQL request :");

    let mut request = String::new();

    io::stdin()
        .read_line(&mut request)
        .expect("Error : Failed to read line");

    return request;
}