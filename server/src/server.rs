use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;
use engine::engine;
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
    
    // Processing the request
    let req = String::from_utf8_lossy(&buffer);
    let req_string = req.trim_end_matches(char::from(0)).to_string();
    println!("Request after processing: {}", req_string);
    let exit_string =String::from("exit\n") ;
    if(req_string==exit_string){
        println!("Le client nous abandonne");
        return;
    }
    // Writing the request to the log
    let res = write_log(&req_string, "./log.txt");

    // Calling the engine function instead of process_request
    let result = match engine(req_string) {
        Ok(response) => response,
        Err(err) => {
            let err_msg = format!("Error processing request: {}", err);
            println!("{}", err_msg);
            // Send the error message through TCP
            let _ = stream.write(err_msg.as_bytes());
            return;
        }
    };
    println!("Responding with: {}", result);
    
    // Writing the response back to the client
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
