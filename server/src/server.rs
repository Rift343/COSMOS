use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;
use engine::process_request;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn write_log(message: &str, log_file_path: &str) -> std::io::Result<()> {
    // Open the log file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file_path)?;

    // Write the log message to the file
    writeln!(file, "{}", message)?;

    Ok(())
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer));
    //we call the main program
    let req = String::from_utf8_lossy(&buffer);
    //req = req.replace("\n", "").into();
    let mut req_string : std::string::String = Default::default();
    req_string = req.to_string();
    req_string = req_string.trim_end_matches(char::from(0)).to_string();
    //req_string.pop();
    println!("Request after processing: {}", req_string);
    
    let res = write_log(&req_string, "./log.txt");

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
