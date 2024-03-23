use std::{collections::HashMap, error::Error, thread};

use json::JsonValue;

use crate::operator::{self, CSVFile};


#[doc = "This function is use for the query and subquery. Need in input a reference of a JsonValue of the Semantic parser. Return a CSVFile or error. Can crash if the date of the CSV are corructped
Use thread for the agragation fonction (for the select statement) and for the intermediary request/sub-query"]
pub fn intermediary_request(sub_requet:&JsonValue)->Result<CSVFile,Box<dyn Error>>
{
    println!("Begin runner_scheduler : {}",sub_requet.dump());
    let parse_json = sub_requet;
    let mut keylst:Vec<String>=Vec::new();//We need to keep the list of the key in memory
    let mut final_proj:Vec<String>= Vec::new();//list of all the 
    let mut dictionnary: HashMap<String, crate::operator::CSVFile> = HashMap::new();
    let mut as_hashmap: HashMap<String,String> = HashMap::new();
    for (key,value) in parse_json["tables"].entries(){
        println!("key : {:?}", key);
        let mut intermediary_vector:Vec<String>=Vec::new();
        for y in 0..value["columns"].len()
        {
            let mut my_str:String = value["use_name_table"].to_string();//We rename immediatly the columns for more simplicity in the next operation (for the future we need to search a methode to apply the rename of the user ( key word 'as'))
            my_str.push('.');
            my_str.push_str(&value["columns"][y]["attribute_name"].to_string());
            intermediary_vector.push(my_str.clone());
            if value["columns"][y]["use_name_attribute"] !=value["columns"][y]["attribute_name"]
            {
                let mut as_str = "".to_string();//parse_json["tables"][i]["table"]["use_name_table"].to_string();
                //as_str.push('.');
                as_str.push_str(&value["columns"][y]["use_name_attribute"].to_string());
                as_hashmap.insert(my_str.clone(), as_str.clone());
            }
            final_proj.push(my_str);
        }
        keylst.push(value["use_name_table"].to_string());
        //println!("{:?}",intermediary_vector);
        //println!("{}",parse_json["tables"][i]["table"]["table_name"].to_string());
        //println!("{:?}",as_hashmap);
        let mut open_file:CSVFile = operator::open_relation(key.to_string(), &value["use_name_table"].to_string())?;//.expect("error");//We open each relation
        if intermediary_vector.len()!=0
        {
            open_file.projection(intermediary_vector);//We made a first projection to keep only the date we use for the request
        }
        dictionnary.insert(value["use_name_table"].to_string(),open_file);//We insert the projected file in a dictionnary
    }




    /* 
    
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

    }*/
    // Now we need to do the cartesian product on all the relation use in the request. For this we made the cartesian product on the first open file.
    for i in 1..keylst.len(){
        //gestion erreur avec le clone
        //let mut test =dictionnary.get_mut(&key[0]).ok_or_else(); //.expect("Get error ").clone();

        let mut test : CSVFile;
        match dictionnary.get_mut(&keylst[0]){
            Some(res) => test = res.clone(),
            _ => return Err(Box::from("Error : Runner : Key 0 doesn't exist"))
        };

        //let test2 =dictionnary.get(&key[i]).expect("Get error");
        let test2 : CSVFile = match dictionnary.get_mut(&keylst[i]){
            Some(res) => res.clone(),
            _ => return Err(Box::from("Error : Runner : Key i doesn't exist"))
        };

        test.cartesian_product(&test2);
        
        dictionnary.insert(keylst[0].to_string(), test);
    }
    //println!("crash");
    //After the cartesian product, we need to close de file. For this we create a file of first open file (so the first entry create in the dictionnary)
    let mut a1 = dictionnary[&keylst[0]].clone();
    
    //println!("{:?}",as_hashmap);
    

    let mut tab_agregate_fun:Vec<JsonValue> = Vec::new();//Vector for the agregation function
    for i in 0..parse_json["aggregates"].len()
    {
        tab_agregate_fun.push(parse_json["aggregates"][i].clone());
        let mut str1 = parse_json["aggregates"][i]["aggregate_type"].to_string();
        str1.push('(');
        let mut str2 = str1.clone();
        str2.push_str(&parse_json["aggregates"][i]["use_name_table"].to_string());
        str2.push('.');
        str2.push_str(&parse_json["aggregates"][i]["attribute_name"].to_string());
        str2.push(')');
        final_proj.push(str2.clone());
        as_hashmap.insert(str2,parse_json["aggregates"][i]["use_name_attribute"].to_string() );
        //println!("{}",parse_json["aggregates"][i].dump());
    }
    let mut tab_thread = Vec::new();
    for i in 0..tab_agregate_fun.len()
    {
        let data_in1 = tab_agregate_fun[i].clone();
        let data_in2 = a1.clone();
        
        tab_thread.push(thread::spawn(move || //begin thread
            {
                //Need to select with if statement between COUNT,SUM,MIN,MAX or AVG
                //println!("thread::{:?}",thread::current().id());
                if data_in1["aggregate_type"].to_string() == "COUNT".to_string()
                {
                    //println!("thread::{:?}",thread::current().id());
                    if data_in1["attribute_name"].to_string() =="*".to_string() // Need to select betwenn * or a attribute.
                    {
                        //println!("{}",&data_in1["attribute_name"].to_string());
                        let data_out1 = data_in2.count(&data_in1["attribute_name"].to_string());//start the count function
                        return data_out1;//return the count result

                    }
                    else
                    {
                        let mut attribute =data_in1["use_name_table"].to_string().clone();//The attribute need to be relation.attribute form
                        attribute.push('.');
                        attribute.push_str(&data_in1["attribute_name"].to_string());
                        //println!("{}",attribute);
                        let data_out1 = data_in2.count(&attribute);
                        return data_out1;
                    }
                    
                }
                else if data_in1["aggregate_type"].to_string() == "SUM".to_string()//statement for the SUM
                {
                    //println!("thread::{:?}",thread::current().id());
                    let mut attribute =data_in1["use_name_table"].to_string().clone();//The attribute need to be relation.attribute form
                    attribute.push('.');
                    attribute.push_str(&data_in1["attribute_name"].to_string());
                    //println!("{}",data_in1["attribute_type"].to_string());
                    let data_out = data_in2.sum(&attribute, &data_in1["attribute_type"].to_string());
                    return data_out;//return of the sum
                }
                //println!("{}", data_in1.dump());  // we can use the data here!
                let data_out: Vec<String> = Vec::new();
                
                data_out // <-- simply return the data from the closure
            }));//end thread
    }
    for i in tab_thread
    {
        let data_out = match i.join() {
            Ok(a) => a,
            _ => return Err(Box::from("Error : A thread don't finish correctly")) ,   
        };
        //println!("crash{}",a1.to_string());
        //println!("{:?}",data_out);
        a1.add_column_for_agregate(&data_out);// <- add all result of the argregation function

    }
    //println!("{}",a1.to_string());
    //println!("{:?}",final_proj);
    //println!("{:?}",as_hashmap);
    a1.projection(final_proj);
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
        let _a = match intermediary_request(&parse_json)  {
            Ok(a) => print!("{}",a.to_string()),
            Err(e) => println!("{}",e),
        };

    }

}