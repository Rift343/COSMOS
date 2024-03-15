use std::io::prelude::*;
use std::net::TcpStream;
use dns_lookup::lookup_host;

fn main() -> std::io::Result<()> {
    // Resolve the address and connect to the server
    let hostname = "pavieroutaboul.fr";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    println!("IPs for {}: {:?}", hostname, ips);
    let ip = ips[0];
    let ip_str = ip.to_string();
    let addr = format!("{}:8880", ip_str);
    
    // Connect to the server
    let mut stream = TcpStream::connect(addr)?;

    // Ask the user for input
    println!("Enter a message to send to the server:");
    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg)?;
    
    // Convert message to bytes
    let msg_bytes = msg.as_bytes();
    let msg_len = msg_bytes.len() as u32; // Length of the message as u32

    // Send the length of the message to the server
    stream.write_all(&msg_len.to_be_bytes())?;

    // Write the message to the server
    println!("Sending: '{}'", msg);
    let mut bytes_sent = 0;
    while bytes_sent < msg_bytes.len() {
        match stream.write(&msg_bytes[bytes_sent..]) {
            Ok(n) if n > 0 => {
                bytes_sent += n;
            }
            Ok(_) | Err(_) => {
                // Handle write errors or cases where no bytes were sent
                println!("Error(?) when sending bytes to the server (0 bytes sent)");
                break;
            }
        }
    }

    let mut buffer = [0; 512];
    // Read the response from the server
    stream.read(&mut buffer)?;
    // Print the response from the server
    println!("Response: {}", String::from_utf8_lossy(&buffer[..]));
    
    Ok(())
} // The stream is closed here
