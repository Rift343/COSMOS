use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;
use engine::process_request;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer));
    //we call the main program
    let mut req = String::from_utf8_lossy(&buffer);
    //req = req.replace("\n", "").into();
    let mut req_string = req.to_string();
    req_string.pop();
    println!("Request after processing: {}", req_string);
    let result = process_request(req_string).to_string();
    println!("Responding with: {}", result);
    //let msg = b"Hello, my name is server!";
    let _ = stream.write(result.as_bytes());
}

pub fn create_listener(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
