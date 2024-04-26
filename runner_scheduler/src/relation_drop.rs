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
    //println!("ici");
    let mut path_file = "data/CSV/".to_string();
    path_file.push_str(table_name);
    path_file.push_str(".csv");
    //Begin modify ALL_TABLES table
    fs::remove_file(path_file.to_string())?;
    println!("ok");
    let mut a1 = match open_relation("ALL_TABLES".to_string(), &"ALL_TABLES".to_string())
    {
        Ok(e) => e,
        Err(e) => return Err(Box::from(e)) ,
    };
    let mut val1 = WhereElement { where_value: "ALL_TABLES.TABLE_NAME".to_string(),boolean_value: true };
    let mut val2 = WhereElement { where_value: table_name.to_string(),boolean_value: false };
    a1.predicat_interpretation("<>".to_string(), "VARCHAR".to_string(), val1, val2);
    fs::remove_file("./data/csv/ALL_TABLES.csv".to_string())?;
    let mut file:File = match OpenOptions::new().read(true).write(true).truncate(true).create(true).open("./data/csv/ALL_TABLES.csv") {
        Ok(e) => e,
        Err(e) =>  return Err(Box::new(e)),
    };
    //println!("{}",a1.to_string());
    a1.descriptor[0][0]="TABLE_NAME".to_string();
    //println!("{:?}",a1.descriptor);
    file.write_all(a1.to_string().as_bytes());
    //drop(file);


    //Modify ALL_COLUMNS table
    println!("ok");
    let mut a2 = match open_relation("ALL_COLUMNS".to_string(), &"ALL_COLUMNS".to_string())
    {
        Ok(e) => e,
        Err(e) => return Err(Box::from(e)) ,
    };
    println!("ok");
    fs::remove_file("./data/csv/ALL_COLUMNS.csv".to_string())?;
    val1 = WhereElement { where_value: "ALL_COLUMNS.TABLE_NAME".to_string(),boolean_value: true };
    val2 = WhereElement { where_value: table_name.to_string(),boolean_value: false };
    a2.predicat_interpretation("<>".to_string(), "VARCHAR".to_string(), val1, val2);
    a2.descriptor[0][0]="TABLE_NAME".to_string();
    a2.descriptor[0][1]="COLUMN_NAME".to_string();
    println!("ok");
    let mut file2:File = match OpenOptions::new().read(true).write(true).truncate(true).create(true).open("./data/csv/ALL_COLUMNS.csv") {
        Ok(e) => e,
        Err(e) =>  return Err(Box::new(e)),
    };
    file2.write_all(a2.to_string().as_bytes());

    //drop(file2);
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