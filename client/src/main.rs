use std::io::{self, prelude::*, Error, ErrorKind};
use std::net::TcpStream;
use dns_lookup::lookup_host;
//use std::process; // used for exits (old and unclean)


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

fn main() -> std::io::Result<()> {
    // We first ask the user if they want to change the hostname so that they can connect to anything
    println!("Do you want to change the hostname? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let hostname = if input.trim() == "y" {
        // Read the user input for the hostname if they want to change it
        println!("Enter the hostname:");
        let mut hostname = String::new();
        std::io::stdin().read_line(&mut hostname)?;
        hostname.trim().to_string()
    } else {
        // Otherwise, use the default hostname
        "pavieroutaboul.fr".to_string()
    };

    // Resolve the address and connect to the server (here we verify if the hostname provided is correct and we can connect to it)
    let ips: Vec<std::net::IpAddr> = lookup_host(&hostname).unwrap();
    println!("IPs for {}: {:?}", hostname, ips);
    let ip = ips[0];
    let ip_str = ip.to_string();
    let addr = format!("{}:8880", ip_str);

    // We connect to the server
    let stream = TcpStream::connect(addr)?;

    // We loop to wait for a query to send to the server 
    loop {
        // Ask the user for input
        println!("\nEnter a message to send to the server (type 'exit' to quit):");

        let mut msg = String::new();
        std::io::stdin().read_line(&mut msg)?;

        // Check if the user wants to exit :
        if msg.trim() == "exit" {
            println!("Exiting...");
            // Cleanly close the stream
            stream.shutdown(std::net::Shutdown::Both)?;
            break;
        }

        // If it's not "exit", we can send the query to the server
        send_query(&stream, &msg)?;

        // Receive response from the server
        let response = receive_response(&stream)?;
        println!("\nResponse: \n{}", response);
    }

    Ok(())
}
