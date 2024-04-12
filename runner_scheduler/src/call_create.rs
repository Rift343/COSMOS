use std::{error::Error, io::Seek};

use crate::relation_creater::relation_creater;
use std::fs::File;
use std::io::Read;

///This function creates the csv file for a new table in the database
/// input : File
/// output : Result<i8,Box<dyn Error>>
/// the function takes as an input the syntaxic file, it extracts the table name and columns names, creates the csv file and return either 1 or an error
pub fn call_create(mut sub_request: &File) -> Result<i8, Box<dyn Error>> {
    sub_request.rewind()?; //set file descriptor to 0
    let mut buffer = Vec::new();
    sub_request.read_to_end(&mut buffer)?; //put the file in a new string
    let mut str_json: String = String::new();
    for i in buffer {
        str_json.push(i as char);
    }
    let parse_json = json::parse(&str_json.to_string()).unwrap(); //Convert Vec<u8> to a JsonValue
    println!("{}", parse_json.to_string());
    // Convert to a serde_json Value type
    //let parsing_value : Value = serde_json::from_str(&*req).expect("Error: Unable to turn JSON file into Value type");
    //let mut r = 1;
    //let parse_json = parsing_value;
    let table_name = parse_json["table_name"][0]["table_name"]
        .to_string()
        .to_uppercase();
    let mut lst_attribut: Vec<String> = Vec::new();
    for i in 0..parse_json["columns"].len() {
        lst_attribut.push(parse_json["columns"][i]["name"].to_string().to_uppercase());
    }
    let r = match relation_creater(&table_name, &lst_attribut) {
        Ok(_) => 1,
        Err(e) => return Err(Box::from(e)),
    };
    Ok(r)
}

mod test {

    #[allow(unused)]
    use super::call_create;

    #[test]

    fn test_call_create_1() {}
}
