mod server;
use std::io::Result;
use std::net::TcpStream;

// Function connect_to_server that will connect to the python server
fn connect_to_server() -> Result<TcpStream> {
    TcpStream::connect("127.0.0.1:8000")
}

fn main() -> Result<()> {
    // Connection to the python server 
    match connect_to_server() {
        Ok(stream) => stream,
        Err(err) => {
            println!("Error connecting to server: {}", err);
            return Err(err);
        }
    };

    // Boot up the rust server :
    server::create_listener(String::from("0.0.0.0:8880"))
}
