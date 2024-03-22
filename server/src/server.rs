use std::io::{self, prelude::*, Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use engine::engine;

// Function write_log to write all queries received in a log file 
fn write_log(message: &str, log_file_path: &str) -> std::io::Result<()> {
    // Open the log file in append mode
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file_path)?;

    // Write the received message to the file
    writeln!(file, "{}", message)?;

    Ok(())
}

// Function receive_response to receive the answer of the query (or technically any message) from a connected machine as a stream (here the server)
fn receive_response(mut stream: &TcpStream) -> io::Result<String> {
    // Read the response length from the server (maximum 4 bytes long)
    let mut len_bytes = [0; 4];
    stream.read_exact(&mut len_bytes)?;

    // Convert the answer length to u32
    let response_len = u32::from_be_bytes(len_bytes) as usize;

    // Allocate a buffer for the answer
    let mut response_buffer = vec![0; response_len];

    // We use a loop to receive the response in chunks until all bytes are received
    let mut bytes_received = 0;
    while bytes_received < response_len {
        let chunk = &mut response_buffer[bytes_received..];
        match stream.read(chunk) {
            Ok(0) => {
                // End of stream reached prematurely (0 bytes received, that's an error)
                return Err(Error::new(ErrorKind::UnexpectedEof, "End of stream reached prematurely"));
            }
            Ok(n) => {
                // Here it went well, we had the bytes received to the total bytes received to double check we got the full response
                bytes_received += n;
            }
            // Otherwise we have an error because the stream was interrupted 
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }

    // Convert response bytes to String
    let response = String::from_utf8_lossy(&response_buffer[..]).to_string();
    Ok(response)
}

// Function send_query to send the query (or technically any message like an exit) to a connected machine as a stream (here the server)
fn send_query(mut stream: &TcpStream, query: &str) -> io::Result<()> {
    // We convert the query to bytes
    let query_bytes = query.as_bytes();
    let query_len = query_bytes.len() as u32; // Length of the query as u32

    // Send the length of the query 
    stream.write_all(&query_len.to_be_bytes())?;

    // Send the query
    let mut bytes_sent = 0;
    while bytes_sent < query_bytes.len() {
        match stream.write(&query_bytes[bytes_sent..]) {
            Ok(n) if n > 0 => {
                bytes_sent += n;
            }
            Ok(_) | Err(_) => {
                // Here we handle errors in the case where we had an explicit error or when we sent 0 bytes
                return Err(Error::new(ErrorKind::Other, "Error sending query to server"));
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        // First we receive the query from the client (number of bytes + message)
        let query = match receive_response(&mut stream) {
            Ok(q) => q,
            Err(err) => {
                println!("Error receiving query from client: {}", err);
                break;
            }
        };

        // We define the exit query and we verify if that's what the client sent
        let exit_string =String::from("exit") ;
        if query.trim() == exit_string {
            // Exit query, client has closed the connection
            println!("Client closed his connection\n");
            break;
        }

        // print just to verify the server received the good message/query
        println!("Received query: {}", query);

        // We log the query to the log file
        if let Err(err) = write_log(&query, "./log.txt") {
            println!("Error writing log: {}", err);
        }

        // We process the query by sending it to the engine and receiving its answer 
        let result = match engine(query.clone()) {
            Ok(response) => response,
            Err(err) => {
                println!("Error processing query: {}", err);
                return;
            }
        };

        // print just to verify we have the good answer before sending it to the client
        println!("Responding with: {}", result);

        // Send the response to the client
        if let Err(err) = send_query(&mut stream, &result) {
            println!("Error sending response to client: {}", err);
            break;
        }
    }
}

// Function create_listener that creates the connection between the client and the server 
pub fn create_listener(addr: String) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
