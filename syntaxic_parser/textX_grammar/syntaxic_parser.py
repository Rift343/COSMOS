import textx
import json
from pprint import pprint
def lbd_parser(query):
        print("we enter the lbd_parser")
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

            error = f"Syntax Error line {e.line}, row {e.col}: {e.message}"

            return error


def ldd_parser(query):
        print("test")
        sql_meta = textx.metamodel_from_file("./textx_for_LDD.tx", ignore_case = True)
        try:
            print("enter try")
            # Analyse SQL query
            model = sql_meta.model_from_str(query)
            print("model good")
            # If the syntax is correct, create the dict structure in which the elements of the query will be stored
            result = {
                "table_name": [],
                "columns": [],
                "status": True,
                "error": ""
            }
            print("result initialized")
            """
{
    "table_name": [
        {
            "table_name": "nom table créé ou inséré"
        }
    ],
    "columns": [#pour chaque colonne dans le insert into ou table créée
        {
            "attribute_name": "*",
            "data" : donné a inséré si utile,
            "datatype" : "type of data",
            "constraint" : contraite a metter du la colonne en cas de création
        }
    ],
    "action" :  "create" | "insert",
    "conditions": "NULL",
    "status": true,
    "error": "NULL"
}
"""

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
                print("we define the result")
                result["table_name"].append({"table_name": table_name})
                result["columns"] = columns
                result["action"] = "create"
            else:
                print("we enter the insert statement")
                table_name = model.tableName
                print("we define the table name :", table_name)
                columns = [column for column in model.columnNames.columnName]
                print("we define the columns :", columns)
                values = [value for value in model.values.value]
                print("we define the values :", values)

                result["table_name"].append({"table_name": table_name})
                result["columns"] = [{"attribute_name": column,"data": value} for column,value in zip(columns,values)]
                result["conditions"] = "NULL"
                result["action"] = "insert"
                print("we define the result")
            json_result = json.dumps(result, indent=4)
            return json_result

        except textx.exceptions.TextXSyntaxError as e:
            # If textx doesn't recognize the syntax
            error = f"Syntax Error line {e.line}, row {e.col}: {e.message}"
            return error


def is_valid_sql(query):
    # Get grammar file
    print("we enter the lbd_parser")
    # we check query length
    #if the first word of the query is SELECT,we use the lbd_parser function
    print(type(query))
    if len(query) < 6:
        return "Query is too short"
    elif query[0:6].upper() == "SELECT":
        return lbd_parser(query)
    else:
        print("we go good way")
        return ldd_parser(query)




if __name__ == "__main__":
    print(is_valid_sql("CREATE TABLE communal (population INT PRIMARY KEY, superficie INT, duree_de_vie VARCHAR(255));"))
    print(is_valid_sql("INSERT INTO communal (population, superficie, duree_de_vie) VALUES (1000, 2000, 'longue');"))