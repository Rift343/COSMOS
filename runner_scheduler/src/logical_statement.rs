use json::JsonValue;

use crate::{operator::CSVFile, where_statement::where_statement};

#[doc = "Match between AND et OR."]
pub fn logical_execution(a1:& mut CSVFile,logical_value:&JsonValue) -> CSVFile
{
    match logical_value["operator"].to_string().as_str() {
        "AND" => return and_execution(a1, &logical_value["right"], &logical_value["left"]),
        "OR" => return or_execution(a1, &logical_value["right"], &logical_value["left"]),
        _ => todo!(),
    };
}

#[doc = "Function to execute AND in WHERE statement. Execute right member then left member. The left member use in input the result if the right member."]
pub fn and_execution (a1 : & mut CSVFile, right:&JsonValue,left:&JsonValue) -> CSVFile
{
    
    let mut a2 = where_statement(a1, left);
    let a3 = where_statement( & mut a2, right);
    return a3;
}

#[doc = "Function to execute OR in WHERE statement. Execute right member then left member. To finish we do the union of the result."]
pub fn or_execution (a1 : & mut CSVFile, right:&JsonValue,left:&JsonValue) -> CSVFile
{
    let mut a2 =  where_statement(a1, right);
    let mut a3 = where_statement(a1, left);
    a2.union(& mut a3);
    return a2
}