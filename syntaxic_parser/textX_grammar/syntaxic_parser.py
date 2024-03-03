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

        if model.attributes:
            # If the attribute is a "*"
            if model.attributes.attribute==['*']:
                result["columns"].append(["","*"])
            else:
                # If there is a list of attributes
                for attr in model.attributes.attribute:
                    result["columns"].append(["",attr.attributeName])


        if model.relations:
            # for every table, add it to the list of tables demanded, and as the first item of each "columns" list
            for relation in model.relations.relation:
                result["table_name"].append(relation)
                for col in result["columns"]:
                    col[0] = relation

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