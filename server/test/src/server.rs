use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer));
    //we call the main program
    let req = String::from_utf8_lossy(&buffer);

    println!("Responding with: {}", String::from_utf8_lossy(b"Hello, my name is server!"));
    let msg = b"Hello, my name is server!";
    let _ = stream.write(msg);
}

pub fn create_listener(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
