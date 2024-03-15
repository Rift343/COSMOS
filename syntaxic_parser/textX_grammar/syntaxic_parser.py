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
            "conditions": "",
            "status": True,
            "error": ""
        }

        if model.relations:
            # for every table, add it to the list of tables demanded, and as the first item of each "columns" list
            for relation in model.relations.relation:
                table_name = {
                    "table_name": relation.relationName,
                    "use_name_table": ""
                }

                if relation.alias :
                    table_name["use_name_table"] = relation.alias
                else :
                    table_name["use_name_table"] = relation.relationName

                result["table_name"].append(table_name)



        if model.attributes:

            # If the attribute is a "*"
            if model.attributes.attribute==['*']:
                columns = {
                    "use_name_table": result["table_name"][0]["use_name_table"],
                    "attribute_name": "*",
                    "use_name_attribute": "*"
                }

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
                        columns["use_name_table"] = result["table_name"][0]["use_name_table"]

                        if attribute.aggregate.aggregateName == "COUNT(*)" :
                            columns["attribute_name"] = '*'
                            #columns["attribute_name"] = attribute.aggregate.aggregateName
                        else :
                            columns["attribute_name"] = attribute.aggregate.attributeName
                            #columns["attribute_name"] = attribute.aggregate.aggregateName + '(' + attribute.aggregate.attributeName +')'


                    # if the attribute is a regular attribute
                    else :
                        columns["attribute_name"] = attribute.attributeName


                    # If the table the attribute belongs to is specified
                    if attribute.table :
                        columns["use_name_table"] = attribute.table
                    else :
                        columns["use_name_table"] = result["table_name"][0]["use_name_table"]

                    # If the attribute is renamed with AS
                    if attribute.alias :
                        columns["use_name_attribute"] = attribute.alias
                    else :
                        columns["use_name_attribute"] = columns["attribute_name"]


                    result["columns"].append(columns)

                    #if attribute.distinctOption == "DISTINCT":
                    #    result["conditions"] = "distinct" + columns["use_name_attribute"]


        # Conditions are not handled for the time being
        result["conditions"] = "NULL"

        # "error" field is null
        result["error"] = "NULL"

        # Convert the dict to Json string
        json_result = json.dumps(result, indent=4)


        return json_result

    except textx.exceptions.TextXSyntaxError as e:
        # If the syntax is incorrect, fill in the "status" and "error" fields accordingly
        result = {
            "status": False,
            "error": f"Syntax Error line {e.line}, row {e.col}: {e.message}"
        }

        # Convert the dict to Json string
        json_result = json.dumps(result, indent=4)


        return json_result