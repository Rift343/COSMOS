use std::error::Error;

use json::JsonValue;

use crate::relation_creater::relation_creater;



pub fn call_create(sub_requet:&JsonValue)->Result<i8,Box<dyn Error>>
{
    let parse_json = sub_requet;
    let table_name = parse_json["table_name"][0]["table_name"].to_string();
    let mut lst_attribut:Vec<String> = Vec::new();
    for i in 0..parse_json["columns"].len()
    {
        lst_attribut.push(parse_json["columns"][i]["nom"].to_string());
    }
    let r = match relation_creater(&table_name,&lst_attribut) {
        Ok(_) =>1 ,
        Err(e) => return Err(Box::from(e)),
    };
    Ok(r)
}