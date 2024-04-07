use std::fs::File;
use std::mem::replace;
use std::thread::available_parallelism;
use engine::engine;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};
use csv::Writer;
use sysinfo::{System};
use std::time::Instant;

struct Bench{
    temps : f64,
    cpu : f32,
    ram : u64
}

fn bench_to_csv(requete : String, db_size : i32, res : Vec<Bench>){
    //ouverture du reader
    let mut w1 = Writer::from_path("benches/res/data.csv").expect("Bench Error Open CSV output");

    //ecriture en tete
    w1.write_record(&["Requete; Taille CSV Input; Temps (Microsecondes) ; CPU %; RAM Kilobytes;"]).expect("Bench Error write csv output");

    //ajout données
    //Ecriture dans le fichier
    let s = "; ";
    let mut res_cpu;
    let mut res_ram;
    for i in 0..=res.len()-1{
        res_cpu = (res[i].cpu*100.0).round().to_string();
        res_ram = (res[i].ram /1e6 as u64).to_string();
        w1.write_record(&[requete.clone().to_string() + &*" ".to_string() + &*db_size.clone().to_string()+ s + &*res[i].temps.round().to_string() + s + &*res_cpu+ s + &*res_ram + s]).expect("Bench Error Write Data");
    }
    let taille =(res.len() +1).to_string();
    w1.write_record(&[" Average :; ;=MOYENNE(C2:C".to_owned() + &*taille +") ;=MOYENNE(D2:D"+ &*taille +")/100;=MOYENNE(E2:E"+ &*taille +");"]).expect("Bench Error write csv output");
    //wtr.write_record(&[id_string,first_name,last_name,age_str])?;

    w1.flush().expect("Bench Error Flush Writer");
    println!("Fin de l'écriture fichier");
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
    let nb_iter = 6;
    //refresh before bench
    sys.refresh_all();
    sys.refresh_all();

    let nb_cpu = sys.cpus().iter().count() as f32;

    // start timer
    let now = Instant::now();
    //bench

    let mut threads = Vec::new();
    //start all threads
    for _i in 0..nb_iter{
        let cloned_request = request.clone();
        println!("CLONED REQUEST {}",cloned_request);
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        threads.push(std::thread::spawn(move|| engine_benchmark_thread (cloned_request)));

    }

    for _i in 0..nb_iter{
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();
        for p in sys.processes_by_name("src-aa2a"){

            println!("PID {}:{}: {}:{}", p.pid(), p.name(), p.cpu_usage(),p.virtual_memory());
            sum_cpu += p.cpu_usage();
            sum_ram += p.virtual_memory();
        }
    }

    println!("*\
    ------------------------------------\
    ");

    for thread in threads {

        thread.join().expect("Thread Join Issue");

    }


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
    let db_size= 10000;
    let request = "Select ID From Personne;".to_string();

    //tab result

    let mut res : Vec<Bench> = Vec::new();



    for _i in 0..=nb_test{
        res.push(engine_benchmark_custom(request.clone()));
    }


    bench_to_csv(request.clone(),db_size,res);


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