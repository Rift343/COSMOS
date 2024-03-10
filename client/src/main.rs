use std::io::prelude::*;
use std::net::TcpStream;
use dns_lookup::lookup_host;
fn main() -> std::io::Result<()> {
    //we resolve the address and connect to the server
    let hostname = "pavieroutaboul.fr";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    println!("IPs for {}: {:?}", hostname, ips);
    let ip = ips[0];
    let ip_str = ip.to_string();
    let addr = format!("{}:8880", ip_str);
    //we connect to the server
    let mut stream = TcpStream::connect(addr)?;
    // we ask the user for input
    println!("enter a message to send to the server :");
    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg)?;
    // we write the message to the server
    //msg = msg.trim_end_matches('\n').to_string();
    println!("Sending: '{}'", msg);
    stream.write(msg.as_bytes())?;
    let mut buffer = [0; 512];
    // we read the response from the server
    stream.read(&mut buffer)?;
    //we print the response from the server
    println!("Response: {}", String::from_utf8_lossy(&buffer[..]));
    Ok(())
} // the stream is closed here
