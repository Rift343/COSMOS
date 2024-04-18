use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use rand::random;

use crate::metadata::check_if_attribute_is_valid;
use crate::structures::semantic_parser_file::{Attribute, ConditionAllowType, Constant, Logical, LogicalAllowType, LogicalNot, SubQuery};
use crate::structures::semantic_parser_file::ConditionAllowType::{Attr, Const, SubQ};
use crate::structures::semantic_parser_file::LogicalAllowType::{Cond, Logi, Not};

use crate::structures::syntaxic_parser_file::{TableNameCouple, Condition, ConditionsAllowType, SyntaxicParserFile};
use crate::structures::table_metadata::TableMetadata;

use crate::structures::syntaxic_parser_file;

#[derive(Debug)]
pub enum Check {
    Val(String),
    SubQ(String),
}

fn transform_to_attr(given_attribute: &String, table_metadata_as_struct: &HashMap<String, TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<TableNameCouple>) -> Result<(String, ConditionAllowType), Box<dyn Error>> {
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
            Attr(
                Attribute {
                    etype: String::from("attribute"),
                    use_name_table: table_name,
                    attribute_name: given_attribute.clone(),
                }
            )
        )
    )
}

fn transform_to_const(given_attribute: &String) -> Result<ConditionAllowType, Box<dyn Error>> {
    Ok(
        Const(
            Constant {
                etype: String::from("constant"),
                value: given_attribute.clone(),
            }
        )
    )
}

fn transform_to_condition_allow_type<'a>(given_object: &'a syntaxic_parser_file::ConditionAllowType, table_metadata_as_struct: &HashMap<String, TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a SyntaxicParserFile>) -> Result<(String, ConditionAllowType), Box<dyn Error>> {
    match given_object {
        syntaxic_parser_file::ConditionAllowType::Str(given_attr_or_const) => {
            if given_attr_or_const.contains('"') {
                return Ok((String::from("VARCHAR"), transform_to_const(&given_attr_or_const)?));
            } else {
                if given_attr_or_const.parse::<usize>().is_ok() {
                    return Ok((String::from("INT"), transform_to_const(&given_attr_or_const)?));
                } else if given_attr_or_const.parse::<f32>().is_ok() {
                    return Ok((String::from("FLOAT"), transform_to_const(&given_attr_or_const)?));
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
                    SubQ(
                        SubQuery {
                            etype: "subquery".to_string(),
                            query: id,
                        }
                    ),
                )
            );
        }
    }
}

fn transform_to_cond<'a>(given_condition: &'a Condition, table_metadata_as_struct: &HashMap<String, TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a SyntaxicParserFile>, subquery_checking: &mut Vec<(String, Check, Rc<RefCell<crate::structures::semantic_parser_file::Condition>>)>) -> Result<LogicalAllowType, Box<dyn Error>> {
    let (left_datatype, left) = transform_to_condition_allow_type(&given_condition.left, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap)?;
    let (right_datatype, right) = transform_to_condition_allow_type(&given_condition.right, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap)?;

    let datatype: String;

    let mut left_subquery_id: Option<String> = None;
    let mut right_subquery_id: Option<String> = None;

    match &left {
        SubQ(left_subquery) => {
            left_subquery_id = Some(left_subquery.query.clone());
        }
        _ => {()}
    }

    match &right {
        SubQ(right_subquery) => {
            right_subquery_id = Some(right_subquery.query.clone());
        }
        _ => {()}
    }

    if (left_subquery_id == None) && (right_subquery_id == None){
        if left_datatype != right_datatype {
            return Err(Box::from(format!("Incompatible datatypes used in where (No implicit conversion) : {} ({}) and {} ({})\n", left, left_datatype, right, right_datatype)));
        }

        datatype = left_datatype.clone();
    }
    else {
        datatype = String::from("");
    }

    let temp = Rc::new(RefCell::new(
        crate::structures::semantic_parser_file::Condition {
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
                    subquery_checking.push((left_id, Check::SubQ(right_id), Rc::clone(&temp)))
                },
                None => {
                    subquery_checking.push((left_id, Check::Val(right_datatype), Rc::clone(&temp)))
                }
            }
        }
        None => {
            match right_subquery_id {
                Some(right_id) => {
                    subquery_checking.push((right_id, Check::Val(left_datatype.clone()), Rc::clone(&temp)))
                },
                None => {
                    ()
                }
            }
        }
    }

    Ok(
        Cond(
            temp
        )
    )
}

fn transform_to_logi(linker: &String, left: LogicalAllowType, right: LogicalAllowType) -> LogicalAllowType {
    Logi(
        Box::from(Logical {
            etype: "logical".to_string(),
            operator: linker.clone(),
            left,
            right,
        })
    )
}

fn transform_to_not(linker: &String, left: LogicalAllowType) -> LogicalAllowType {
    Not(Box::from(LogicalNot {
        etype: "logical".to_string(),
        operator: linker.clone(),
        left,
    }))
}

pub fn handle_where<'a>(condition_list: &'a Vec<ConditionsAllowType>, linker_list: &Vec<String>, start: usize, end: usize, table_metadata_as_struct: &HashMap<String, TableMetadata>, renamed_table_name_map: &HashMap<String, String>, selected_table_list: &Vec<TableNameCouple>, subquery_hasmap: &mut HashMap<String, &'a SyntaxicParserFile>, subquery_checking: &mut Vec<(String, Check, Rc<RefCell<crate::structures::semantic_parser_file::Condition>>)>) -> Result<(usize, usize, LogicalAllowType), Box<dyn Error>> {
    if end == start {
        return if (linker_list[start] == "OR") || (linker_list[start] == "AND") {
            let left = match &condition_list[start] {
                ConditionsAllowType::Cond(left_cond) => {
                    transform_to_cond(left_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
                ConditionsAllowType::SubCond(c) => {
                    handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
                }
            };

            let right = match &condition_list[start + 1] {
                ConditionsAllowType::Cond(right_cond) => {
                    transform_to_cond(right_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
                ConditionsAllowType::SubCond(c) => {
                    handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
                }
            };

            let res = transform_to_logi(&linker_list[start], left, right);

            Ok((start, start + 1, res))
        } else {
            let left = match &condition_list[start] {
                ConditionsAllowType::Cond(left_cond) => {
                    transform_to_cond(left_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
                }
                ConditionsAllowType::SubCond(c) => {
                    handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
                }
            };

            let res = transform_to_not(&linker_list[start], left);

            Ok((start, start, res))
        };
    }

    let mut min_index = end;
    let mut temp = &linker_list[min_index];

    for i in start..(end + 1) {
        if linker_list[i] == "OR" {
            min_index = i;
            temp = &linker_list[min_index];
        } else if linker_list[i] == "AND" && temp == "NOT" {
            min_index = i;
            temp = &linker_list[min_index];
        }
    }

    let mut start_end_range: (usize, usize) = (min_index, min_index + 1);

    let left: LogicalAllowType;
    let right: LogicalAllowType;

    if min_index > start {
        (start_end_range.0, _, left) = handle_where(condition_list, linker_list, start, min_index - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?;
    } else {
        left = match &condition_list[min_index] {
            ConditionsAllowType::Cond(left_cond) => {
                transform_to_cond(left_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
            ConditionsAllowType::SubCond(c) => {
                handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
            }
        };
    }

    if min_index < end {
        (_, start_end_range.1, right) = handle_where(condition_list, linker_list, min_index + 1, end, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?;
    } else {
        right = match &condition_list[min_index + 1] {
            ConditionsAllowType::Cond(right_cond) => {
                transform_to_cond(right_cond, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?
            }
            ConditionsAllowType::SubCond(c) => {
                handle_where(&c.conditions, &c.linkers, 0, c.linkers.len() - 1, table_metadata_as_struct, renamed_table_name_map, selected_table_list, subquery_hasmap, subquery_checking)?.2
            }
        };
    }

    return Ok((start_end_range.0, start_end_range.1, transform_to_logi(&linker_list[min_index], left, right)));
}