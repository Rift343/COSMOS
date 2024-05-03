use std::fs::OpenOptions;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use crate::operator::CSVFile;
use crate::relation_insert::relation_insert;

#[allow(unused_variables)]
#[allow(unused_must_use)]
#[allow(unused)]

pub(crate) fn relation_creater(table_name:&String,attribute_list :&Vec<String>)->Result<i8,Box<dyn Error>>
{ 
    let mut vec = Vec::new();
    vec.push(attribute_list.to_vec());
    let prototype_csv:CSVFile=CSVFile{ name: table_name.to_string(), descriptor: vec };
    let string_prototype_csv:String = prototype_csv.to_string();
    let mut path_prototype = "./data/CSV/".to_string();
    path_prototype.push_str(table_name);
    path_prototype.push_str(".csv");
    let mut file:File = match OpenOptions::new().write(true).truncate(true).create(true).open(path_prototype) {
        Ok(e) => e,
        Err(e) =>  return Err(Box::new(e)),
    };
    let _a = file.write_all(string_prototype_csv.as_bytes())?;
    let mut vector: Vec<String> = Vec::new();
    vector.push(table_name.to_string());
    let meta_table = "ALL_TABLES".to_string();
    relation_insert(&meta_table, &vector).expect("error");
    for i in 0..attribute_list.len()
    {
        let mut insert_vec = Vec::new();
        insert_vec.push(table_name.to_string());
        insert_vec.push(attribute_list[i].clone());
        relation_insert(&"ALL_COLUMNS".to_string(), &insert_vec).expect("error");
        drop(insert_vec);
    }
    Ok(0)
}

mod tests {
    #[allow(unused)]
    use super::relation_creater;


    #[test]
    fn test_relation_creater()
    {
        let name_test="csv_test2".to_string();
        let mut attribut_test:Vec<String> = Vec::new();
        attribut_test.push("test1".to_string());
        attribut_test.push("test2".to_string());
        attribut_test.push("test3".to_string());
        let _i = relation_creater(&name_test, &attribut_test).expect("errorTest");


    }

}