mod server;
use std::io::Result;

fn main() -> Result<()> {
    // Boot up the rust server :
    server::create_listener(String::from("0.0.0.0:8880"))
}
