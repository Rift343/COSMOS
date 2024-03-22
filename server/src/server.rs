use std::io::{self, prelude::*, Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use engine::engine;

// function write_log to write in logs the queries that were received and processed
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

// Function receive_response to receive the query (or any message) from the connected client (as a stream)
fn receive_response(mut stream: &TcpStream) -> io::Result<String> {
    // First we read the response length from the server
    let mut len_bytes = [0; 4];
    stream.read_exact(&mut len_bytes)?;

    // Convert the response length to u32
    let response_len = u32::from_be_bytes(len_bytes) as usize;

    // Allocate a buffer for the response
    let mut response_buffer = vec![0; response_len];

    // We use a loop to receive the response in chunks until all bytes are received to make sure we received all that was planned
    let mut bytes_received = 0;
    while bytes_received < response_len {
        let chunk = &mut response_buffer[bytes_received..];
        match stream.read(chunk) {
            Ok(0) => {
                // End of stream reached prematurely (0 bytes were received)
                return Err(Error::new(ErrorKind::UnexpectedEof, "End of stream reached prematurely"));
            }
            Ok(n) => {
                // Case where we received bytes, we had that number of bytes we received to the total we received
                bytes_received += n;
            }
            // If the reception went bad, we return an error
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }

    // Convert response bytes to String
    let response = String::from_utf8_lossy(&response_buffer[..]).to_string();
    Ok(response)
}

// Function send_query to send the answer of the query (or any message like an error) to the connected client (as a stream)
fn send_query(mut stream: &TcpStream, query: &str) -> io::Result<()> {
    // Convert message to bytes
    let query_bytes = query.as_bytes();
    let query_len = query_bytes.len() as u32; // Length of the message as u32

    // We send the length of the message to the server in bytes
    stream.write_all(&query_len.to_be_bytes())?;

    // We write the message to the server
    let mut bytes_sent = 0;
    while bytes_sent < query_bytes.len() {
        match stream.write(&query_bytes[bytes_sent..]) {
            Ok(n) if n > 0 => {
                bytes_sent += n;
            }
            Ok(_) | Err(_) => {
                // Handle write errors or cases where no bytes were sent
                return Err(Error::new(ErrorKind::Other, "Error sending query to server"));
            }
        }
    }

    Ok(())
}

// Function handle_connection to manage all the processing we need to do (receive query, send it to the engine, process the answer and send it back...)
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

        // We define an exit string that the client will use to stop cleanly the connection, it can be customized
        let exit_string = String::from("exit");
        if query.trim() == exit_string {
            // Exit query, client has closed the connection
            println!("Client closed his connection\n");
            break;
        }

        // Just a print to make sure we sent what was supposed to be sent
        println!("Received query: {}", query);

        // Log the query to keep track of what happened
        if let Err(err) = write_log(&query, "./log.txt") {
            println!("Error writing log: {}", err);
        }

        // Now we process the query by sending it to the engine 
        let result = match engine(query.clone()) {
            Ok(response) => response,
            Err(err) => {
                println!("Responding with: {}", err);
                // Send the error to the client so that he knows he did something bad
                if let Err(err) = send_query(&mut stream, &err.to_string()) {
                    println!("Error sending response to client: {}", err);
                    break;
                }
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

// Function create_listener that will create the stream used to connect and talk to/with the client
pub fn create_listener(addr: String) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
