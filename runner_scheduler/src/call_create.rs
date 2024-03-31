use std::error::Error;

use json::JsonValue;

use crate::relation_creater::relation_creater;
use std::fs::File;
use serde_json;
use std::io::Read;
use serde_json::Value;
pub fn call_create(sub_requet:File)->Result<i8,Box<dyn Error>>
{
    //let mut req = String::new();
    //sub_requet.read_to_string(&mut req).expect("Error: Unable to read syntaxic parsing file");

    // Convert to a serde_json Value type
    //let parsing_value : Value = serde_json::from_str(&*req).expect("Error: Unable to turn JSON file into Value type");
    let mut r = 1;
    //let parse_json = parsing_value;
    //for (key,val) in parse_json{
    //    let table_name = key;
    //    let mut lst_attribut:Vec<String> = Vec::new();
    //    for i in 0..val.len()
    //    {
    //        lst_attribut.push(parse_json["columns"][i]["column_name"].to_string());
    //    }
    //    let res = match relation_creater(&table_name,&lst_attribut) {
    //        Ok(_) =>1 ,
    //        Err(e) => return Err(Box::from(e)),
    //    };
    //    r = res;
    //}
    
    Ok(r)
}