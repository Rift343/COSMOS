use std::error::Error;
use std::fs::File;
use std::mem::replace;
use std::thread::{available_parallelism, sleep};
use engine::engine;
use rnglib::{Language, RNG};
use rand::Rng;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};
use csv::Writer;
use sysinfo::{System};
use std::time::{Duration, Instant};

struct Bench{
    temps : f64,
    cpu : f32,
    ram : u64
}
//Write bench result in a csv file
fn bench_to_csv(requete : String, db_size : i32, res : Vec<Bench>,filename : &str){
    //ouverture du reader
    let path = "benches/res/".to_string() + filename + ".csv";
    //"benches/res/data.csv"
    let mut w1 = Writer::from_path(path).expect("Bench Error Open CSV output");

    //ecriture en tete
    w1.write_record(&["Requete; Taille CSV Input; Temps (Microsecondes) ; CPU %; RAM Kilobytes;"]).expect("Bench Error write csv output");

    //ajout données
    //Ecriture dans le fichier
    let s = "; ";
    let mut res_cpu;
    let mut res_ram;
    let mut taille = db_size;
    for i in 0..=res.len()-1{
        res_cpu = (res[i].cpu).round().to_string();
        res_ram = (res[i].ram /1e6 as u64).to_string();
        w1.write_record(&[requete.clone().to_string() + &*" ".to_string() + &*taille.clone().to_string()+ s + &*res[i].temps.round().to_string() + s + &*res_cpu+ s + &*res_ram + s]).expect("Bench Error Write Data");
        taille += 20;
    }
    let taille =(res.len() +1).to_string();
    w1.write_record(&[" Average :; ;=MOYENNE(C2:C".to_owned() + &*taille +") ;=MOYENNE(D2:D"+ &*taille +");=MOYENNE(E2:E"+ &*taille +");"]).expect("Bench Error write csv output");
    //wtr.write_record(&[id_string,first_name,last_name,age_str])?;

    w1.flush().expect("Bench Error Flush Writer");
    println!("Fin de l'écriture fichier");
}
//Append input file
pub fn generator(nb_line : i32) -> Result<(),Box<dyn Error>>{
    //ouverture du reader
    let w1 = Writer::from_path("data/CSV/Personne.csv");
    //gerer en fonction de comment est lance le module
    let mut wtr :Writer<File>;
    //Si le premier path fail, on teste le deuxieme
    match w1 {
        Ok(r) => wtr = r,
        Err(..) => wtr = Writer::from_path("data/CSV/Personne.csv")?
    }


    //definition random name
    let rng_s = RNG::try_from(&Language::Elven).unwrap();
    //def rng age
    let mut rng_age = rand::thread_rng();

    wtr.write_record(&["ID;NOM;PRENOM;AGE;"])?;
    //ajout données
    for x in 0..nb_line{
        //ID
        let id_string = (x+1).to_string();

        //def random name
        let first_name = rng_s.generate_name();
        let last_name = rng_s.generate_name();

        //def random age
        let age_str= rng_age.gen_range(0..100).to_string();

        //Ecriture dans le fichier
        wtr.write_record(&[id_string+";"+ &*first_name +";"+ &*last_name +";"+ &*age_str])?;
        wtr.flush()?;
    }
    Ok(())
}


/*-----------------------CRITERION----------------------------*/

fn engine_benchmark_criterion(c : &mut Criterion, request : String){
    /*-----------------------CRITERION----------------------------*/
    c.bench_function(
        "engine_bench1",
        |b| b.iter(|| engine(request.clone()))
    );
}

/*-----------------------CUSTOM----------------------------*/
fn engine_benchmark_custom(request : String) -> Bench{
    let bench_ram_utilisation = 2000000; //bench ram utilisation in bytes = 0.019 gigaoctets environ
    //init sys
    let mut sys = System::new_all();
    //init before refresh
    let mut sum_cpu  = 0f32;
    let mut sum_ram = 0;
    let nb_iter = 4;
    //refresh before bench
    sys.refresh_all();
    sys.refresh_all();

    let nb_cpu = sys.cpus().iter().count() as f32;

    // start timer
    let now = Instant::now();




    //test
    engine(request.clone()).expect("Erreur engine");
    let time = now.elapsed().as_nanos() as u32;
    let half_time = time/ 2u32;


    let now = Instant::now();
    for _i in 0..nb_iter{
        let mut threads = Vec::new();

        let cloned_request = request.clone();
        threads.push(std::thread::spawn(move|| engine_benchmark_thread (cloned_request)));
        sleep(Duration::new(0,half_time));
        sys.refresh_all();
        for p in sys.processes_by_name("src-aa2a"){
            println!("PID {}:{}: {}:{}", p.pid(), p.name(), p.cpu_usage(),p.virtual_memory());
            sum_cpu += p.cpu_usage();
            sum_ram += p.virtual_memory();
        }
        for thread in threads {
            thread.join().expect("Thread Join Issue");
        }
    }



    //bench
    //let mut threads = Vec::new();

    //start all threads
/*
    for _i in 0..nb_iter{
        let cloned_request = request.clone();
        println!("CLONED REQUEST {}",cloned_request);
        threads.push(std::thread::spawn(move|| engine_benchmark_thread (cloned_request)));
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();
        for p in sys.processes_by_name("src-aa2a"){
            println!("PID {}:{}: {}:{}", p.pid(), p.name(), p.cpu_usage(),p.virtual_memory());
            sum_cpu += p.cpu_usage();
        }

    }


    println!("*\
    ------------------------------------\
    ");
    for _i in 0..nb_iter{
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();
        for p in sys.processes_by_name("src-aa2a"){
            println!("PID {}:{}: {}:{}", p.pid(), p.name(), p.cpu_usage(),p.virtual_memory());
            sum_ram += p.virtual_memory();
        }
        std::thread::sleep(Duration::new(1,0));
    }


    for thread in threads {

        thread.join().expect("Thread Join Issue");

    }



 */

    //end timer
    let mut time = now.elapsed().as_micros() as f64;

    sum_cpu = sum_cpu/nb_iter as f32;//Pour chaque itérations
    sum_ram = sum_ram/nb_iter as u64;
    time = time/nb_iter as f64;

    sum_cpu = sum_cpu/nb_cpu;//Diviser par le nombre de coeurs


    println!("sumcpu : {}, sum_ram {}, time {}",sum_cpu,sum_ram, time);






    let res : Bench = Bench {
        temps: time,
        cpu: sum_cpu,
        ram: sum_ram,
    };
    //println!("RAM before {} RAM after {} resram {}",sum_ram_before,sum_ram_after, res_ram);
    return res;
}


fn engine_benchmark_thread(request: String){
    engine(request).expect("Benchmark : Engine Panic");
}
fn engine_benchmark(c: &mut Criterion) {
    let nb_test = 10;
    let db_size= 1000000;
    let request = "Select ID From Personne;".to_string();

    let mut nb_line = 20000; //20000
    let nb_line_init = nb_line;
    let max_line =  5000000;
    let mut step = 20000;

    let mut avg_bench : Vec<Bench> = Vec::new();

    let mut sum_time: f64 = 0f64;
    let mut sum_cpu : f32 = 0f32;
    let mut sum_ram : u64 = 0;


    match generator(nb_line){
        Ok(..) => println!("Init file Ok"),
        Err(..) => println!("ERROR : init file")
    };

    //warmup

    for _i in 0..100{
        let cloned_request = request.clone();
        engine(cloned_request).expect("ERROR : Warmup Panic");
    }

    //tab result
    while (nb_line <= max_line) {
        println!("---------------Taille en cours : {} -------------------", nb_line);
        let mut res: Vec<Bench> = Vec::new();
        let mut cpu_v = 0f32;
        let mut ram_v: u64 = 0;
        //TEST ICI
        //init
        let mut sys = System::new_all();
        //refresh before bench
        sys.refresh_all();
        sys.refresh_all();

        //let nb_cpu = sys.cpus().iter().count() as f32;

        // start timer
        let now = Instant::now();

        //test
        engine(request.clone()).expect("Erreur engine");
        let time = now.elapsed().as_nanos() as u32;
        let half_time = time / 4u32;

        while res.len() < nb_test {
            let mut threads = Vec::new();
            let cloned_request = request.clone();
            let now = Instant::now();
            threads.push(std::thread::spawn(move || engine_benchmark_thread(cloned_request)));
            sleep(Duration::new(0, half_time));
            sys.refresh_all();
            for p in sys.processes_by_name("src-") {
                println!("PID {}:{}: {}:{}", p.pid(), p.name(), p.cpu_usage(), p.virtual_memory());
                cpu_v = p.cpu_usage();
                ram_v = p.virtual_memory();
            }
            for thread in threads {
                thread.join().expect("Thread Join Issue");
            }
            let time = now.elapsed().as_millis() as f64;

            //Pour l'average
            sum_cpu += cpu_v;
            sum_ram += ram_v;
            sum_time += time;


            let ben: Bench = Bench {
                temps: time,
                cpu: cpu_v,
                ram: ram_v,
            };

            res.push(ben);
        }


        /*
    for _i in 0..=nb_test{
        res.push(engine_benchmark_custom(request.clone()));
    }



 */

        let mut filename = "data".to_owned() + &*nb_line.to_string();
        //bench_to_csv(request.clone(), db_size, res, &*filename);
        if(nb_line == 1000000){
            step = 1000000;
        }
        nb_line += step;
        generator(nb_line).expect("ERROR : Panic generator");

        let avg : Bench = Bench{
            temps : sum_time/nb_test as f64,
            cpu : sum_cpu/nb_test as f32,
            ram : sum_ram/nb_test as u64
        };
        avg_bench.push(avg);
        sum_time = 0f64;
        sum_cpu = 0f32;
        sum_ram = 0;
    }
    bench_to_csv(request.clone(), nb_line_init/1000,avg_bench,"Average");


// test
    /*
    let num_calcs : u64 = 1;
    let num_iters : u64 = 10;

    let available_cores: u64 = available_parallelism().unwrap().get() as u64; // get info how many threads we can use and use half of them
    let iter_per_core: u64 = num_calcs / available_cores;

    let now = Instant::now();
    for _i in 0..num_iters {
        let mut results = Vec::new();
        let mut threads = Vec::new();
        threads.push(std::thread::spawn(move|| engine_benchmark_thread ("Select ID From Personne;".to_string())));
        for thread in threads {
            results.extend(thread.join());
        }
    }

*/
    //criterion
    //engine_benchmark_criterion(c,request.clone());
}


criterion_group!(benches, engine_benchmark);
criterion_main!(benches);
/*
cargo bench --bench src
cargo bench --bench src -- --save-baseline my_benchmark

 */