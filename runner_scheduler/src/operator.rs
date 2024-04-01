use std::collections::HashMap;
use std::fs::OpenOptions;
use std::{fs::File, io::Read};
use std::error::Error;
use std::io::{BufReader, Seek, Write};

#[allow(unused_variables)]
#[allow(unused_must_use)]

#[allow(unused)]

#[doc = "That struct is use for the where. The boolean_value indicate the if where_value contain a attribute of a relation or a constant"]
pub struct WhereElement {
    pub where_value:String,
    pub boolean_value:bool,//if true then it's an attribute, false if it's a const
}

#[allow(unused)]
#[doc = r" Structur with a name and a 'descriptor' value. The descriptor is a Vec of Vec of String and represent the CSV file ligne by ligne.
Use the function open_relation(pathcsv:String,name1:String) to create a CSVFile object."]

#[derive(Clone)]

pub struct CSVFile{
    pub name:String,
    pub descriptor:Vec<Vec<String>>
}

#[allow(unused)]

impl CSVFile {

#[doc = "Method to check if a list of attribute are not in list of value. Change the self value"]
pub fn exclude(&mut self,to_chek: Vec<Vec<String>>)
{
    let mut res_vec:Vec<Vec<String>> =Vec::new();
    res_vec.push(self.descriptor[0].clone());
    for ligne in 1..self.descriptor.len()
    {
        let mut bool_ligne = true;
        let lst_compare = self.descriptor[ligne].clone();
        for element in 0..to_chek.len()
        {
            if to_chek[element] == lst_compare
            {
                bool_ligne = false;
                break;
            }
        }
        if bool_ligne == true
        {
            res_vec.push(lst_compare);
        }
    }
    self.descriptor = res_vec;
}





#[doc = "Methode to shedule a boolean predicat. That methode choose the good function to use depending of the value of tje WhereElement"]
pub fn predicat_interpretation (&mut self, operation : String, type_expression: String, element_1 : WhereElement,element_2:WhereElement)
{
    match element_1.boolean_value==true {
        true => match element_2.boolean_value == true {
            true => self.predicat_interpretation_with_no_const(operation, type_expression, element_1, element_2),//case no constant
            false => self.predicat_interpretation_with_one_const_2(operation, type_expression, element_1, element_2.where_value),//case element_2 is a constant
        },
        false => match element_2.boolean_value == true {
            true => self.predicat_interpretation_with_one_const(operation, type_expression, element_1.where_value, element_2),//case element_1 is a constant
            false => self.predicat_interpretation_with_two_const(operation, type_expression, element_1.where_value, element_2.where_value),//case both element are constant
        },
    }

}




#[doc = "Methode use for the interpretation of a boolean statement. Need one constant value, the operator (=,<>...) and the type (INT,FLOAT,CHAR)
In a first place we need to match the operation then the type.Finaly whe check the condition line by line. If possible the cast operation was done before the for statement"]
pub fn predicat_interpretation_with_one_const (&mut self, operation : String, type_expression: String, element_1 : String,element_2:WhereElement) 
{
    //println!("1");
    //println!("{}{}{}",element_1,operation,element_2.where_value);
    let mut index;
    let mut i = 0 ;
    let mut final_vec:Vec<Vec<String>>= Vec::new();
    final_vec.push(self.descriptor[0].to_vec());
    while i<self.descriptor[0].len() && element_2.where_value.to_string() != self.descriptor[0][i].to_string() {
        i = i + 1;       
    }
    index = i;
    //println!("here : {}",index);
    if operation == "=".to_string()
    {
        if type_expression == "FLOAT".to_string()
        {
            let value:f64 = element_1.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value == value2 ) {
                    //self.descriptor.remove(i);
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            

        }
        else if type_expression == "INT".to_string() 
        {
            let value:i128 = element_1.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            let value = element_1;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            
        }
        else if type_expression == "CHAR".to_string() 
        {
            let value = element_1;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }

    }
    else if operation == "<=".to_string() 
    {//begin section
        if type_expression == "FLOAT".to_string()
        {
            let value:f64 = element_1.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            

        }
        else if type_expression == "INT".to_string() 
        {
            let value:i128 = element_1.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            let value = element_1;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "CHAR".to_string() 
        {
            let value = element_1;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
    }//end section
    else if operation == ">=".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                ;
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
    else if operation == "<>".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
        else if operation == "<".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
        else if operation == ">".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_1.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_1;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
    /* 
    for i in 1..self.descriptor.len()
    {

    }*/
    

}

#[doc = "Methode use for the interpretation of a boolean statement. Need one constant value, the operator (=,<>...) and the type (INT,FLOAT,CHAR)"]
pub fn predicat_interpretation_with_one_const_2 (&mut self, operation : String, type_expression: String, element_1 : WhereElement,element_2:String) 
{
    //println!("2");
    //println!("{}{}{}",element_1.where_value,operation,element_2);

    let mut index;
    let mut i = 0 ;
    let mut final_vec:Vec<Vec<String>>= Vec::new();
    final_vec.push(self.descriptor[0].to_vec());
    while i<self.descriptor[0].len() && element_1.where_value.to_string() != self.descriptor[0][i].to_string() {
        i = i + 1;       
    }
    index = i;
    //println!("here : {}",index);
    if operation == "=".to_string()
    {
        if type_expression == "FLOAT".to_string()
        {
            let value:f64 = element_2.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value == value2 ) {
                    //self.descriptor.remove(i);
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            

        }
        else if type_expression == "INT".to_string() 
        {
            let value:i128 = element_2.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            let value = element_2;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            
        }
        else if type_expression == "CHAR".to_string() 
        {
            let value = element_2;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }

    }
    else if operation == "<=".to_string() 
    {//begin section
        if type_expression == "FLOAT".to_string()
        {
            let value:f64 = element_2.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value2 <= value ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            

        }
        else if type_expression == "INT".to_string() 
        {
            let value:i128 = element_2.parse().unwrap();
            for i in 1..self.descriptor.len()
            {
                let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                if (value2 <= value ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            let value = element_2;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value2 <= value ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "CHAR".to_string() 
        {
            let value = element_2;
            for i in 1..self.descriptor.len()
            {
                let value2 = self.descriptor[i][index].clone(); 
                if (value2 <= value ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
    }//end section
    else if operation == ">=".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value2 >= value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value2 >= value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                ;
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value2 >= value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value2 >= value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
    else if operation == "<>".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    //println!("ok {} {}",value,value2);
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            //println!("{}",self.to_string());
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            println!("{:?}",self.descriptor);
            
            }
        }//end section
        else if operation == "<".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value2 < value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value2 < value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value2 < value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value2 < value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
        else if operation == ">".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                let value:f64 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value2 > value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                let value:i128 = element_2.parse().unwrap();
                for i in 1..self.descriptor.len()
                {
                    let value2:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    if (value2 > value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value2 > value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                let value = element_2;
                for i in 1..self.descriptor.len()
                {
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value2 > value ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
    /* 
    for i in 1..self.descriptor.len()
    {

    }*/
    

}

#[doc = "Methode use for the interpretation of a boolean statement. Need two constant value, the operator (=,<>...) and the type (INT,FLOAT,CHAR)"]
pub fn predicat_interpretation_with_two_const (&mut self, operation : String, type_expression: String, element_1 : String,element_2:String)
{
    let mut final_vec:Vec<Vec<String>>= Vec::new();
    final_vec.push(self.descriptor[0].to_vec());
    
    if operation == "=".to_string()
    {//begin section
        if type_expression == "FLOAT".to_string()
        {
           let val1:f64 = element_1.parse().unwrap();
           let val2:f64 = element_2.parse().unwrap();
           if !(val1 == val2)
           {
            self.descriptor = final_vec;
           }
        }
        else if type_expression == "INT".to_string() 
        {
            let val1:i128 = element_1.parse().unwrap();
            let val2:i128 = element_2.parse().unwrap();
            if !(val1 == val2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            if !(element_1 == element_2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "CHAR".to_string() 
        {
            if !(element_1 == element_2)
            {
                self.descriptor = final_vec;
            }
        }

    }//end section
    else if operation == "<=".to_string() 
    {//begin section
        if type_expression == "FLOAT".to_string()
        {
           let val1:f64 = element_1.parse().unwrap();
           let val2:f64 = element_2.parse().unwrap();
           if !(val1 <= val2)
           {
            self.descriptor = final_vec;
           }
        }
        else if type_expression == "INT".to_string() 
        {
            let val1:i128 = element_1.parse().unwrap();
            let val2:i128 = element_2.parse().unwrap();
            if !(val1 <= val2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            if !(element_1 <= element_2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "CHAR".to_string() 
        {
            if !(element_1 <= element_2)
            {
                self.descriptor = final_vec;
            }
        }

    }//end section
    else if operation == ">=".to_string() 
    {//begin section
        if type_expression == "FLOAT".to_string()
        {
           let val1:f64 = element_1.parse().unwrap();
           let val2:f64 = element_2.parse().unwrap();
           if !(val1 >= val2)
           {
            self.descriptor = final_vec;
           }
        }
        else if type_expression == "INT".to_string() 
        {
            let val1:i128 = element_1.parse().unwrap();
            let val2:i128 = element_2.parse().unwrap();
            if !(val1 >= val2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            if !(element_1 >= element_2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "CHAR".to_string() 
        {
            if !(element_1 >= element_2)
            {
                self.descriptor = final_vec;
            }
        }

    }//end section
    else if operation == "<>".to_string() 
    {//begin section
        if type_expression == "FLOAT".to_string()
        {
           let val1:f64 = element_1.parse().unwrap();
           let val2:f64 = element_2.parse().unwrap();
           if !(val1 != val2)
           {
            self.descriptor = final_vec;
           }
        }
        else if type_expression == "INT".to_string() 
        {
            let val1:i128 = element_1.parse().unwrap();
            let val2:i128 = element_2.parse().unwrap();
            if !(val1 != val2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            if !(element_1 != element_2)
            {
                self.descriptor = final_vec;
            }
        }
        else if type_expression == "CHAR".to_string() 
        {
            if !(element_1 != element_2)
            {
                self.descriptor = final_vec;
            }
        }

    }//end section
    else if operation == "<".to_string() 
    {//begin section
            if type_expression == "FLOAT".to_string()
            {
               let val1:f64 = element_1.parse().unwrap();
               let val2:f64 = element_2.parse().unwrap();
               if !(val1 < val2)
               {
                self.descriptor = final_vec;
               }
            }
            else if type_expression == "INT".to_string() 
            {
                let val1:i128 = element_1.parse().unwrap();
                let val2:i128 = element_2.parse().unwrap();
                if !(val1 < val2)
                {
                    self.descriptor = final_vec;
                }
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                if !(element_1 < element_2)
                {
                    self.descriptor = final_vec;
                }
            }
            else if type_expression == "CHAR".to_string() 
            {
                if !(element_1 < element_2)
                {
                    self.descriptor = final_vec;
                }
            }
    
    }//end section
    else if operation == ">".to_string() 
    {//begin section
            if type_expression == "FLOAT".to_string()
            {
               let val1:f64 = element_1.parse().unwrap();
               let val2:f64 = element_2.parse().unwrap();
               if !(val1 > val2)
               {
                self.descriptor = final_vec;
               }
            }
            else if type_expression == "INT".to_string() 
            {
                let val1:i128 = element_1.parse().unwrap();
                let val2:i128 = element_2.parse().unwrap();
                if !(val1 > val2)
                {
                    self.descriptor = final_vec;
                }
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                if !(element_1 > element_2)
                {
                    self.descriptor = final_vec;
                }
            }
            else if type_expression == "CHAR".to_string() 
            {
                if !(element_1 > element_2)
                {
                    self.descriptor = final_vec;
                }
            }
    
        }//end section    
    /* 
    for i in 1..self.descriptor.len()
    {

    }*/
    

}

#[doc = "Methode use for the interpretation of a boolean statement. Need two WhereElement, the operator (=,<>...) and the type (INT,FLOAT,CHAR)"]
pub fn predicat_interpretation_with_no_const (&mut self, operation : String, type_expression: String, element_1 : WhereElement,element_2:WhereElement)
{
    let mut index;
    let mut index2;
    let mut i = 0 ;
    let mut final_vec:Vec<Vec<String>>= Vec::new();
    final_vec.push(self.descriptor[0].to_vec());
    while i<self.descriptor[0].len() && element_1.where_value.to_string() != self.descriptor[0][i].to_string() {
        i = i + 1;       
    }
    index = i;
    i=0;
    while i<self.descriptor[0].len() && element_2.where_value.to_string() != self.descriptor[0][i].to_string() {
        i = i + 1;       
    }
    index2 = i;
    
    if operation == "=".to_string()
    {
        if type_expression == "FLOAT".to_string()
        {
            for i in 1..self.descriptor.len()
            {
                let value:f64 = self.descriptor[i][index].clone().parse().unwrap();
                let value2:f64 = self.descriptor[i][index2].clone().parse().unwrap(); 
                if (value == value2 ) {
                    //self.descriptor.remove(i);
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            

        }
        else if type_expression == "INT".to_string() 
        {
            
            for i in 1..self.descriptor.len()
            {
                let value:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                let value2:i128 = self.descriptor[i][index2].clone().parse().unwrap(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            let value = element_1;
            for i in 1..self.descriptor.len()
            {
                let value = self.descriptor[i][index].clone();
                let value2 = self.descriptor[i][index2].clone(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            
        }
        else if type_expression == "CHAR".to_string() 
        {
            let value = element_1;
            for i in 1..self.descriptor.len()
            {
                let value = self.descriptor[i][index].clone();
                let value2 = self.descriptor[i][index2].clone(); 
                if (value == value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }

    }
    else if operation == "<=".to_string() 
    {//begin section
        if type_expression == "FLOAT".to_string()
        {
            for i in 1..self.descriptor.len()
            {
                let value:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                let value2:f64 = self.descriptor[i][index2].clone().parse().unwrap(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            

        }
        else if type_expression == "INT".to_string() 
        {
            for i in 1..self.descriptor.len()
            {
                let value:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                let value2:i128 = self.descriptor[i][index2].clone().parse().unwrap(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "VARCHAR".to_string() 
        {
            for i in 1..self.descriptor.len()
            {
                let value = self.descriptor[i][index].clone(); 
                let value2 = self.descriptor[i][index2].clone(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
        else if type_expression == "CHAR".to_string() 
        {
            for i in 1..self.descriptor.len()
            {                
                let value = self.descriptor[i][index].clone(); 
                let value2 = self.descriptor[i][index2].clone(); 
                if (value <= value2 ) {
                    final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
        }
    }//end section
    else if operation == ">=".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                for i in 1..self.descriptor.len()
                {
                    let value:f64 = self.descriptor[i][index].clone().parse().unwrap();
                    let value2:f64 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value:i128 = self.descriptor[i][index].clone().parse().unwrap();
                    let value2:i128 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                ;
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone();
                    let value2 = self.descriptor[i][index2].clone(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone();
                    let value2 = self.descriptor[i][index2].clone(); 
                    if (value >= value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
    else if operation == "<>".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                for i in 1..self.descriptor.len()
                {
                    let value:f64 = self.descriptor[i][index].clone().parse().unwrap();
                    let value2:f64 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    let value2:i128 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone();
                    let value2 = self.descriptor[i][index2].clone(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone(); 
                    let value2 = self.descriptor[i][index2].clone(); 
                    if (value != value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
        else if operation == "<".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                for i in 1..self.descriptor.len()
                {
                    let value:f64 = self.descriptor[i][index].clone().parse().unwrap();
                    let value2:f64 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    let value2:i128 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone(); 
                    let value2 = self.descriptor[i][index2].clone(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone(); 

                    let value2 = self.descriptor[i][index2].clone(); 
                    if (value < value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
        else if operation == ">".to_string() 
        {//begin section
            if type_expression == "FLOAT".to_string()
            {
                for i in 1..self.descriptor.len()
                {
                    let value:f64 = self.descriptor[i][index].clone().parse().unwrap(); 
                    let value2:f64 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
    
            }
            else if type_expression == "INT".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value:i128 = self.descriptor[i][index].clone().parse().unwrap(); 
                    let value2:i128 = self.descriptor[i][index2].clone().parse().unwrap(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                    }
                }
                self.descriptor = final_vec;
                
            }
            else if type_expression == "VARCHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone(); 
                    let value2 = self.descriptor[i][index2].clone(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
            else if type_expression == "CHAR".to_string() 
            {
                for i in 1..self.descriptor.len()
                {
                    let value = self.descriptor[i][index].clone(); 
                    let value2 = self.descriptor[i][index].clone(); 
                    if (value > value2 ) {
                        final_vec.push(self.descriptor[i].to_vec());
                }
            }
            self.descriptor = final_vec;
            
            }
        }//end section
    /* 
    for i in 1..self.descriptor.len()
    {

    }*/
    

}


#[doc = "Method to add a column on a table when we use a agregate methode (SUM,MIN,MAX...)\n"]
pub fn add_column_for_agregate(&mut self,column:&Vec<String>)
{
    if column.len() != 0
    {
        self.descriptor[0].push(column[0].to_string());
        self.descriptor[1].push(column[1].to_string());
        /* 
        for i in 1..self.descriptor.len()
        {
            self.descriptor[i].push(column[1].to_string());
        }*/
    }
    
} 



pub fn set_name(&mut self, name:&String)
{
    self.name = name.to_string();
}

pub fn set_descriptor(&mut self, list_data:&Vec<String>)
{
    let mut vec1 = Vec::new();
    vec1.push(list_data.to_vec());
    self.descriptor = vec1;
}


#[doc = "Methode to count the number of line for a columns. If the parameter is \"*\"then the NULL and NIL value are not counted.
return a Vec of string with the name of the attribute and the value of the COUNT.
TODO ==> Need to be tested"]
pub fn count(self,attribut_count:&String) -> Vec<String>
{
    let mut result_vec = Vec::new();
    result_vec.push("COUNT(".to_string()+attribut_count+")");
    let mut counter:usize = 0;
    if attribut_count=="*" {counter = self.descriptor.len()-1;}//If we have * then we just have to count the numbers of rows with ".len()" method
    else//Else we have to count all the row, excluding the NULL and NIL value
    { 
            let mut index: usize = 0;
            for i in 0..self.descriptor[0].len()
            {
                if self.descriptor[0][i] == attribut_count.to_string()
                {
                    index = i;
                    break;
                }
            }
            if self.descriptor[0][index] != attribut_count.to_string()
            {
                panic!("Wrong attribute value");
            }

            for i in 1..self.descriptor.len()
            {
                if (self.descriptor[i][index] != "NULL" || self.descriptor[i][index] != "NILL")
                {
                    counter = counter+1;
                }
                
            }
        

        }
        
    result_vec.push(counter.to_string());

    return result_vec;
}


pub fn sum(self,attribut: &String,type_attr:&String)->Vec<String>
{
    let mut result_vec = Vec::new();
    result_vec.push("SUM(".to_string()+attribut+")");
    let mut sum = 0;
    let mut sum2 : f64 = 0.0;
    let mut index: usize = 0;
    for i in 0..self.descriptor[0].len()
    {
        if self.descriptor[0][i] == attribut.to_string()
        {
            index = i;
            break;
        }
    }
    for i in 1..self.descriptor.len()
    {
        if (self.descriptor[i][index] != "NULL" || self.descriptor[i][index] != "NILL")
        {
            if type_attr == "INT"
            {
                let value : i128 = self.descriptor[i][index].parse().unwrap();
                sum=sum+value;
            }
            else if type_attr == "FLOAT"
            {
                let value : f64 = self.descriptor[i][index].parse().unwrap();
                sum2=sum2+value;
            }
        }
                
    }
    if type_attr == "INT"
            {
                result_vec.push(sum.to_string());
                return result_vec;
            }
    else if type_attr == "FLOAT"
    {
        result_vec.push(sum2.to_string());
        return result_vec;
    }
    else {
        result_vec.push("Err".to_string());
        return result_vec;
    }
    
}

pub fn min(self,attribut: &String,type_attr: &String)->Vec<String>
{
    let mut return_vec = Vec::new();
    return_vec.push("MIN(".to_string()+attribut+")");
    let mut index=0;
    for i in 0..self.descriptor[0].len()
    {
        if self.descriptor[0][i] == attribut.to_string()
        {
            index = i;
            break;
        }
    }
    if (type_attr=="FLOAT"||type_attr=="INT")
    {
        let mut minus_str = &self.descriptor[1][index].clone();
        let mut minus:f64 = minus_str.parse().unwrap();
        for i in 2..self.descriptor.len()
        {
            let mut minus_str = &self.descriptor[i][index].clone();
            let test :f64 = minus_str.parse().unwrap();
            if test < minus
            {
                minus = test;
            }
        }
        return_vec.push(minus.to_string());
    }
    else 
    {
        let mut minus = &self.descriptor[1][index];
        for i in 2..self.descriptor.len()
        {
            let test = &self.descriptor[i][index];
            if test < minus
            {
                minus = test;
            }
        }
        return_vec.push(minus.to_string());    
    }
    return return_vec;
}


pub fn max(self,attribut: &String,type_attr: &String)->Vec<String>
{
    let mut return_vec = Vec::new();
    return_vec.push("MAX(".to_string()+attribut+")");
    let mut index=0;
    for i in 0..self.descriptor[0].len()
    {
        if self.descriptor[0][i] == attribut.to_string()
        {
            index = i;
            break;
        }
    }
    if (type_attr=="FLOAT"||type_attr=="INT")
    {
        let mut maxi_str = &self.descriptor[1][index].clone();
        let mut maxi:f64 = maxi_str.parse().unwrap();
        for i in 2..self.descriptor.len()
        {
            let mut maxi_str = &self.descriptor[i][index].clone();
            let test :f64 = maxi_str.parse().unwrap();
            if test > maxi
            {
                maxi = test;
            }
        }
        return_vec.push(maxi.to_string());
    }
    else 
    {
        let mut maxi = &self.descriptor[1][index];
        for i in 2..self.descriptor.len()
        {
            let test = &self.descriptor[i][index];
            if test > maxi
            {
                maxi = test;
            }
        }
        return_vec.push(maxi.to_string());    
    }
    return return_vec;
}


pub(crate)fn avg(self,attribut: &String,type_attr:&String)->Vec<String>
{
    let mut result_vec = Vec::new();
    result_vec.push("SUM(".to_string()+attribut+")");
    let mut sum = 0;
    let mut sum2 : f64 = 0.0;
    let mut index: usize = 0;
    for i in 0..self.descriptor[0].len()
    {
        if self.descriptor[0][i] == attribut.to_string()
        {
            index = i;
            break;
        }
    }
    for i in 1..self.descriptor.len()
    {
        if (self.descriptor[i][index] != "NULL" || self.descriptor[i][index] != "NILL")
        {
            if type_attr == "INT"
            {
                let value : i128 = self.descriptor[i][index].parse().unwrap();
                sum=sum+value;
            }
            else if type_attr == "FLOAT"
            {
                let value : f64 = self.descriptor[i][index].parse().unwrap();
                sum2=sum2+value;
            }
        }
                
    }
    if type_attr == "INT"
            {
                let mut avg: f64 = sum as f64;
                avg=avg/((self.descriptor.len()-1) as f64);
                result_vec.push(avg.to_string());
                return result_vec;
            }
    else if type_attr == "FLOAT"
    {
        sum2 = sum2/((self.descriptor.len()-1) as f64);
        result_vec.push(sum2.to_string());
        return result_vec;
    }
    else {
        result_vec.push("Err".to_string());
        return result_vec;
    }
    
}


#[doc =r"Write a CSV file with the descriptor in ./data/transferFile/result.csv file "]
pub fn to_file(&self)->Result<File,Box<dyn Error>>{
        let mut file:File = match OpenOptions::new().read(true).write(true).truncate(true).create(true).open("./data/transferFile/result.csv") {
            Ok(e) => e,
            Err(e) =>  return Err(Box::new(e)),
        };
        
        //File::create("./data/transferFile/result.csv").expect("Error : Can't create the resultFile");
        file.write_all(self.to_string().as_bytes());
        file.rewind();
        Ok(file)
    }

#[doc = r"To string of the descriptor who separate the attribute with ',' and the ligne with '\\r\\n' if you use a Windows or '\\n' if you use Linux or Max OS."]
pub fn to_string(&self) -> String{
        let mut result_string : String="".to_string();
        for ligne in 0..self.descriptor.len()-1{
            result_string = result_string+ &self.descriptor[ligne].clone().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";");
            if (std::env::consts::OS == "windows" ){
                result_string = result_string+"\r\n";
            }
            else{
                result_string = result_string+"\n";
            }
        }
        result_string = result_string+ &self.descriptor[self.descriptor.len()-1].clone().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";");
        result_string 
    }

#[doc = r"print the name of the CSVFile, usefull for debug"]
    fn print_csv_file(&self){
        println!("{}",&self.name);
    }

#[doc = r"The projection operator, the method select the columns write in list_attribute. To do this, the projection need to inverse the ligne and columns, that operation cost O(n²). This for a final complexity of O(3n²+2n)"]
pub fn projection(&mut self,list_attribute:Vec<String>){
    //println!("{:?}",self.to_string());
    //println!("{:?}",list_attribute);
        let mut transpose: Vec<Vec<String>> = Vec::new();
        for i in 0..self.descriptor[0].len(){
            transpose.push(Vec::new());
        }
        //println!("{:?}",transpose);
        for i in 0..self.descriptor.len(){//We transpose the matrix to simplify the operation (we change columns and ligne)
            for y in 0..self.descriptor[i].len(){
                let a =&self.descriptor[i][y];
                transpose[y].push(a.to_string());
            }
        }
        //println!("{:?}",transpose);
        let mut pre_result:Vec<Vec<String>>=Vec::new();//we keep the ligne we want
        for i in transpose{
            for y in &list_attribute{
                if i[0]==y.to_string(){
                    let a = &i;
                    pre_result.push(a.to_vec());
                }
            }
        }
        //println!("{:?}",pre_result);
        let mut transpose: Vec<Vec<String>> = Vec::new();//we made anoter transpose to have the ligne a view by ligne
        for i in 0..pre_result[0].len(){
            transpose.push(Vec::new());
        }
        //println!("{:?}",transpose);
        for i in 0..pre_result.len(){
            for y in 0..pre_result[i].len(){
                let a =&pre_result[i][y];
                transpose[y].push(a.to_string());
            }
        }
        //println!("{:?}",transpose);
        self.descriptor = transpose.to_vec();
    }
#[doc = "Method for the cartesian product. Need another CSVFile in input and the self object take the cartesian product between self and another_csv\n
Complexity of O(n²)"]
pub fn cartesian_product(&mut self,another_csv: &CSVFile){
    let mut operation_result : Vec<Vec<String>>=Vec::new();
    let mut transition: Vec<String>;
    transition = self.descriptor[0].clone();
    transition.append(&mut another_csv.descriptor[0].clone());
    operation_result.push(transition);
    for i in 1..self.descriptor.len(){
        transition = self.descriptor[i].clone();
        //println!("{:?}",transition);
        for y in 1..another_csv.descriptor.len(){
            let mut transition2 = transition.clone();
            //println!("{:?}{:?}",transition2,&mut another_csv.descriptor[y]);
            transition2.append(&mut another_csv.descriptor[y].clone());
            operation_result.push(transition2);
           }
        
        }
        self.descriptor = operation_result;
    }

pub fn replace_as (&mut self,dico:&HashMap<String,String>)
{
    for i in 0..self.descriptor[0].len()
    {
        if dico.contains_key(&self.descriptor[0][i].to_string())
        {
            self.descriptor[0][i]=dico[&self.descriptor[0][i].to_string()].to_string();
        }
    }
}

#[doc = "methode for the union betwen two CSVFile. Need in input anoter CSVFile. Return nothing because the result of the union is save on the struct."]
pub fn union(&mut self,union_csv:&CSVFile)
{
    let mut result_operation : &mut Vec<Vec<String>> = &mut self.descriptor;
    let mut union_value = &union_csv.descriptor;
    for i in 1..union_value.len()
    {
        if (result_operation[i]!=union_value[i])
        {
            let val = &union_value[i];
            result_operation.push(val.to_vec());
        }
    }
    self.descriptor = result_operation.to_vec();
}




}


#[allow(unused)]
#[doc = r"This fonction take the name of the CSV file and read this file in the ../data/CSV/ directory. That function return of Vec of Vec of String who represent the CSV file ligne by ligne"]
pub fn csv_read_by_ligne(path_file:String,table_name:String)-> Result<Vec<Vec<String>>,Box<dyn Error>>{
    let mut path:String = "./data/CSV/".to_string();
    path.push_str(&path_file);
    path.push_str(".csv");
    let reader = match File::open(path) {
        Ok(e) => e,
        Err(e) => return Err(Box::new(e)),
    };
    let mut buffer = BufReader::new(reader);
    let mut csv_string = String::new();
    let i: usize = match buffer.read_to_string(&mut csv_string) {
        Err(e) => return  Err(Box::new(e)),
        Ok(e) => e,
    } ;
    //println!("{}",std::env::consts::OS);
    let separator_ligne:String= if (std::env::consts::OS == "windows"){
        "\r\n".to_string()
    }
    else {
        "\n".to_string()
    };
    let first_vec :Vec<&str>=csv_string.split(&separator_ligne).collect::<Vec<_>>();
    let mut final_vec: Vec<Vec<_>> = [first_vec[0].split(';').map(|x| x.to_string()).collect()].to_vec();
    for i in 0..final_vec[0].len(){
        final_vec[0][i]= (table_name.clone()+"."+&final_vec[0][i]).to_string();
    }
    for ligne in 1..first_vec.len(){
        final_vec.push(first_vec[ligne].split(';').map(|x| x.to_string()).collect());
    } 
    
    Ok(final_vec)
}
/*
fn csv_read_by_columns(path_file:String)/*->CSVFile*/{
    let mut reader = File::open(path_file).expect("Error there is no file here");
    let mut buffer = BufReader::new(reader);
    let mut csv_string = String::new();
    buffer.read_to_string(&mut csv_string).expect("Can't read this file");
    println!("{}",csv_string);
}
 */

#[allow(unused)]
#[doc = r"Create a CSVFile with the name you want and the name of the CSV file to open"]
pub fn open_relation(pathcsv:String,name1:&String)->Result<CSVFile,Box<dyn Error>>{
    match csv_read_by_ligne(pathcsv,name1.to_string()){
        Ok(res) => Ok(CSVFile { name:name1.to_string(), descriptor: res }),
        Err(e) => Err(e)
    }
    //let file:CSVFile = CSVFile { name:name1, descriptor:   };/*  = CSVFile { name: name1, descriptor:  } */;
    //return file;
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;



    #[test]
    fn test_where1()
    {
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let val1 = WhereElement { where_value: "personneTest.NOM".to_string(),boolean_value: true };
        let val2 = WhereElement { where_value: "Zasanten".to_string(),boolean_value: false };
        table1.predicat_interpretation('='.to_string(), "VARCHAR".to_string(), val1, val2);
        println!("{}",table1.to_string());
    }

    #[test]
    fn test_where2()
    {
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let val1 = WhereElement { where_value: "personneTest.ID".to_string(),boolean_value: true };
        let val2 = WhereElement { where_value: "5".to_string(),boolean_value: false };
        table1.predicat_interpretation('='.to_string(), "INT".to_string(), val1, val2);
        println!("{}",table1.to_string());
    }
    #[test]
    fn test_where3()
    {
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let val1 = WhereElement { where_value: "personneTest.ID".to_string(),boolean_value: true };
        let val2 = WhereElement { where_value: "5".to_string(),boolean_value: false };
        table1.predicat_interpretation("<=".to_string(), "INT".to_string(), val1, val2);
        println!("{}",table1.to_string());
    }
    

    #[test]
    fn test_where1_1()
    {
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let val1 = WhereElement { where_value: "5".to_string(),boolean_value: false };
        let val2 = WhereElement { where_value: "5".to_string(),boolean_value: false };
        table1.predicat_interpretation('='.to_string(), "INT".to_string(), val2, val1);
        println!("{}",table1.to_string());
    }

    #[test]
    fn test_where2_2()
    {
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let val1 = WhereElement { where_value: "55".to_string(),boolean_value: false };
        let val2 = WhereElement { where_value: "5".to_string(),boolean_value: false };
        table1.predicat_interpretation('='.to_string(), "INT".to_string(), val1, val2);
        println!("{}",table1.to_string());
    }
    

    #[test]
    fn test_count()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.ID".to_string();
        let test = table1.count(&path_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,10);
        println!("{:?}",test);
    }

    #[test]
    fn test_sum()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.AGE".to_string();
        let type_str = "INT".to_string();
        let test = table1.sum(&path_str, &type_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,400);
        println!("{:?}",test);
    }

    #[test]
    fn test_min()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.AGE".to_string();
        let type_str = "INT".to_string();
        let test = table1.min(&path_str, &type_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,1);
        println!("{:?}",test);

    }
    #[test]
    fn test_max()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.AGE".to_string();
        let type_str = "INT".to_string();
        let test = table1.max(&path_str, &type_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,85);
        println!("{:?}",test);

    }

    #[test]
    fn test_avg()
    {
        let table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let path_str="personneTest.AGE".to_string();
        let type_str = "INT".to_string();
        let test = table1.avg(&path_str, &type_str);
        let value:u128 = test[1].parse().unwrap();
        assert_eq!(value,40);
        println!("{:?}",test);
    }

    #[test]
    fn test_union()
    {
        let mut table1 = open_relation("personneTest".to_string(), &"personneTest".to_string()).expect("Error");
        let table2 = open_relation("personneTest2".to_string(), &"personneTest2".to_string()).expect("Error");
        table1.union(&table2);
        let str = table1.to_string();
        //print!("{}",str);
        let mut val2 = File::open("./data/expectedFile/unionTest1Expected.csv").expect("error");
        let mut str_compare=String::new();
        let _ = val2.read_to_string(&mut str_compare).expect("Error");
        //str_compare.to_string();
        let string_compare = str_compare.to_string(); 
        if string_compare==str
        {
            assert!(true);
        }
        else 
        {
            assert!(false);
        }
    }

    #[test]
    fn test1(){
        let res = open_relation("personneTest".to_string(), &"personneTest".to_string());
        let mut a1 : CSVFile;
        match res {
            Ok(o) => a1 = o,
            Err(..) => panic!("Error")
         };
        a1.print_csv_file();
        let now = Instant::now();
        println!("{:?}",a1.descriptor[0]);    
        a1.projection(["personneTest.ID".to_string(),"personneTest.PRENOM".to_string()].to_vec());
        let time_passed = now.elapsed();
        println!("The projection with personneTest.csv took {} seconde", time_passed.as_secs());
        let val: String = a1.to_string();
        let mut val2 = File::open("./data/expectedFile/test1Expect.csv").expect("error");
        let mut str_compare=String::new();
        let _ = val2.read_to_string(&mut str_compare).expect("Error");
        str_compare.to_string();
        let string_compare = str_compare.to_string();
        if string_compare==val{
            assert!(true);
        }
        else {
            assert!(false);
        }
        

        //print!("{}",a1.to_string());
        
        //println!("{:?}",a1.descriptor);
        
    }

    #[test]
    #[should_panic]
    fn test_csv_read_ligne(){
        let a1 = "../data/CSV/personneTest.csv".to_string();
        csv_read_by_ligne(a1,"personne".to_string()).expect("TODO: panic message");


    }

    #[test]
    fn test_cartesian() {
        let mut a1 = open_relation("personneTest".to_string(), &"personneTest".to_string());
        let a2 = open_relation("personneTest".to_string(), &"personneTest".to_string());
        match a1{
            Ok(ref mut res1) =>         match a2{
                Ok(res2) => {
                                        res1.cartesian_product(&res2);
                                        let val =res1.to_string();
                                        //println!("{}",val);
                                        let mut val2 = File::open("./data/expectedFile/testCartesian.csv").expect("error");
                                        let mut str_compare=String::new();
                                        let _ = val2.read_to_string(&mut str_compare).expect("Error");
                                        //str_compare.to_string();
                                        let string_compare = str_compare.to_string();
                                        if string_compare==val
                                        {
                                            assert!(true);
                                        }
                                        else 
                                        {
                                            assert!(false);
                                        }
                                    
                                    
                                    
                                    },
                Err(..) => panic!("Error")
            }
            Err(..) => panic!("Error")

        }

        //a1.expect("REASON").cartesian_product( &a2);
        a1.expect("REASON").to_file().expect("error");
    }
}