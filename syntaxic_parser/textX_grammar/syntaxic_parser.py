import textx
import json

def is_valid_sql(query):
    # Get grammar file
    sql_meta = textx.metamodel_from_file("syntaxic_parser/textX_grammar/grammar_file.tx", ignore_case = True)
    try:
        # Analyse SQL query
        model = sql_meta.model_from_str(query)

        # If the syntax is correct, create the dict structure in which the elements of the query will be stored
        result = {
            "table_name": [],
            "columns": [],
            "conditions": []
        }

        if model.relations:
            # For every table, add it to the list of tables
            for relation in model.relations.relation:
                table_name = {
                    "table_name": relation.relationName.upper(),
                    "use_name_table": ""
                }

                # If the table is renamed
                if relation.alias :
                    table_name["use_name_table"] = relation.alias
                else :
                    table_name["use_name_table"] = table_name["table_name"]

                result["table_name"].append(table_name)



        if model.attributes:

            # If the attribute is a "*"
            if model.attributes.attribute==['*']:
                columns = {
                    "use_name_table": "",
                    "attribute_name": "*",
                    "use_name_attribute": ""
                }

                # If the table is specified
                if model.attributes.table :
                    columns["use_name_table"] = model.attributes.table.upper()

                # If the '*' attribute is renamed with AS
                if model.attributes.alias :
                    columns["use_name_attribute"] = model.attributes.alias

                result["columns"].append(columns)

            else:
                # If there is a list of attributes
                for attribute in model.attributes.attribute:

                    columns = {
                        "use_name_table": "",
                        "attribute_name": "",
                        "use_name_attribute": ""
                    }

                    # if the attribute is an aggregate function
                    if attribute.aggregate :
                        # If the table is specified
                        if attribute.aggregate.table :
                            columns["use_name_table"] = attribute.aggregate.table


                        columns["attribute_name"] = attribute.aggregate.aggregateName + ',' + attribute.aggregate.attributeName.upper()


                        # If the attribute is renamed with AS
                        if attribute.alias :
                            columns["use_name_attribute"] = attribute.alias
                        else :
                            columns["use_name_attribute"] = attribute.aggregate.aggregateName + '(' + attribute.aggregate.attributeName.upper() + ')'


                    # If the attribute is a regular attribute
                    else :
                        columns["attribute_name"] = attribute.attributeName.upper()


                        # If the table the attribute belongs to is specified
                        if attribute.table :
                            columns["use_name_table"] = attribute.table.upper()


                        # If the attribute is renamed with AS
                        if attribute.alias :
                            columns["use_name_attribute"] = attribute.alias
                        else :
                            columns["use_name_attribute"] = columns["attribute_name"]


                    result["columns"].append(columns)

                    #if attribute.distinctOption == "DISTINCT":
                    #    result["conditions"] = "distinct" + columns["use_name_attribute"]


        # Conditions
        if model.whereClause :
            # If there is only one condition
            cond = model.whereClause.conditions.condition

            structCondition = {
                "left" : cond.left,
                "op" : str(cond.op),
                "right" : str(cond.right),
                "linker" : 'AND'
            }

            result["conditions"].append(structCondition)

            # If there are several
            if model.whereClause.conditions.linked_condition :
                for condition in model.whereClause.conditions.linked_condition :
                    structCondition = {
                        "left": condition.left,
                        "op": str(condition.op),
                        "right": str(condition.right),
                        "linker": condition.linker
                    }

                    result["conditions"].append(structCondition)



        # Convert the dict to Json string
        json_result = json.dumps(result, indent=4)


        return json_result

    except textx.exceptions.TextXSyntaxError as e:
        # If the syntax is incorrect, return the error and its location

        error = f"Syntax Error line {e.line}, row {e.col}: {e.message}"

        return error