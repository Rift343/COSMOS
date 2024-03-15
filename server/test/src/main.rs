mod server;
use std::io::Result;

fn main() -> Result<()> {
    server::create_listener(String::from("127.0.0.1:8880"))
}
