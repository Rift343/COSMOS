use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use crate::operator::open_relation;
use crate::operator::WhereElement;

#[allow(unused_variables)]
#[allow(unused_must_use)]
#[allow(unused)]

pub(crate) fn relation_drop(table_name:&String)->Result<i8,Box<dyn Error>>
{ 
    println!("ici");
    let mut path_file = "data/CSV/".to_string();
    path_file.push_str(table_name);
    path_file.push_str(".csv");
    
    fs::remove_file(path_file.to_string())?;
    let mut a1 = match open_relation("ALL_TABLE".to_string(), &"ALL_TABLE".to_string())
    {
        Ok(e) => e,
        Err(e) => return Err(Box::from(e)) ,
    };
    let val1 = WhereElement { where_value: "ALL_TABLE.TABLE_NAME".to_string(),boolean_value: true };
    let val2 = WhereElement { where_value: table_name.to_string(),boolean_value: false };
    a1.predicat_interpretation("<>".to_string(), "VARCHAR".to_string(), val1, val2);
    fs::remove_file("./data/csv/ALL_TABLE.csv".to_string())?;
    let mut file:File = match OpenOptions::new().read(true).write(true).truncate(true).create(true).open("./data/csv/ALL_TABLE.csv") {
        Ok(e) => e,
        Err(e) =>  return Err(Box::new(e)),
    };
    //println!("{}",a1.to_string());
    a1.descriptor[0][0]="TABLE_NAME".to_string();
    //println!("{:?}",a1.descriptor);
    file.write_all(a1.to_string().as_bytes());
    Ok(0)
}

mod tests {
    #[allow(unused)]
    use super::relation_drop;


    #[test]
    fn test_relation_drop()
    {
        let _ = relation_drop(&"csv_test2".to_string()).expect("msg");


    }

}