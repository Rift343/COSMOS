use std::fs::{self, OpenOptions};
use std::{fs::File, io::Read};
use std::error::Error;
use std::io::{BufReader, Seek, Write};

#[allow(unused_variables)]
#[allow(unused_must_use)]

#[allow(unused)]
struct Attribute {
    name: String,
    table: String,
}

#[allow(unused)]
#[doc = r" Structur with a name and a 'descriptor' value. The descriptor is a Vec of Vec of String and represent the CSV file ligne by ligne.
Use the function open_relation(pathcsv:String,name1:String) to create a CSVFile object."]

#[derive(Clone)]

pub(crate) struct CSVFile{
    name:String,
    descriptor:Vec<Vec<String>>
}

#[allow(unused)]

impl CSVFile {

#[doc =r"Write a CSV file with the descriptor in ../data/transferFile/result.csv file "]
pub(crate)fn to_file(&self)->File{
        let mut file:File = OpenOptions::new()
        .read(true)
        .write(true)
            .truncate(true)
        .create(true)
        .open("./data/transferFile/result.csv").expect("error");
        
        //File::create("./data/transferFile/result.csv").expect("Error : Can't create the resultFile");
        file.write_all(self.to_string().as_bytes());
        file.rewind();
        file
    }

#[doc = r"To string of the descriptor who separate the attribute with ',' and the ligne with '\\r\\n' if you use a Windows or '\\n' if you use Linux or Max OS."]
pub(crate)fn to_string(&self) -> String{
        let mut result_string : String="".to_string();
        for ligne in 0..self.descriptor.len()-1{
            result_string = result_string+ &self.descriptor[ligne].clone().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";");
            if (std::env::consts::OS == "windows" ){
                result_string = result_string+"\r\n";
            }
            else{
                result_string = result_string+"\n";
            }
        }
        result_string = result_string+ &self.descriptor[self.descriptor.len()-1].clone().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";");
        result_string 
    }

#[doc = r"print the name of the CSVFile, usefull for debug"]
    fn print_csv_file(&self){
        println!("{}",&self.name);
    }

#[doc = r"The projection operator, the method select the columns write in list_attribute. To do this, the projection need to inverse the ligne and columns, that operation cost O(n²). This for a final complexity of O(3n²+2n)"]
pub(crate)fn projection(&mut self,list_attribute:Vec<String>){
        let mut transpose: Vec<Vec<String>> = Vec::new();
        for i in 0..self.descriptor[0].len(){
            transpose.push(Vec::new());
        }
        //println!("{:?}",transpose);
        for i in 0..self.descriptor.len(){//We transpose the matrix to simplify the operation (we change columns and ligne)
            for y in 0..self.descriptor[i].len(){
                let a =&self.descriptor[i][y];
                transpose[y].push(a.to_string());
            }
        }
        //println!("{:?}",transpose);
        let mut pre_result:Vec<Vec<String>>=Vec::new();//we keep the ligne we want
        for i in transpose{
            for y in &list_attribute{
                if i[0]==y.to_string(){
                    let a = &i;
                    pre_result.push(a.to_vec());
                }
            }
        }
        //println!("{:?}",pre_result);
        let mut transpose: Vec<Vec<String>> = Vec::new();//we made anoter transpose to have the ligne a view by ligne
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
pub(crate)fn cartesian_product(&mut self,another_csv: &CSVFile){
    let mut operation_result : Vec<Vec<String>>=Vec::new();
    let mut transition: Vec<String>;
    transition = self.descriptor[0].clone();
    transition.append(&mut another_csv.descriptor[0].clone());
    operation_result.push(transition);
    for i in 1..self.descriptor.len(){
        transition = self.descriptor[i].clone();
        //println!("{:?}",transition);
        for y in 1..another_csv.descriptor.len(){
            let mut transition2 = transition.clone();
            //println!("{:?}{:?}",transition2,&mut another_csv.descriptor[y]);
            transition2.append(&mut another_csv.descriptor[y].clone());
            operation_result.push(transition2);
           }
        
        }
        self.descriptor = operation_result;
    }



}


#[allow(unused)]
#[doc = r"This fonction take the name of the CSV file and read this file in the ../data/CSV/ directory. That function return of Vec of Vec of String who represent the CSV file ligne by ligne"]
pub(crate)fn csv_read_by_ligne(path_file:String)-> Result<Vec<Vec<String>>,Box<dyn Error>>{
    let mut path:String = "./data/CSV/".to_string();
    path.push_str(&path_file);
    path.push_str(".csv");
    let reader = File::open(path).expect("Error there is no file here");
    let mut buffer = BufReader::new(reader);
    let mut csv_string = String::new();
    buffer.read_to_string(&mut csv_string).expect("Can't read this file");
    //println!("{}",std::env::consts::OS);
    let separator_ligne:String= if (std::env::consts::OS == "windows"){
        "\r\n".to_string()
    }
    else {
        "\n".to_string()
    };
    let first_vec :Vec<&str>=csv_string.split(&separator_ligne).collect::<Vec<_>>();
    let mut final_vec: Vec<Vec<_>> = [first_vec[0].split(';').map(|x| x.to_string()).collect()].to_vec();
    for i in 0..final_vec[0].len(){
        final_vec[0][i]= (path_file.clone()+"."+&final_vec[0][i]).to_string();
    }
    for ligne in 1..first_vec.len(){
        final_vec.push(first_vec[ligne].split(';').map(|x| x.to_string()).collect());
    } 
    
    Ok(final_vec)
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
#[doc = r"Create a CSVFile with the name you want and the name of the CSV file to open"]
pub(crate)fn open_relation(pathcsv:String,name1:String)->Result<CSVFile,Box<dyn Error>>{
    match csv_read_by_ligne(pathcsv){
        Ok(res) => Ok(CSVFile { name:name1, descriptor: res }),
        Err(e) => Err(e)
    }
    //let file:CSVFile = CSVFile { name:name1, descriptor:   };/*  = CSVFile { name: name1, descriptor:  } */;
    //return file;
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    #[test]
    fn test1(){
        let res = open_relation("personneTest".to_string(), "R1".to_string());
        let mut a1 : CSVFile;
        match res {
            Ok(o) => a1 = o,
            Err(..) => panic!("Error")
        };
        a1.print_csv_file();
        let now = Instant::now();
        println!("{:?}",a1.descriptor[0]);
        a1.projection(["personneTest.id".to_string(),"personneTest.prenom".to_string()].to_vec());
        let time_passed = now.elapsed();
        println!("The projection with personneTest.csv took {} seconde", time_passed.as_secs());
        a1.to_file();
        //print!("{}",a1.to_string());
        
        //println!("{:?}",a1.descriptor);
        
    }

    #[test]
    #[should_panic]
    fn test_csv_read_ligne(){
        let a1 = "../data/CSV/personneTest.csv".to_string();
        csv_read_by_ligne(a1).expect("TODO: panic message");

    }

    #[test]
    fn test_cartesian() {
        let mut a1 = open_relation("Personne".to_string(), "R1".to_string());
        let a2 = open_relation("Personne".to_string(), "R1".to_string());
        match a1{
            Ok(ref mut res1) =>         match a2{
                Ok(res2) => res1.cartesian_product(&res2),
                Err(..) => panic!("Error")
            }
            Err(..) => panic!("Error")

        }

        //a1.expect("REASON").cartesian_product( &a2);
        a1.expect("REASON").to_file();
    }
}