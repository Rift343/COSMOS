use json::JsonValue;

use crate::{logical_statement::logical_execution, operator::CSVFile, operator::WhereElement};

pub fn where_statement(a1:& mut CSVFile,where_value:&JsonValue) -> CSVFile
{
    println!("{}",where_value["type"].to_string());
    if where_value["type"].to_string() == "logical".to_string()
    {
        println!("ok-->logical_execution(a1,where_value)");
        return logical_execution(a1, where_value);
    }
    else if where_value["type"].to_string() == "condition".to_string() 
    {
        println!("ok");
        let left = convert_json_to_where_element(&where_value["left"]);
        let right = convert_json_to_where_element(&where_value["right"]);
        println!("{}",a1.to_string());
        println!("-----------------------------------------------------------------");
        a1.predicat_interpretation(where_value["condition"].to_string(), where_value["datatype"].to_string(), left, right);
        println!("{}",a1.to_string());
        return a1.clone();
    }
    else 
    {
        todo!()
    }
}

pub fn convert_json_to_where_element (value:&JsonValue) -> WhereElement
{
    if value["type"].to_string() == "CONSTANT".to_string().to_lowercase()
    {
        let return_value = WhereElement{where_value:value["value"].to_string(),boolean_value:false};
        return_value
    }
    else if value["type"].to_string() == "ATTRIBUTE".to_string().to_lowercase() {
        let mut attribute_str = "".to_string();
        attribute_str.push_str(&value["use_name_table"].to_string());
        attribute_str.push('.');
        attribute_str.push_str(&value["attribute_name"].to_string());
        let return_value = WhereElement{where_value:attribute_str,boolean_value:true};
        return_value
    }
    else if value["type"].to_string() == "SUBQUERY".to_string().to_lowercase() 
    {
        println!("HERE");
        todo!()    
    }
    else {
        println!("HERE");
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use crate::operator::open_relation;

    use super::*;

    #[test]
    fn test1()
    {
        let mut json_file:std::fs::File = File::open("semantique2.json").expect("Error ==> Can't read the JSON file");
        let mut buffer = Vec::new();
        json_file.read_to_end(&mut buffer).expect("Read to end error");
        let mut str_json  : String = String::new();
        for i in buffer{
            str_json.push(i as char);
        }
        let parse_json=json::parse(&str_json.to_string()).unwrap();
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        where_statement(&mut table1, &parse_json["conditions"]);
    }

}