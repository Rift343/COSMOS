use std::io::{self, prelude::*, Error, ErrorKind};
use std::net::TcpStream;
use dns_lookup::lookup_host;
//use std::process; //pour des exit

fn send_query(mut stream: &TcpStream, query: &str) -> io::Result<()> {
    // Convert query to bytes
    let query_bytes = query.as_bytes();
    let query_len = query_bytes.len() as u32; // Length of the query as u32

    // Send the length of the query to the server
    stream.write_all(&query_len.to_be_bytes())?;

    // Write the query to the server
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

fn receive_response(mut stream: &TcpStream) -> io::Result<String> {
    // Read the response length from the server
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

fn main() -> std::io::Result<()> {
    // Ask the user if they want to change the hostname
    println!("Do you want to change the hostname? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let hostname = if input.trim() == "y" {
        // Read the user input for the hostname
        println!("Enter the hostname:");
        let mut hostname = String::new();
        std::io::stdin().read_line(&mut hostname)?;
        hostname.trim().to_string()
    } else {
        // Use the default hostname
        "pavieroutaboul.fr".to_string()
    };

    // Resolve the address and connect to the server
    let ips: Vec<std::net::IpAddr> = lookup_host(&hostname).unwrap();
    println!("IPs for {}: {:?}", hostname, ips);
    let ip = ips[0];
    let ip_str = ip.to_string();
    let addr = format!("{}:8880", ip_str);

    // Connect to the server
    let stream = TcpStream::connect(addr)?;

    // Ask the user for input
    println!("Enter a message to send to the server (type 'exit' to quit):");

    loop {
        let mut msg = String::new();
        std::io::stdin().read_line(&mut msg)?;

        // Check if the user wants to exit
        if msg.trim() == "exit" {
            println!("Exiting...");
            // Cleanly close the stream
            stream.shutdown(std::net::Shutdown::Both)?;
            break;
        }

        // Send query to the server
        send_query(&stream, &msg)?;

        // Receive response from the server
        let response = receive_response(&stream)?;
        println!("Response: {}", response);
    }

    Ok(())
} // The stream is closed here
