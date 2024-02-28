pub mod generator_mod{
    use std::error::Error;
    use std::io;
    use csv::Writer;
    use rand::Rng;
    use rnglib::{Language, RNG};

    //csv generator nbline -> csv + return () or Error
    pub fn generator(nb_line : i32) -> Result<(),Box<dyn Error>>{
        //ouverture du reader
        let mut wtr = Writer::from_path("data_set.csv")?;
        //definition random name
        let rng_s = RNG::try_from(&Language::Elven).unwrap();
        //def rng age
        let mut rng_age = rand::thread_rng();

        wtr.write_record(&["ID","First_name","Last_name","Age"])?;
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
            wtr.write_record(&[id_string,first_name,last_name,age_str])?;
            wtr.flush()?;
        }
        Ok(())
    }
    //User input of an i32
    pub fn input() ->Result<i32, Box<dyn Error>> {
        let mut res_input = String::new();

        println!("Entrer nombre de ligne souhaitées : ");

        io::stdin().read_line(&mut res_input)?;

        //on enlève le \n
        res_input = res_input.trim().to_string();

        //conversion int
        let nb_line : i32 = res_input.parse()?;
        Ok(nb_line)
    }


}