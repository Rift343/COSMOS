use std::fs::OpenOptions;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use crate::operator::CSVFile;

#[allow(unused_variables)]
#[allow(unused_must_use)]
#[allow(unused)]

pub fn relation_insert(table_name:&String,value_list :&Vec<String>)->Result<i8,Box<dyn Error>>
{ 
    let mut vec = Vec::new();
    vec.push(value_list.to_vec());
    let prototype_csv:CSVFile=CSVFile{ name: table_name.to_string(), descriptor: vec };
    let mut string_prototype_csv;
    if (std::env::consts::OS == "windows" ){
        string_prototype_csv = "\r\n".to_string();
    }
    else{
        string_prototype_csv = "\n".to_string();
    }
    string_prototype_csv=string_prototype_csv+ &prototype_csv.to_string();
    let mut path_prototype = "./data/CSV/".to_string();
    path_prototype.push_str(table_name);
    path_prototype.push_str(".csv");
    let mut file:File = match OpenOptions::new().append(true).open(path_prototype) {
        Ok(e) => e,
        Err(e) =>  return Err(Box::new(e)),
    };
    let _a = file.write_all(string_prototype_csv.as_bytes())?;
    Ok(0)
}

mod tests {
    #[allow(unused)]
    use super::relation_insert;


    #[test]
    fn test_relation_insert()
    {
        let name_test="csv_test2".to_string();
        let mut attribut_test:Vec<String> = Vec::new();
        attribut_test.push("val1".to_string());
        attribut_test.push("val2".to_string());
        attribut_test.push("val3".to_string());
        let _i = relation_insert(&name_test, &attribut_test).expect("errorTest");


    }

}