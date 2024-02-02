use std::{fs::File, io::Read};
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
Use the open_relation(pathcsv:String,name1:String) to create a CSVFile object."]
struct CSVFile{
    name:String,
    descriptor:Vec<Vec<String>>
}

#[allow(unused)]
impl CSVFile {

#[doc =r"Write a CSV file with the descriptor in ../data/transferFile/result.csv file "]
    fn to_file(&self)->File{
        let mut file:File = File::create("../data/transferFile/result.csv").expect("Error : Can't create the resultFile");
        file.write_all(self.to_string().as_bytes());
        file.rewind();
        return file;
    }

#[doc = r"To string of the descriptor who separate the attribute with ',' and the ligne with '\\r\\n' if you use a Windows or '\\n' if you use Linux or Max OS."]
    fn to_string(&self) -> String{
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
        return result_string; 
    }

#[doc = r"print the name of the CSVFile, usefull for debug"]
    fn print_csv_file(&self){
        println!("{}",&self.name);
    }

#[doc = r"The projection operator, the method select the columns write in list_attribute. To do this, the projection need to inverse the ligne and columns, that operation cost O(n²). This for a final complexity of O(3n²+2n)"]
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
#[doc = r"This fonction take the name of the CSV file and read this file in the ../data/CSV/ directory. That function return of Vec of Vec of String who represent the CSV file ligne by ligne"]
fn csv_read_by_ligne(path_file:String)->Vec<Vec<String>>{
    let mut path:String = "../data/CSV/".to_string();
    path.push_str(&path_file);
    path.push_str(".csv");
    let reader = File::open(path).expect("Error there is no file here");
    let mut buffer = BufReader::new(reader);
    let mut csv_string = String::new();
    buffer.read_to_string(&mut csv_string).expect("Can't read this file");
    //println!("{}",std::env::consts::OS);
    let separator_ligne:String;
    if (std::env::consts::OS == "windows"){
        separator_ligne = "\r\n".to_string();
    }
    else {
        separator_ligne = "\n".to_string();
    }
    let first_vec :Vec<&str>=csv_string.split(&separator_ligne).collect::<Vec<_>>();
    let mut final_vec: Vec<Vec<_>> = [first_vec[0].split(';').map(|x| x.to_string()).collect()].to_vec();
    for i in 0..final_vec[0].len(){
        final_vec[0][i]= (path_file.clone()+"."+&final_vec[0][i]).to_string();
    }
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
#[doc = r"Create a CSVFile with the name you want and the name of the CSV file to open"]
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
        let mut a1 = open_relation("personnetest".to_string(), "R1".to_string());
        a1.print_csv_file();
        let now = Instant::now();
        println!("{:?}",a1.descriptor[0]);
        a1.projection(["personnetest.id".to_string(),"personnetest.prenom".to_string()].to_vec());
        let time_passed = now.elapsed();
        println!("The projection with personnetest.csv took {} seconde", time_passed.as_secs());
        a1.to_file();
        //print!("{}",a1.to_string());
        
        //println!("{:?}",a1.descriptor);
        
    }

    #[test]
    fn test_csv_read_ligne(){
        let a1 = "../data/CSV/personnetest.csv".to_string();
        csv_read_by_ligne(a1);

    }
}