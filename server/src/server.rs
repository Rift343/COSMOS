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
    // Receive the length of the message from the client
    let mut len_bytes = [0; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    let msg_len = u32::from_be_bytes(len_bytes) as usize;

    // Use a loop to receive the message in chunks until all bytes are received
    let mut buffer = vec![0; msg_len];
    let mut bytes_received = 0;
    while bytes_received < msg_len {
        match stream.read(&mut buffer[bytes_received..]) {
            Ok(n) if n > 0 => {
                bytes_received += n;
            }
            Ok(_) | Err(_) => {
                // Handle read errors or cases where no bytes were received
                println!("Error(?) when receiving bytes from the client (0 bytes received)");
                break;
            }
        }
    }
    let msg = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", msg);

    // Processing the request
    let req_string = msg.trim_end_matches(char::from(0)).to_string();
    println!("Request after processing: {}", req_string);

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
