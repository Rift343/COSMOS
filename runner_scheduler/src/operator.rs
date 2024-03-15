use std::collections::HashMap;
use std::fs::OpenOptions;
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
    pub(crate) name:String,
    pub(crate) descriptor:Vec<Vec<String>>
}

#[allow(unused)]

impl CSVFile {

#[doc = "Method to add a column on a table when we use a agregate methode (SUM,MIN,MAX...)\n"]
pub(crate) fn add_column_for_agregate(&mut self,column:&Vec<String>)
{
    self.descriptor[0].push(column[0].to_string());
    for i in 1..self.descriptor.len()
    {
        self.descriptor[i].push(column[0].to_string());
    }
} 



pub(crate)fn set_name(&mut self, name:&String)
{
    self.name = name.to_string();
}

pub(crate)fn set_descriptor(&mut self, list_data:&Vec<String>)
{
    let mut vec1 = Vec::new();
    vec1.push(list_data.to_vec());
    self.descriptor = vec1;
}


#[doc = "Methode to count the number of line for a columns. If the parameter is \"*\"then the NULL and NIL value are not counted.
return a Vec of string with the name of the attribute and the value of the COUNT.
TODO ==> Need to be tested"]
pub(crate)fn count(self,attribut_count:&String) -> Vec<String>
{
    let mut result_vec = Vec::new();
    result_vec.push("COUNT(".to_string()+attribut_count+")");
    let mut counter:usize = 0;
    if attribut_count=="*" {counter = self.descriptor.len();}//If we have * then we just have to count the numbers of rows with ".len()" method
    else//Else we have to count all the row, excluding the NULL and NIL value
    { 
            let mut index: usize = 0;
            for i in 0..self.descriptor[0].len()
            {
                if self.descriptor[0][i] == attribut_count.to_string()
                {
                    index = i;
                    break;
                }
            }
        
            for i in 1..self.descriptor.len()
            {
                if (self.descriptor[i][index] != "NULL" || self.descriptor[i][index] != "NILL")
                {
                    counter = counter+1;
                }
                
            }
        

        }
        
    result_vec.push(counter.to_string());

    return result_vec;
}


pub(crate)fn sum(self,attribut: &String,type_attr:&String)->Vec<String>
{
    let mut result_vec = Vec::new();
    result_vec.push("SUM(".to_string()+attribut+")");
    let mut sum = 0;
    let mut sum2 : f64 = 0.0;
    let mut index: usize = 0;
    for i in 0..self.descriptor[0].len()
    {
        if self.descriptor[0][i] == attribut.to_string()
        {
            index = i;
            break;
        }
    }
    for i in 1..self.descriptor.len()
    {
        if (self.descriptor[i][index] != "NULL" || self.descriptor[i][index] != "NILL")
        {
            if type_attr == "INTEGER"
            {
                let value : i128 = self.descriptor[i][index].parse().unwrap();
                sum=sum+value;
            }
            else if type_attr == "FLOAT"
            {
                let value : f64 = self.descriptor[i][index].parse().unwrap();
                sum2=sum2+value;
            }
        }
                
    }
    if type_attr == "INTEGER"
            {
                result_vec.push(sum.to_string());
                return result_vec;
            }
    else if type_attr == "FLOAT"
    {
        result_vec.push(sum2.to_string());
        return result_vec;
    }
    else {
        result_vec.push("Err".to_string());
        return result_vec;
    }
    
}

pub(crate)fn min(self,attribut: &String,type_attr: &String)->Vec<String>
{
    let mut return_vec = Vec::new();
    return_vec.push("MIN(".to_string()+attribut+")");
    let mut index=0;
    for i in 0..self.descriptor[0].len()
    {
        if self.descriptor[0][i] == attribut.to_string()
        {
            index = i;
            break;
        }
    }
    if (type_attr=="FLOAT"||type_attr=="INTEGER")
    {
        let mut minus_str = &self.descriptor[1][index].clone();
        let mut minus:f64 = minus_str.parse().unwrap();
        for i in 2..self.descriptor.len()
        {
            let mut minus_str = &self.descriptor[i][index].clone();
            let test :f64 = minus_str.parse().unwrap();
            if test < minus
            {
                minus = test;
            }
        }
        return_vec.push(minus.to_string());
    }
    else 
    {
        let mut minus = &self.descriptor[1][index];
        for i in 2..self.descriptor.len()
        {
            let test = &self.descriptor[i][index];
            if test < minus
            {
                minus = test;
            }
        }
        return_vec.push(minus.to_string());    
    }
    return return_vec;
}


pub(crate)fn max(self,attribut: &String,type_attr: &String)->Vec<String>
{
    let mut return_vec = Vec::new();
    return_vec.push("MAX(".to_string()+attribut+")");
    let mut index=0;
    for i in 0..self.descriptor[0].len()
    {
        if self.descriptor[0][i] == attribut.to_string()
        {
            index = i;
            break;
        }
    }
    if (type_attr=="FLOAT"||type_attr=="INTEGER")
    {
        let mut maxi_str = &self.descriptor[1][index].clone();
        let mut maxi:f64 = maxi_str.parse().unwrap();
        for i in 2..self.descriptor.len()
        {
            let mut maxi_str = &self.descriptor[i][index].clone();
            let test :f64 = maxi_str.parse().unwrap();
            if test > maxi
            {
                maxi = test;
            }
        }
        return_vec.push(maxi.to_string());
    }
    else 
    {
        let mut maxi = &self.descriptor[1][index];
        for i in 2..self.descriptor.len()
        {
            let test = &self.descriptor[i][index];
            if test > maxi
            {
                maxi = test;
            }
        }
        return_vec.push(maxi.to_string());    
    }
    return return_vec;
}


#[doc =r"Write a CSV file with the descriptor in ./data/transferFile/result.csv file "]
pub(crate)fn to_file(&self)->Result<File,Box<dyn Error>>{
        let mut file:File = match OpenOptions::new().read(true).write(true).truncate(true).create(true).open("./data/transferFile/result.csv") {
            Ok(e) => e,
            Err(e) =>  return Err(Box::new(e)),
        };
        
        //File::create("./data/transferFile/result.csv").expect("Error : Can't create the resultFile");
        file.write_all(self.to_string().as_bytes());
        file.rewind();
        Ok(file)
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
    //println!("{:?}",self.to_string());
    //println!("{:?}",list_attribute);
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
#[doc = "Method for the cartesian product. Need another CSVFile in input and the self object take the cartesian product between self and another_csv\n
Complexity of O(n²)"]
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

pub(crate) fn replace_as (&mut self,dico:&HashMap<String,String>)
{
    for i in 0..self.descriptor[0].len()
    {
        if dico.contains_key(&self.descriptor[0][i].to_string())
        {
            self.descriptor[0][i]=dico[&self.descriptor[0][i].to_string()].to_string();
        }
    }
}

#[doc = "methode for the union betwen two CSVFile. Need in input anoter CSVFile. Return nothing because the result of the union is save on the struct."]
pub(crate)fn union(&mut self,union_csv:&CSVFile)
{
    let mut result_operation : &mut Vec<Vec<String>> = &mut self.descriptor;
    let mut union_value = &union_csv.descriptor;
    for i in 1..union_value.len()
    {
        if (result_operation[i]!=union_value[i])
        {
            let val = &union_value[i];
            result_operation.push(val.to_vec());
        }
    }
    self.descriptor = result_operation.to_vec();
}




}


#[allow(unused)]
#[doc = r"This fonction take the name of the CSV file and read this file in the ../data/CSV/ directory. That function return of Vec of Vec of String who represent the CSV file ligne by ligne"]
pub(crate)fn csv_read_by_ligne(path_file:String,table_name:String)-> Result<Vec<Vec<String>>,Box<dyn Error>>{
    let mut path:String = "./data/CSV/".to_string();
    path.push_str(&path_file);
    path.push_str(".csv");
    let reader = match File::open(path) {
        Ok(e) => e,
        Err(e) => return Err(Box::new(e)),
    };
    let mut buffer = BufReader::new(reader);
    let mut csv_string = String::new();
    let i: usize = match buffer.read_to_string(&mut csv_string) {
        Err(e) => return  Err(Box::new(e)),
        Ok(e) => e,
    } ;
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
        final_vec[0][i]= (table_name.clone()+"."+&final_vec[0][i]).to_string();
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
pub(crate)fn open_relation(pathcsv:String,name1:&String)->Result<CSVFile,Box<dyn Error>>{
    match csv_read_by_ligne(pathcsv,name1.to_string()){
        Ok(res) => Ok(CSVFile { name:name1.to_string(), descriptor: res }),
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
    fn test_count()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.ID".to_string();
        let test = table1.count(&path_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,10);
        println!("{:?}",test);
    }

    #[test]
    fn test_sum()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.AGE".to_string();
        let type_str = "INTEGER".to_string();
        let test = table1.sum(&path_str, &type_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,400);
        println!("{:?}",test);
    }

    #[test]
    fn test_min()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.AGE".to_string();
        let type_str = "INTEGER".to_string();
        let test = table1.min(&path_str, &type_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,1);
        println!("{:?}",test);

    }
    #[test]
    fn test_max()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.AGE".to_string();
        let type_str = "INTEGER".to_string();
        let test = table1.max(&path_str, &type_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,85);
        println!("{:?}",test);

    }


    #[test]
    fn test_union()
    {
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let table2 = open_relation("personneTest2".to_string(), &"personneTest2".to_string()).expect("Error");
        table1.union(&table2);
        let str = table1.to_string();
        //print!("{}",str);
        let mut val2 = File::open("./data/expectedFile/unionTest1Expected.csv").expect("error");
        let mut str_compare=String::new();
        let _ = val2.read_to_string(&mut str_compare).expect("Error");
        //str_compare.to_string();
        let string_compare = str_compare.to_string(); 
        if string_compare==str
        {
            assert!(true);
        }
        else 
        {
            assert!(false);
        }
    }

    #[test]
    fn test1(){
        let res = open_relation("personneTest".to_string(), &"personneTest".to_string());
        let mut a1 : CSVFile;
        match res {
            Ok(o) => a1 = o,
            Err(..) => panic!("Error")
         };
        a1.print_csv_file();
        let now = Instant::now();
        println!("{:?}",a1.descriptor[0]);    
        a1.projection(["personneTest.ID".to_string(),"personneTest.PRENOM".to_string()].to_vec());
        let time_passed = now.elapsed();
        println!("The projection with personneTest.csv took {} seconde", time_passed.as_secs());
        let val: String = a1.to_string();
        let mut val2 = File::open("./data/expectedFile/test1Expect.csv").expect("error");
        let mut str_compare=String::new();
        let _ = val2.read_to_string(&mut str_compare).expect("Error");
        str_compare.to_string();
        let string_compare = str_compare.to_string();
        if string_compare==val{
            assert!(true);
        }
        else {
            assert!(false);
        }
        

        //print!("{}",a1.to_string());
        
        //println!("{:?}",a1.descriptor);
        
    }

    #[test]
    #[should_panic]
    fn test_csv_read_ligne(){
        let a1 = "../data/CSV/personneTest.csv".to_string();
        csv_read_by_ligne(a1,"personne".to_string()).expect("TODO: panic message");


    }

    #[test]
    fn test_cartesian() {
        let mut a1 = open_relation("personneTest".to_string(), &"personneTest".to_string());
        let a2 = open_relation("personneTest".to_string(), &"personneTest".to_string());
        match a1{
            Ok(ref mut res1) =>         match a2{
                Ok(res2) => {
                                        res1.cartesian_product(&res2);
                                        let val =res1.to_string();
                                        //println!("{}",val);
                                        let mut val2 = File::open("./data/expectedFile/testCartesian.csv").expect("error");
                                        let mut str_compare=String::new();
                                        let _ = val2.read_to_string(&mut str_compare).expect("Error");
                                        //str_compare.to_string();
                                        let string_compare = str_compare.to_string();
                                        if string_compare==val
                                        {
                                            assert!(true);
                                        }
                                        else 
                                        {
                                            assert!(false);
                                        }
                                    
                                    
                                    
                                    },
                Err(..) => panic!("Error")
            }
            Err(..) => panic!("Error")

        }

        //a1.expect("REASON").cartesian_product( &a2);
        a1.expect("REASON").to_file().expect("error");
    }
}