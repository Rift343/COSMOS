mod request_receiver;
use crate::request_receiver::request_receiver;


fn main() {
    println!("Hello, world!");
    let req = request_receiver();
    println!("res {req}");

}
