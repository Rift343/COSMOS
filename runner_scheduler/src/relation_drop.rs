use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use crate::operator::open_relation;
use crate::operator::CSVFile;

#[allow(unused_variables)]
#[allow(unused_must_use)]
#[allow(unused)]

pub(crate) fn relation_drop(table_name:&String)->Result<i8,Box<dyn Error>>
{ 
    fs::remove_file("table_name".to_string())?;
    let mut a1 = match open_relation("ALL_TABLE".to_string(), &"ALL_TABLE".to_string())
    {
        Ok(e) => e,
        Err(e) => return Err(Box::from(e)) ,
    };
    





    Ok(0)
}

mod tests {
    #[allow(unused)]
    use super::relation_drop;


    #[test]
    fn test_relation_drop()
    {
    


    }

}