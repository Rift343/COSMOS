use std::{collections::HashMap, error::Error};

use json::JsonValue;

use crate::operator::{self, CSVFile};

pub fn intermediary_request(sub_requet:&JsonValue)->Result<CSVFile,Box<dyn Error>>
{
    println!("{}",sub_requet.dump());
    let parse_json = sub_requet;
    let mut key:Vec<String>=Vec::new();//We need to keep the list of the key in memory
    let mut final_proj:Vec<String>= Vec::new();//list of all the 
    let mut dictionnary: HashMap<String, crate::operator::CSVFile> = HashMap::new();
    let mut as_hashmap: HashMap<String,String> = HashMap::new();
    for i in 0..parse_json["tables"].len(){
        let mut intermediary_vector:Vec<String>=Vec::new();
        for y in 0..parse_json["tables"][i]["columns"].len(){
            let mut my_str:String = parse_json["tables"][i]["table"]["use_name_table"].to_string();//We rename immediatly the columns for more simplicity in the next operation (for the future we need to search a methode to apply the rename of the user ( key word 'as'))
            my_str.push('.');
            my_str.push_str(&parse_json["tables"][i]["columns"][y]["attribute_name"].to_string());
            //println!("{}",my_str);
            intermediary_vector.push(my_str.clone());
            if parse_json["tables"][i]["columns"][y]["use_name_attribute"] !=parse_json["tables"][i]["columns"][y]["attribute_name"]
            {
                let mut as_str = "".to_string();//parse_json["tables"][i]["table"]["use_name_table"].to_string();
                //as_str.push('.');
                as_str.push_str(&parse_json["tables"][i]["columns"][y]["use_name_attribute"].to_string());
                as_hashmap.insert(my_str.clone(), as_str.clone());
            }
            final_proj.push(my_str);
            

        }
        key.push(parse_json["tables"][i]["table"]["use_name_table"].to_string());
        //println!("{:?}",intermediary_vector);
        //println!("{}",parse_json["tables"][i]["table"]["table_name"].to_string());
        //println!("{:?}",as_hashmap);
        let mut open_file:CSVFile = operator::open_relation(parse_json["tables"][i]["table"]["table_name"].to_string(), &parse_json["tables"][i]["table"]["use_name_table"].to_string())?;//.expect("error");//We open each relation
        open_file.projection(intermediary_vector);//We made a first projection to keep only the date we use for the request
        dictionnary.insert(parse_json["tables"][i]["table"]["use_name_table"].to_string(),open_file);//We insert the projected file in a dictionnary

    }
    // Now we need to do the cartesian product on all the relation use in the request. For this we made the cartesian product on the first open file.
    for i in 1..key.len(){
        //gestion erreur avec le clone
        //let mut test =dictionnary.get_mut(&key[0]).ok_or_else(); //.expect("Get error ").clone();

        let mut test : CSVFile;
        match dictionnary.get_mut(&key[0]){
            Some(res) => test = res.clone(),
            _ => return Err(Box::from("Error : Runner : Key 0 doesn't exist"))
        };

        //let test2 =dictionnary.get(&key[i]).expect("Get error");
        let test2 : CSVFile = match dictionnary.get_mut(&key[i]){
            Some(res) => res.clone(),
            _ => return Err(Box::from("Error : Runner : Key i doesn't exist"))
        };

        test.cartesian_product(&test2);

        dictionnary.insert(key[0].to_string(), test);
    }
    //After the cartesian product, we need to close de file. For this we create a file of first open file (so the first entry create in the dictionnary)
    let mut a1 = dictionnary[&key[0]].clone();
    a1.projection(final_proj);
    println!("{:?}",as_hashmap);
    a1.replace_as(&as_hashmap);
    
    Ok(a1)
}

mod tests {
    

    #[allow(unused)]
    use super::intermediary_request;


    #[test]
    fn test_intermediary_request()
    {
        let mut json_file:std::fs::File = std::fs::File::open("semantique.json").expect("Error ==> Can't read the JSON file");
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut json_file, &mut buffer).expect("error"); //.expect("Read to end error");
        let mut str_json  : String = String::new();
        for i in buffer{
            str_json.push(i as char);
        }
        let parse_json=json::parse(&str_json.to_string()).unwrap();
        let a = match intermediary_request(&parse_json)  {
            Ok(a) => print!("{}",a.to_string()),
            Err(e) => println!("{}",e),
        };

    }

}