use csv_generator::generator_mod::{generator, input};
fn main() {
    let res_input = input();
    match res_input {
        Ok(i) => match generator(i) {
            Ok(..) => println!("CSV created"),
            Err(e) => return println!("Error Generator {:?}", e)
        },
        
        Err(e) => return println!("Error Input: {:?} ", e)
    };
}
