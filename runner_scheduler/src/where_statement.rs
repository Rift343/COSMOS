use std::{collections::{hash_map, HashMap}, thread::JoinHandle};

use json::JsonValue;

use crate::{logical_statement::logical_execution, operator::CSVFile, operator::WhereElement};

pub fn where_statement(a1:& mut CSVFile,where_value:&JsonValue,thread_hashmap : &mut HashMap<String,JoinHandle<CSVFile>>) -> CSVFile
{
    //println!("{}",where_value["etype"].to_string());
    if where_value["etype"].to_string() == "logical".to_string()
    {
        //println!("ok-->logical_execution(a1,where_value)");
        return logical_execution(a1, where_value,thread_hashmap);
    }
    else if where_value["etype"].to_string() == "condition".to_string() 
    {
        //println!("ok");
        let left = convert_json_to_where_element(&where_value["left"],thread_hashmap);
        let right = convert_json_to_where_element(&where_value["right"],thread_hashmap);
        //println!("{}",a1.to_string());
        //println!("-----------------------------------------------------------------");
        a1.predicat_interpretation(where_value["condition"].to_string(), where_value["datatype"].to_string(), left, right);
        println!("condition");
        //println!("{}",a1.to_string());
        return a1.clone();
    }
    else if where_value["etype"].to_string() == "checker".to_string() 
    {
        //println!("ok");
        match where_value["check_type"].to_string().as_str() {
            "IN" => {
                let left: WhereElement = convert_json_to_where_element(&where_value["left"],thread_hashmap);
                let right: HashMap<String, i8> = convert_json_to_hashmap(a1, &where_value["right"], thread_hashmap);
                
                todo!();
            },
            "EXIST" => todo!(),
            _ => todo!(),
            
        }
    }
    else  
    {
        todo!()//case if something is wrong in the semantic file. If you go here verify were you call this function
               //or check of the semantic file is good
    }
}

pub fn convert_json_to_where_element (value:&JsonValue,thread_hashmap : &mut HashMap<String,JoinHandle<CSVFile>>) -> WhereElement
{
    if value["etype"].to_string() == "CONSTANT".to_string().to_lowercase()
    {
        let return_value = WhereElement{where_value:value["value"].to_string(),boolean_value:false};
        return_value
    }
    else if value["etype"].to_string() == "ATTRIBUTE".to_string().to_lowercase() {
        let mut attribute_str = "".to_string();
        attribute_str.push_str(&value["use_name_table"].to_string());
        attribute_str.push('.');
        attribute_str.push_str(&value["attribute_name"].to_string());
        let return_value = WhereElement{where_value:attribute_str,boolean_value:true};
        return_value
    }
    else if value["etype"].to_string() == "SUBQUERY".to_string().to_lowercase() 
    {
        println!("{:?}",thread_hashmap.keys());
        println!("{}",&value["query".to_string()].to_string());
        let thread_handle = thread_hashmap.remove(&value["query".to_string()].to_string());//remove the JoinHandle of the hashmap
        let res = match thread_handle {
            Some(x)=> x,
            None => todo!(),
        };

        let res = match res.join() {//join the thread
            Ok(e) => e,
            Err(_) => panic!("Thread error"),
        };
        
        
        let return_value = WhereElement{where_value:res.descriptor[1][0].to_string(),boolean_value:false};//We need only the first element of the result
        return_value 
    }
    else {
        println!("HERE2");
        todo!()
    }
}

pub fn convert_json_to_vec_string(a1:& mut CSVFile,value:&JsonValue,thread_hashmap :&mut HashMap<String,JoinHandle<CSVFile>>)//why I write this function, good question
{
    match value["etype"].to_string().as_str() {
        "datalist"=>todo!(),
        "subquery"=>todo!(),
        _=>todo!(),
        
    }
}

pub fn convert_json_to_hashmap(a1:& mut CSVFile,value:&JsonValue,thread_hashmap :&mut HashMap<String,JoinHandle<CSVFile>>)->HashMap<String, i8>
{
    match value["etype"].to_string().as_str() {
        "datalist"=>{
            let mut res_hashmap:HashMap<String, i8> = HashMap::new();

            let mut index_hash:HashMap<String, usize> = HashMap::new();

            for i in 0..a1.descriptor[0].len()
            {
                index_hash.insert(a1.descriptor[0][i].to_string().clone(), i);//get hashmap of the index ==> easy to get the index of a columns after
            }

            for i in 1..a1.descriptor.len()
            {
                let mut v1 = Vec::new();
                for y in 0..value["value"].len()
                {
                    let json_value = value["value"][y].clone();
                    if json_value["etype"].to_string() == "attribute".to_string()
                    {
                        let get_val = match index_hash.get(&json_value["value"].to_string())  {
                            Some(x) => x,
                            None => todo!(),
                        };
                        v1.push(a1.descriptor[i][*get_val].clone())
                    }
                    else 
                    {
                        v1.push(json_value["value"].to_string());    
                    }
                }
                res_hashmap.insert(v1[0].clone(), 1);//we have a hashmap of all the value in the columns where_element.where_value
                
            }
            res_hashmap 
        },
        "subquery"=>{
            let thread_handle = thread_hashmap.remove(&value["query".to_string()].to_string());//remove the JoinHandle of the hashmap
            let res = match thread_handle {
                Some(x)=> x,
                None => todo!(),
                };

            let res = match res.join() {//join the thread
                Ok(e) => e,
                Err(_) => panic!("Thread error"),
                };
            let mut returned_hash:HashMap<String, i8> = HashMap::new();
            for i in 1..res.descriptor.len()
            {
                returned_hash.insert(res.descriptor[i][0].clone(), 1);//in this case the subquery have form (select count(...) from...) or (select id from ...) so we just keep the value of the first column

            }
            return returned_hash;
            },
        _=>todo!(),
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
        //parse_json["conditions"] = "higyuv".into();
        //println!("{}",parse_json["conditions"].to_string());
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let mut thread_hashmap = HashMap::new();
        let a1 = where_statement(&mut table1, &parse_json["conditions"],&mut thread_hashmap);
        println!("FINAL RESULT :\n{}",a1.to_string());
    }

    

}