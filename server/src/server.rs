use std::io::{self, prelude::*, Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use engine::engine;

fn write_log(message: &str, log_file_path: &str) -> std::io::Result<()> {
    // Open the log file in append mode
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file_path)?;

    // Write the log message to the file
    writeln!(file, "{}", message)?;

    Ok(())
}

fn receive_response(mut stream: &TcpStream) -> io::Result<String> {
    // Read the response length from the client
    let mut len_bytes = [0; 4];
    stream.read_exact(&mut len_bytes)?;

    // Convert the response length to u32
    let response_len = u32::from_be_bytes(len_bytes) as usize;

    // Allocate a buffer for the response
    let mut response_buffer = vec![0; response_len];

    // Use a loop to receive the response in chunks until all bytes are received
    let mut bytes_received = 0;
    while bytes_received < response_len {
        let chunk = &mut response_buffer[bytes_received..];
        match stream.read(chunk) {
            Ok(0) => {
                // End of stream reached prematurely
                return Err(Error::new(ErrorKind::UnexpectedEof, "End of stream reached prematurely"));
            }
            Ok(n) => {
                bytes_received += n;
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }

    // Convert response bytes to String
    let response = String::from_utf8_lossy(&response_buffer[..]).to_string();
    Ok(response)
}

fn send_query(mut stream: &TcpStream, query: &str) -> io::Result<()> {
    // Convert query to bytes
    let query_bytes = query.as_bytes();
    let query_len = query_bytes.len() as u32; // Length of the query as u32

    // Send the length of the query to the client
    stream.write_all(&query_len.to_be_bytes())?;

    // Write the query to the client
    let mut bytes_sent = 0;
    while bytes_sent < query_bytes.len() {
        match stream.write(&query_bytes[bytes_sent..]) {
            Ok(n) if n > 0 => {
                bytes_sent += n;
            }
            Ok(_) | Err(_) => {
                // Handle write errors or cases where no bytes were sent
                return Err(Error::new(ErrorKind::Other, "Error sending query to client"));
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        // Receive the query from the client
        let query = match receive_response(&mut stream) {
            Ok(q) => q,
            Err(err) => {
                println!("Error receiving query from client: {}", err);
                break;
            }
        };

        let exit_string =String::from("exit") ;
        if query.trim() == exit_string {
            // Exit query, client has closed the connection
            println!("Client closed his connection\n");
            break;
        }

        println!("Received query: {}", query);

        // Log the query
        if let Err(err) = write_log(&query, "./log.txt") {
            println!("Error writing log: {}", err);
        }

        // Processing the query
        let result = match engine(query.clone()) {
            Ok(response) => response,
            Err(err) => {
                println!("Error processing query: {}", err);
                return;
            }
        };

        println!("Responding with: {}", result);

        // Send the response to the client
        if let Err(err) = send_query(&mut stream, &result) {
            println!("Error sending response to client: {}", err);
            break;
        }
    }
}


pub fn create_listener(addr: String) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
