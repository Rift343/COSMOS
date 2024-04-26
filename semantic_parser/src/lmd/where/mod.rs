use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use rand::random;
use crate::lmd::semantic_parser;
use crate::lmd::r#where;
use crate::structures::table_metadata;

use crate::structures::semantic_parser_file;

use crate::metadata::check_if_attribute_is_valid;
use crate::structures::semantic_parser_file::{Attribute};


use crate::structures::syntaxic_parser_file;

#[derive(Debug)]
pub enum SubQHashMapAllowType {
    Cond(Rc<RefCell<semantic_parser_file::Condition>>),
    Checker(Rc<RefCell<semantic_parser_file::Checker>>),
}

#[derive(Debug)]
pub enum Check {
    Val(String),
    SubQ(String),
}

fn transform_to_attr(given_attribute: &String, table_metadata_as_struct: &HashMap<String, table_metadata::TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<syntaxic_parser_file::TableNameCouple>) -> Result<(String, semantic_parser_file::ConditionAllowType), Box<dyn Error>> {
    let table_name = check_if_attribute_is_valid(table_metadata_as_struct, &given_attribute, &"".to_string(), renamed_table_name_map, selected_table_list)?;

    let mut attribute_type = {
        match table_metadata_as_struct.get(&table_name) {
            None => {
                return Err(Box::from(format!("Unknown error : {} not found in metadata despite validation. (transform_to_attr)\n", table_name)));
            }
            Some(table_metadata) => {
                table_metadata.get_type_of_attribute(given_attribute)?
            }
        }
    };

    if attribute_type.starts_with("VARCHAR") {
        attribute_type.truncate(7);
    }

    Ok(
        (
            attribute_type,
            semantic_parser_file::ConditionAllowType::Attr(
                Attribute {
                    etype: String::from("attribute"),
                    use_name_table: table_name,
                    attribute_name: given_attribute.clone(),
                }
            )
        )
    )
}

fn transform_to_const(given_attribute: &String) -> Result<semantic_parser_file::ConditionAllowType, Box<dyn Error>> {
    Ok(
        semantic_parser_file::ConditionAllowType::Const(
            semantic_parser_file::Constant {
                etype: String::from("constant"),
                value: given_attribute.clone(),
            }
        )
    )
}

fn get_type_of_object(given_object: &String) -> String {
    if (given_object.starts_with('"') || given_object.starts_with("'")) {
        return String::from("VARCHAR");
    }
    if given_object.parse::<usize>().is_ok() {
        return String::from("INT");
    }
    if given_object.parse::<f64>().is_ok() {
        return String::from("FLOAT");
    }
    return String::from("ATTR");
}

fn transform_to_condition_allow_type<'a>(given_object: &'a syntaxic_parser_file::ConditionAllowType, table_metadata_as_struct: &HashMap<String, table_metadata::TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<syntaxic_parser_file::TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a syntaxic_parser_file::SyntaxicParserFile>) -> Result<(String, semantic_parser_file::ConditionAllowType), Box<dyn Error>> {
    match given_object {
        syntaxic_parser_file::ConditionAllowType::Str(given_attr_or_const) => {
            if (given_attr_or_const.contains('"') || given_attr_or_const.contains("'")) {
                return Ok((get_type_of_object(&given_attr_or_const), transform_to_const(&given_attr_or_const)?));
            } else {
                if given_attr_or_const.parse::<usize>().is_ok() {
                    return Ok((get_type_of_object(&given_attr_or_const), transform_to_const(&given_attr_or_const)?));
                } else if given_attr_or_const.parse::<f32>().is_ok() {
                    return Ok((get_type_of_object(&given_attr_or_const), transform_to_const(&given_attr_or_const)?));
                }
            }

            return Ok(transform_to_attr(&given_attr_or_const, table_metadata_as_struct, renamed_table_name_map, selected_table_list)?);
        }

        syntaxic_parser_file::ConditionAllowType::SubQuery(given_subquery) => {
            let mut id: String = random::<u32>().to_string();

            while subquery_hasmap.contains_key(&id) {
                id = random::<u32>().to_string();
            }

            subquery_hasmap.insert(id.clone(), given_subquery);

            return Ok(
                (
                    "".to_string(),
                    semantic_parser_file::ConditionAllowType::SubQ(
                        semantic_parser_file::SubQuery {
                            etype: "subquery".to_string(),
                            query: id,
                        }
                    ),
                )
            );
        }
    }
}

fn transform_to_cond<'a>(given_condition: &'a syntaxic_parser_file::Condition, table_metadata_as_struct: &HashMap<String, table_metadata::TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<syntaxic_parser_file::TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a syntaxic_parser_file::SyntaxicParserFile>, subquery_checking: &mut Vec<(String, Check, r#where::SubQHashMapAllowType)>) -> Result<semantic_parser_file::LogicalAllowType, Box<dyn Error>> {
    let (left_datatype, left) = transform_to_condition_allow_type(&given_condition.left, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap)?;
    let (right_datatype, right) = transform_to_condition_allow_type(&given_condition.right, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap)?;

    let datatype: String;

    let mut left_subquery_id: Option<String> = None;
    let mut right_subquery_id: Option<String> = None;

    match &left {
        semantic_parser_file::ConditionAllowType::SubQ(left_subquery) => {
            left_subquery_id = Some(left_subquery.query.clone());
        }
        _ => { () }
    }

    match &right {
        semantic_parser_file::ConditionAllowType::SubQ(right_subquery) => {
            right_subquery_id = Some(right_subquery.query.clone());
        }
        _ => { () }
    }

    if (left_subquery_id == None) && (right_subquery_id == None) {
        if left_datatype != right_datatype {
            return Err(Box::from(format!("Incompatible datatypes used in where (No implicit conversion) : {} ({}) and {} ({})\n", left, left_datatype, right, right_datatype)));
        }

        datatype = left_datatype.clone();
    } else {
        datatype = String::from("");
    }

    let temp = Rc::new(RefCell::new(
        semantic_parser_file::Condition {
            etype: String::from("condition"),
            condition: given_condition.op.clone(),
            datatype,
            left,
            right,
        }
    ));

    match left_subquery_id {
        Some(left_id) => {
            match right_subquery_id {
                Some(right_id) => {
                    subquery_checking.push((left_id, Check::SubQ(right_id), SubQHashMapAllowType::Cond(Rc::clone(&temp))))
                }
                None => {
                    subquery_checking.push((left_id, Check::Val(right_datatype), SubQHashMapAllowType::Cond(Rc::clone(&temp))))
                }
            }
        }
        None => {
            match right_subquery_id {
                Some(right_id) => {
                    subquery_checking.push((right_id, Check::Val(left_datatype.clone()), SubQHashMapAllowType::Cond(Rc::clone(&temp))))
                }
                None => {
                    ()
                }
            }
        }
    }

    Ok(
        semantic_parser_file::LogicalAllowType::Cond(
            temp
        )
    )
}

fn transform_to_logi(linker: &String, left: semantic_parser_file::LogicalAllowType, right: semantic_parser_file::LogicalAllowType) -> semantic_parser_file::LogicalAllowType {
    semantic_parser_file::LogicalAllowType::Logi(
        Box::from(
            semantic_parser_file::Logical {
                etype: "logical".to_string(),
                operator: linker.clone(),
                left,
                right,
            }
        )
    )
}

fn transform_to_not(linker: &String, left: semantic_parser_file::LogicalAllowType) -> semantic_parser_file::LogicalAllowType {
    semantic_parser_file::LogicalAllowType::Not(
        Box::from(
            semantic_parser_file::LogicalNot {
                etype: "logical".to_string(),
                operator: linker.clone(),
                left,
            }
        )
    )
}

fn transform_to_checker_left_type<'a>(left_object: &'a syntaxic_parser_file::CheckerLeftAllowType, table_metadata_as_struct: &HashMap<String, table_metadata::TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<syntaxic_parser_file::TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a syntaxic_parser_file::SyntaxicParserFile>) -> Result<(String, semantic_parser_file::CheckerLeftAllowType), Box<dyn Error>> {
    match left_object {
        syntaxic_parser_file::CheckerLeftAllowType::Str(value) => {
            let datatype = get_type_of_object(value);

            if datatype == String::from("ATTR") {
                let table_name = check_if_attribute_is_valid(table_metadata_as_struct, value, &"".to_string(), renamed_table_name_map, selected_table_list)?;

                let attribute_type = {
                    match table_metadata_as_struct.get(&table_name) {
                        None => {
                            return Err(Box::from(format!("Unknown error : {} not found in metadata despite validation. (transform_to_attr)\n", table_name)));
                        }
                        Some(table_metadata) => {
                            table_metadata.get_type_of_attribute(value)?
                        }
                    }
                };

                Ok(
                    (
                        attribute_type,
                        semantic_parser_file::CheckerLeftAllowType::Attr(
                            Attribute {
                                etype: String::from("attribute"),
                                use_name_table: table_name,
                                attribute_name: value.clone(),
                            }
                        )
                    )
                )
            }

            else {
                Ok(
                    (
                        datatype,
                        semantic_parser_file::CheckerLeftAllowType::Const(
                            semantic_parser_file::Constant {
                                etype: String::from("constant"),
                                value: value.clone(),
                            }
                        )
                    )
                )
            }
        }

        syntaxic_parser_file::CheckerLeftAllowType::SubQuery(given_subquery) => {
            let mut id: String = random::<u32>().to_string();

            while subquery_hasmap.contains_key(&id) {
                id = random::<u32>().to_string();
            }

            subquery_hasmap.insert(id.clone(), given_subquery);

            Ok(
                (
                    String::from(""),
                    semantic_parser_file::CheckerLeftAllowType::SubQ(
                        semantic_parser_file::SubQuery {
                            etype: "subquery".to_string(),
                            query: "".to_string(),
                        }
                    )
                )
            )
        }
    }
}

fn transform_to_checker_right_type<'a>(right_object: &'a syntaxic_parser_file::CheckerRightAllowType, table_metadata_as_struct: &HashMap<String, table_metadata::TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<syntaxic_parser_file::TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a syntaxic_parser_file::SyntaxicParserFile>) -> Result<(String, semantic_parser_file::CheckerRightAllowType), Box<dyn Error>> {
    match right_object {
        syntaxic_parser_file::CheckerRightAllowType::DataLi(objects_list) => {
            // technically allowable ? But we can't infer datatype if list is empty so non-accepted use case
            if (objects_list.len() < 1) {
                return Err(Box::from("Given Datalist in query contains no element (Non-accepted scenario)\n"));
            }

            let mut datatype = get_type_of_object(&objects_list[0]);

            let table_name = check_if_attribute_is_valid(table_metadata_as_struct, &objects_list[0], &"".to_string(), renamed_table_name_map, selected_table_list)?;

            datatype = {
                match table_metadata_as_struct.get(&table_name) {
                    None => {
                        return Err(Box::from(format!("Unknown error : {} not found in metadata despite validation. (transform_to_attr)\n", table_name)));
                    }
                    Some(table_metadata) => {
                        table_metadata.get_type_of_attribute(&objects_list[0])?
                    }
                }
            };

            let mut values: Vec<semantic_parser_file::DatalistAllowType> = Vec::new();

            for element in objects_list {
                let element_datatype = get_type_of_object(element);

                if element_datatype == String::from("ATTR") {
                    let table_name = check_if_attribute_is_valid(table_metadata_as_struct, element, &"".to_string(), renamed_table_name_map, selected_table_list)?;

                    let attribute_type = {
                        match table_metadata_as_struct.get(&table_name) {
                            None => {
                                return Err(Box::from(format!("Unknown error : {} not found in metadata despite validation. (transform_to_attr)\n", table_name)));
                            }
                            Some(table_metadata) => {
                                table_metadata.get_type_of_attribute(element)?
                            }
                        }
                    };

                    if attribute_type != datatype {
                        return Err(Box::from(format!("Given values in datalist mismatched types : {} ({}) and {} ({})\n", datatype, objects_list[0], attribute_type, element)));
                    }

                    values.push(
                        semantic_parser_file::DatalistAllowType::Attr(
                            Attribute {
                                etype: String::from("attribute"),
                                use_name_table: table_name,
                                attribute_name: element.clone(),
                            }
                        )
                    );
                } else {
                    if (datatype != element_datatype) {
                        return Err(Box::from(format!("Given values in datalist mismatched types : {} ({}) and {} ({})\n", datatype, objects_list[0], element_datatype, element)));
                    }

                    values.push(
                        semantic_parser_file::DatalistAllowType::Const(
                            semantic_parser_file::Constant {
                                etype: String::from("constant"),
                                value: element.clone(),
                            }
                        )
                    );
                }
            }

            Ok(
                (
                    datatype,
                    semantic_parser_file::CheckerRightAllowType::DataLi(semantic_parser_file::DataList {
                etype: "datalist".to_string(),
                value: values,
            })))
        }

        syntaxic_parser_file::CheckerRightAllowType::SubQuery(given_subquery) => {
            let mut id: String = random::<u32>().to_string();

            while subquery_hasmap.contains_key(&id) {
                id = random::<u32>().to_string();
            }

            subquery_hasmap.insert(id.clone(), given_subquery);

            Ok(
                (
                    String::from(""),
                    semantic_parser_file::CheckerRightAllowType::SubQ(
                        semantic_parser_file::SubQuery {
                            etype: "subquery".to_string(),
                            query: "".to_string(),
                        }
                    )
                )
            )
        }
    }
}

fn transform_to_checker<'a>(given_checker: &'a syntaxic_parser_file::Checker, table_metadata_as_struct: &HashMap<String, table_metadata::TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<syntaxic_parser_file::TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a syntaxic_parser_file::SyntaxicParserFile>, subquery_checking: &mut Vec<(String, Check, r#where::SubQHashMapAllowType)>) -> Result<semantic_parser_file::LogicalAllowType, Box<dyn Error>> {
    let (left_datatype, left) = transform_to_checker_left_type(&given_checker.left, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap)?;
    let (right_datatype, right) = transform_to_checker_right_type(&given_checker.right, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap)?;

    let datatype: String;

    let mut left_subquery_id: Option<String> = None;
    let mut right_subquery_id: Option<String> = None;

    match &left {
        semantic_parser_file::CheckerLeftAllowType::SubQ(left_subquery) => {
            left_subquery_id = Some(left_subquery.query.clone());
        }
        _ => { () }
    }

    match &right {
        semantic_parser_file::CheckerRightAllowType::SubQ(right_subquery) => {
            right_subquery_id = Some(right_subquery.query.clone());
        }
        _ => { () }
    }

    if (left_subquery_id == None) && (right_subquery_id == None) {
        if left_datatype != right_datatype {
            return Err(Box::from(format!("Incompatible datatypes used in where (No implicit conversion) : {} ({}) and {} ({})\n", left, left_datatype, right, right_datatype)));
        }

        datatype = left_datatype.clone();
    } else {
        datatype = String::from("");
    }

    let temp =
        Rc::new(RefCell::new(semantic_parser_file::Checker {
        etype: "checker".to_string(),
        check_type: "IN".to_string(),
        datatype,
        left,
        right
    }));

    match left_subquery_id {
        Some(left_id) => {
            match right_subquery_id {
                Some(right_id) => {
                    subquery_checking.push((left_id, Check::SubQ(right_id), SubQHashMapAllowType::Checker(Rc::clone(&temp))))
                }
                None => {
                    subquery_checking.push((left_id, Check::Val(right_datatype), SubQHashMapAllowType::Checker(Rc::clone(&temp))))
                }
            }
        }
        None => {
            match right_subquery_id {
                Some(right_id) => {
                    subquery_checking.push((right_id, Check::Val(left_datatype.clone()), SubQHashMapAllowType::Checker(Rc::clone(&temp))))
                }
                None => {
                    ()
                }
            }
        }
    }

    Ok(
        semantic_parser_file::LogicalAllowType::Check(
            temp
        )
    )
}

pub fn handle_where<'a>(condition_list: &'a Vec<syntaxic_parser_file::ConditionsAllowType>, linker_list: &Vec<String>, start: isize, end: isize, table_metadata_as_struct: &HashMap<String, table_metadata::TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<syntaxic_parser_file::TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a syntaxic_parser_file::SyntaxicParserFile>, subquery_checking: &mut Vec<(String, Check, r#where::SubQHashMapAllowType)>) -> Result<(isize, isize, semantic_parser_file::LogicalAllowType), Box<dyn Error>> {
    if linker_list.len() == 0 {
        let temp = match &condition_list[0] {
            syntaxic_parser_file::ConditionsAllowType::Cond(left_cond) => {
                transform_to_cond(left_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
            syntaxic_parser_file::ConditionsAllowType::SubCond(c) => {
                handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() as isize - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
            }
            syntaxic_parser_file::ConditionsAllowType::Check(check) => {
                transform_to_checker(check, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
        };

        return Ok((0, 0, temp));

    }

    if end == start {
        return if (linker_list[start as usize] == "OR") || (linker_list[start as usize] == "AND") {
            let left = match &condition_list[start as usize] {
                syntaxic_parser_file::ConditionsAllowType::Cond(left_cond) => {
                    transform_to_cond(left_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
                syntaxic_parser_file::ConditionsAllowType::SubCond(c) => {
                    handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() as isize - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
                }
                syntaxic_parser_file::ConditionsAllowType::Check(check) => {
                    transform_to_checker(check, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
            };

            let right = match &condition_list[start as usize + 1] {
                syntaxic_parser_file::ConditionsAllowType::Cond(right_cond) => {
                    transform_to_cond(right_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
                syntaxic_parser_file::ConditionsAllowType::SubCond(c) => {
                    handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() as isize - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
                }
                syntaxic_parser_file::ConditionsAllowType::Check(check) => {
                    transform_to_checker(check, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
            };

            let res = transform_to_logi(&linker_list[start as usize], left, right);

            Ok((start, start + 1, res))
        } else {
            let left = match &condition_list[start as usize] {
                syntaxic_parser_file::ConditionsAllowType::Cond(left_cond) => {
                    transform_to_cond(left_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
                syntaxic_parser_file::ConditionsAllowType::SubCond(c) => {
                    handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() as isize - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
                }
                syntaxic_parser_file::ConditionsAllowType::Check(check) => {
                    transform_to_checker(check, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
            };

            let res = transform_to_not(&linker_list[start as usize], left);

            Ok((start, start, res))
        };
    }

    let mut min_index = end;
    let mut temp = &linker_list[min_index as usize];

    for i in start..(end + 1) {
        if linker_list[i as usize] == "OR" {
            min_index = i;
            temp = &linker_list[min_index as usize];
        } else if linker_list[i as usize] == "AND" && temp == "NOT" {
            min_index = i;
            temp = &linker_list[min_index as usize];
        }
    }

    let mut start_end_range: (isize, isize) = (min_index, min_index + 1);

    let left: semantic_parser_file::LogicalAllowType;
    let right: semantic_parser_file::LogicalAllowType;

    if min_index > start {
        (start_end_range.0, _, left) = handle_where(condition_list, linker_list, start, min_index - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?;
    } else {
        left = match &condition_list[min_index as usize] {
            syntaxic_parser_file::ConditionsAllowType::Cond(left_cond) => {
                transform_to_cond(left_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
            syntaxic_parser_file::ConditionsAllowType::SubCond(c) => {
                handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() as isize - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
            }
            syntaxic_parser_file::ConditionsAllowType::Check(check) => {
                transform_to_checker(check, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
        };
    }

    if min_index < end {
        (_, start_end_range.1, right) = handle_where(condition_list, linker_list, min_index + 1, end, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?;
    } else {
        right = match &condition_list[min_index as usize + 1] {
            syntaxic_parser_file::ConditionsAllowType::Cond(right_cond) => {
                transform_to_cond(right_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
            syntaxic_parser_file::ConditionsAllowType::SubCond(c) => {
                handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() as isize - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
            }
            syntaxic_parser_file::ConditionsAllowType::Check(check) => {
                transform_to_checker(check, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
        };
    }

    return Ok((start_end_range.0, start_end_range.1, transform_to_logi(&linker_list[min_index as usize], left, right)));
}