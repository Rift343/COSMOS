import textx
import json

def lmd_parser(query):
    # Get grammar file
    sql_meta = textx.metamodel_from_file("syntaxic_parser/textX_grammar/grammar_file.tx", ignore_case=True)
    try:
        # Analyse SQL query and get textX object
        model = sql_meta.model_from_str(query)

        # Call the handle_select_statement function with the textX object and get the corresponding dictionary
        result = handle_select_statement(model)

        # Convert the dictionary to a Json string
        json_result = json.dumps(result, indent=4)

        return json_result

    except textx.exceptions.TextXSyntaxError as e:
        # If the syntax is incorrect, return the error and its location

        error = f"Syntax Error line {e.line}, row {e.col}: {e.message} at : {query[e.col - 1:]}"

        return error


# If the syntax is correct, this function is called to create a dictionary with the elements of the query
def handle_select_statement(model):
    # Create the dict structure in which the elements of the query will be stored
    result = {
        "table_name": [],
        "columns": [],
        "where_clause": {
            "conditions": [],
            "linkers": []
        }
    }

    # TABLES
    if model.relations:
        # For every table, add it to the list of tables
        for relation in model.relations.relation:
            table_name = {
                "table_name": relation.relationName.upper(),
                "use_name_table": ""
            }

            # If the table is renamed
            if relation.alias:
                table_name["use_name_table"] = relation.alias
            else:
                table_name["use_name_table"] = table_name["table_name"]

            result["table_name"].append(table_name)

    # ATTRIBUTES
    if model.attributes:
        # If the attribute is a "*"
        if model.attributes.attribute == ['*']:
            columns = {
                "use_name_table": "",
                "attribute_name": "*",
                "use_name_attribute": ""
            }

            # If the table is specified
            if model.attributes.table:
                columns["use_name_table"] = model.attributes.table.upper()

            # If the '*' attribute is renamed with AS
            if model.attributes.alias:
                columns["use_name_attribute"] = model.attributes.alias
            else:
                columns["use_name_attribute"] = "*"

            result["columns"].append(columns)

        else:
            # If there is a list of attributes
            for attribute in model.attributes.attribute:

                columns = {
                    "use_name_table": "",
                    "attribute_name": "",
                    "use_name_attribute": ""
                }

                # If the attribute is an aggregate function
                if attribute.aggregate:
                    columns["attribute_name"] = attribute.aggregate.aggregateName + ',' + attribute.aggregate.attributeName.upper()

                    # If the table is specified
                    if attribute.aggregate.table:
                        columns["use_name_table"] = attribute.aggregate.table

                    # If the attribute is renamed with AS
                    if attribute.alias:
                        columns["use_name_attribute"] = attribute.alias
                    else:
                        columns["use_name_attribute"] = attribute.aggregate.aggregateName + '(' + attribute.aggregate.attributeName.upper() + ')'

                # If the attribute is a regular attribute
                else:
                    columns["attribute_name"] = attribute.attributeName.upper()

                    # If the table is specified
                    if attribute.table:
                        columns["use_name_table"] = attribute.table

                    # If the attribute is renamed with AS
                    if attribute.alias:
                        columns["use_name_attribute"] = attribute.alias
                    else:
                        columns["use_name_attribute"] = columns["attribute_name"]

                result["columns"].append(columns)

                # if attribute.distinctOption == "DISTINCT":
                #    result["conditions"] = "distinct" + columns["use_name_attribute"]

    # CONDITIONS
    if model.whereClause:
        # Call the handle_conditions function
        result["where_clause"] = handle_conditions(model.whereClause.conditions)

    # Return dictionary structure
    return result


def handle_conditions(condition_list_path):
    conditions = {
        "conditions": [],
        "linkers": []
    }

    # The first condition in a ConditionList, it has no linker attached to it
    if condition_list_path.in_condition:
        conditions["conditions"].append(handle_in_condition(condition_list_path.in_condition))
    elif condition_list_path.condition:
        cond = condition_list_path.condition

        # If it is a condition list between brackets, call handle_conditions again
        if cond.prioritised_conditions:
            conditions["conditions"].append(handle_conditions(cond.prioritised_conditions))
        else:
            conditions["conditions"].append(handle_single_condition(cond))

    # Linkers list is empty for single condition

    # Following conditions
    if condition_list_path.linked_condition:
        for cond in condition_list_path.linked_condition:
            # Add the linker to the list
            conditions["linkers"].append(cond.linker)

            if cond.in_condition:
                conditions["conditions"].append(handle_in_condition(cond.in_condition))
            # If it is a condition list between brackets, call handle_conditions again
            elif cond.condition.prioritised_conditions:
                conditions["conditions"].append(handle_conditions(cond.condition.prioritised_conditions))
            # Else, call single_condition
            else:
                conditions["conditions"].append(handle_single_condition(cond.condition))
    return conditions


def handle_single_condition(single_condition_path):
    cond = single_condition_path
    struct_condition = {
        "left": None,
        "op": str(cond.op),
        "right": None,
    }

    # LEFT
    # If the left side is a subquery, call handle_select_statement with the textX object that is the subquery
    if cond.leftSubquery:
        struct_condition["left"] = handle_select_statement(cond.leftSubquery.subquery)
    else:
        # If it's a static value
        if cond.left:
            struct_condition["left"] = str(cond.left)

        else:
            # If it's an attribute and the table is specified
            if cond.leftAttribute.table:
                struct_condition["left"] = cond.leftAttribute.table + '.' + cond.leftAttribute.attr.upper()
            else:
                struct_condition["left"] = cond.leftAttribute.attr.upper()

    # RIGHT
    # If the right side is a subquery, call handle_select_statement with the textX object that is the subquery
    if cond.rightSubquery:
        struct_condition["right"] = handle_select_statement(cond.rightSubquery.subquery)

    else:
        # If it's a static value
        if cond.right:
            struct_condition["right"] = str(cond.right)

        else:
            # If it's an attribute and the table is specified
            if cond.rightAttribute.table:
                struct_condition["right"] = cond.rightAttribute.table + '.' + cond.rightAttribute.attr.upper()
            else:
                struct_condition["right"] = cond.rightAttribute.attr.upper()
    return struct_condition


def handle_in_condition(in_condition_path):
    cond = in_condition_path
    struct_condition = {
        "left": cond.leftAttribute.attr.upper(),
        "op": 'IN',
        "right": [],
    }

    if cond.leftAttribute.table:
        struct_condition["left"] = cond.leftAttribute.table + '.' + cond.leftAttribute.attr.upper()

    if cond.notOption:
        struct_condition["op"] = 'NOT ' + struct_condition["op"]

    for right in cond.right:
        struct_condition["right"].append(right)

    return struct_condition



def ldd_parser(query):
        """
        this function handles the syntax of LDD requests
        it takes a string as an input and use a textx grammar to verify syntax and extract fields of interest
        
        """
        sql_meta = textx.metamodel_from_file("syntaxic_parser/textX_grammar/textx_for_LDD.tx", ignore_case = True)
        try:
            # Analyse SQL query
            model = sql_meta.model_from_str(query)
            # If the syntax is correct, create the dict structure in which the elements of the query will be stored
            result = {
                "table_name": [],
                "columns": [],
                "status": True,
                "error": ""
            }
            print("result initialized")
            # Handle CREATE TABLE statement
            if model.__class__.__name__ == "CreateStatement":
                print("we enter the create statement")
                table_name = model.tableName
                columns = []
                print("we define the table name and columns array")
                for column_def in model.columnDefinitions.columnDefinition:
                    #print(vars(column_def.columnConstraints))
                    #print(column_def.columnConstraints.columnConstraint)
                    column = {
                        "name": column_def.columnName,
                        "datatype": query[column_def.columnType._tx_position:column_def.columnType._tx_position_end],
                        "constraints": [constraint for constraint in column_def.columnConstraints.columnConstraint] if column_def.columnConstraints else []
                    }
                    print("we define the column : ", column)
                    columns.append(column)
                result["table_name"].append({"table_name": table_name})
                result["columns"] = columns
                result["action"] = "create"
            else:
                print("we enter the insert statement")
                table_name = model.tableName
                print("we define the table name :", table_name)
                columns = [column.upper() for column in model.columnNames.columnName]
                print("we define the columns :", columns)
                values = [value for value in model.values.value]
                print("we define the values :", values)

                result["table_name"].append({"table_name": table_name})
                result["columns"] = [{"name": column,"data": [str(value)], "constraints" : [""], "datatype" : ""} for column,value in zip(columns,values)]
                result["conditions"] = "NULL"
                result["action"] = "insert"
            json_result = json.dumps(result, indent=4)
            return json_result

        except textx.exceptions.TextXSyntaxError as e:
            # If textx doesn't recognize the syntax
            error = f"Syntax Error line {e.line}, row {e.col}: {e.message}"
            return error


def is_valid_sql(query):
    #if the first word of the query is SELECT,we use the lmd_parser function
    if query[0:6].upper() == "SELECT":
        return lmd_parser(query)
    else:
        print("it is an LDD request")
        return ldd_parser(query)




if __name__ == "__main__":
    print(is_valid_sql("CREATE TABLE communal (population INT PRIMARY KEY, superficie INT, duree_de_vie VARCHAR(255));"))
    print(is_valid_sql("INSERT INTO communal (population, superficie, duree_de_vie) VALUES (1000, 2000, 'longue');"))
    #INSERT INTO PERSONNE ("ID", "NOM", "PRENOM", "AGE") VALUES (15,"dd","ddg",999);
    #INSERT INTO personne (id, nom, prenom, age) VALUES (100, 'fff', 'longue',555);