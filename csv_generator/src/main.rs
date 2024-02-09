use std::error::Error;
use std::io;
use std::time::Instant;
use rnglib::{RNG, Language};
use rand::Rng;
use csv::Writer;

fn main() {
    /*
    let res_input = input();
    match  res_input{
        Ok(i) => match generator(i) {
            Ok(..) => println!("CSV created"),
            Err(e) => return println!("Error Generator {:?}",e)
        },
        Err(e) => return println!("Error Input: {:?} ",e)
    };

     */

    let start = Instant::now();
    let res = generator(200000);
    let end = Instant::now();

    println!("Time : {:?}",end.duration_since(start));
    println!("{:?}",res);
}
fn generator(nb_line : i32) -> Result<(),Box<dyn Error>>{
    //ouverture du reader
    let mut wtr = Writer::from_path("csv_generator/data_set.csv").expect("Error : writer");
    //definition random name
    let rng_s = RNG::try_from(&Language::Elven).unwrap();
    //def rng age
    let mut rng_age = rand::thread_rng();

    wtr.write_record(&["ID","First_name","Last_name","Age"]).expect("Error : write record");
    //ajout données
    for x in 0..nb_line{
        //ID
        let id_string = x.to_string();

        //def random name
        let first_name = rng_s.generate_name();
        let last_name = rng_s.generate_name();

        //def random age
        let age_str= rng_age.gen_range(0..100).to_string();

        //Ecriture dans le fichier
        wtr.write_record(&[id_string,first_name,last_name,age_str]).expect("Error : write record");
        wtr.flush().expect("Error : flush");
    }
    Ok(())
}

fn input() ->Result<i32, Box<dyn Error>> {
    let mut res_input = String::new();

    println!("Entrer nombre de ligne souhaitées : ");

    io::stdin().read_line(&mut res_input)?;

    //on enlève le \n
    res_input = res_input.trim().to_string();

    //conversion int
    let nb_line : i32 = res_input.parse()?;
    Ok(nb_line)
}

/*
fn bench_generator_200_000(){
    let start = Instant::now();
    let res = generator(200000);
    let end = Instant::now();

    let runtime_nanos = start.to(end).num_nanoseconds().expect("Benchmark iter took greater than 2^63 nanoseconds");
    let runtime_secs = runtime_nanos as f64 / 1_000_000_000.0;
    println!("Time : {}",runtime_secs);
}

 */