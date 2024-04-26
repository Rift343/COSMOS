use std::{collections::HashMap, thread::JoinHandle};

use json::JsonValue;

use crate::{operator::CSVFile, where_statement::where_statement};

#[doc = "Match between AND et OR."]
pub fn logical_execution(a1:& mut CSVFile,logical_value:&JsonValue,thread_hashmap : &mut HashMap<String,JoinHandle<CSVFile>>) -> CSVFile
{
    match logical_value["operator"].to_string().as_str() {
        "AND" => return and_execution(a1, &logical_value["right"], &logical_value["left"],thread_hashmap),
        "OR" => return or_execution(a1, &logical_value["right"], &logical_value["left"],thread_hashmap),
        "NOT" => return not_execution(a1, &logical_value["right"], &logical_value["left"],thread_hashmap),
        _ => {println!("heure");todo!()},
    };
}

#[doc = "Function to execute AND in WHERE statement. Execute right member then left member. The left member use in input the result if the right member."]
pub fn and_execution (a1 : & mut CSVFile, right:&JsonValue,left:&JsonValue,thread_hashmap : &mut HashMap<String,JoinHandle<CSVFile>>) -> CSVFile
{
    
    let mut a2 = where_statement(a1, left,thread_hashmap);
    let a3 = where_statement( & mut a2, right,thread_hashmap);
    println!("and");
    return a3;
}

#[doc = "Function to execute OR in WHERE statement. Execute right member then left member. To finish we do the union of the result."]
pub fn or_execution (a1 : & mut CSVFile, right:&JsonValue,left:&JsonValue,thread_hashmap : & mut HashMap<String,JoinHandle<CSVFile>>) -> CSVFile
{
    let mut a2 =  where_statement(& mut a1.clone(), right,thread_hashmap);
    let mut a3 = where_statement(& mut a1.clone(), left,thread_hashmap);
    a2.union(& mut a3);
    println!("or");
    return a2
}

#[doc = "Function to execute not in WHERE statement. Right member should by null. We exclude the element selected by where_statement on the left member."]
pub fn not_execution (a1 : & mut CSVFile, _right:&JsonValue,left:&JsonValue,thread_hashmap : &mut HashMap<String,JoinHandle<CSVFile>>) -> CSVFile
{
    
    let a2 =  where_statement(& mut a1.clone(), left,thread_hashmap);
    let mut to_exclude =a2.descriptor;
    to_exclude.remove(0); 
    a1.exclude(to_exclude);
    println!("not");
    return a1.to_owned()
}