use std::{fs::File, io::Read};
use std::io::BufReader;
#[allow(unused_variables)]
#[allow(unused_must_use)]

#[allow(unused)]
struct Attribute {
    name: String,
    table: String,
}

#[allow(unused)]
struct CSVFile{
    name:String,
    descriptor:Vec<Vec<String>>
}

#[allow(unused)]
impl CSVFile {
    fn print_csv_file(&self){
        println!("{}",&self.name);
    }
    fn projection(&mut self,list_attribute:Vec<String>){
        let mut transpose: Vec<Vec<String>> = Vec::new();
        for i in 0..self.descriptor[0].len(){
            transpose.push(Vec::new());
        }
        //println!("{:?}",transpose);
        for i in 0..self.descriptor.len(){
            for y in 0..self.descriptor[i].len(){
                let a =&self.descriptor[i][y];
                transpose[y].push(a.to_string());
            }
        }
        //println!("{:?}",transpose);
        let mut pre_result:Vec<Vec<String>>=Vec::new();
        for i in transpose{
            for y in &list_attribute{
                if i[0]==y.to_string(){
                    let a = &i;
                    pre_result.push(a.to_vec());
                }
            }
        }
        //println!("{:?}",pre_result);
        let mut transpose: Vec<Vec<String>> = Vec::new();
        for i in 0..pre_result[0].len(){
            transpose.push(Vec::new());
        }
        //println!("{:?}",transpose);
        for i in 0..pre_result.len(){
            for y in 0..pre_result[i].len(){
                let a =&pre_result[i][y];
                transpose[y].push(a.to_string());
            }
        }
        //println!("{:?}",transpose);
        self.descriptor = transpose.to_vec();
    }
}

#[allow(unused)]
fn csv_read_by_ligne(path_file:String)->Vec<Vec<String>>{
    let reader = File::open(path_file).expect("Error there is no file here");
    let mut buffer = BufReader::new(reader);
    let mut csv_string = String::new();
    buffer.read_to_string(&mut csv_string).expect("Can't read this file");
    println!("{}",std::env::consts::OS);
    let separator_ligne:String;
    if (std::env::consts::OS == "windows"){
        separator_ligne = "\r\n".to_string();
    }
    else {
        separator_ligne = "\n".to_string();
    }
    let first_vec :Vec<&str>=csv_string.split(&separator_ligne).collect::<Vec<_>>();
    let mut final_vec: Vec<Vec<_>> = [first_vec[0].split(';').map(|x| x.to_string()).collect()].to_vec();
    for ligne in 1..first_vec.len(){
        final_vec.push(first_vec[ligne].split(';').map(|x| x.to_string()).collect());
    }
    return final_vec;
}
/*
fn csv_read_by_columns(path_file:String)/*->CSVFile*/{
    let mut reader = File::open(path_file).expect("Error there is no file here");
    let mut buffer = BufReader::new(reader);
    let mut csv_string = String::new();
    buffer.read_to_string(&mut csv_string).expect("Can't read this file");
    println!("{}",csv_string);
}
 */

#[allow(unused)]
fn open_relation(pathcsv:String,name1:String)->CSVFile{
    let file:CSVFile = CSVFile { name:name1, descriptor: csv_read_by_ligne(pathcsv) };/*  = CSVFile { name: name1, descriptor:  } */;
    return file;
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    #[test]
    fn test1(){
        let mut a1 = open_relation("../data/CSV/personnetest.csv".to_string(), "R1".to_string());
        a1.print_csv_file();
        let now = Instant::now();
        a1.projection(["id".to_string(),"prenom".to_string()].to_vec());
        let time_passed = now.elapsed();
        println!("The the projection with personnetest.csv took {} seconde", time_passed.as_secs());
        //println!("{:?}",a1.descriptor);
        
    }

    #[test]
    fn test_csv_read_ligne(){
        let a1 = "../data/CSV/personnetest.csv".to_string();
        csv_read_by_ligne(a1);

    }
}