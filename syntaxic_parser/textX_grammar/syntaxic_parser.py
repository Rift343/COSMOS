import textx
import json

def is_valid_sql(query):
    # Get grammar file
    sql_meta = textx.metamodel_from_file("syntaxic_parser/textX_grammar/grammar_file.tx", ignore_case = True)
    try:
        # Analyse de la requête SQL
        model = sql_meta.model_from_str(query)

        # Si la requête est bien formée, sauvegardez les données dans un dictionnaire
        result = {
            "table_name": [],
            "columns": [],
            "conditions": "",
            "status": True,
            "error": ""
        }

        if model.attributes:
            #print("Les Attributs = ",model.attributes.attribute)
            if model.attributes.attribute==['*']:
                result["columns"].append(["","*"])
            else:
                for attr in model.attributes.attribute:
                    result["columns"].append(["",attr.attributeName])


        if model.relations:
            #print("Les relations = ",model.relations.relation)
            for relation in model.relations.relation:
                result["table_name"].append(relation)
                for col in result["columns"]:
                    col[0] = relation


        result["conditions"] = "NULL"
        result["error"] = "NULL"

        # Conversion du dictionnaire en Json
        json_result = json.dumps(result, indent=4)

        # Affichage du JSON
        #print(json_result)
        return json_result

    except textx.exceptions.TextXSyntaxError as e:
        # En cas d'erreur de syntaxe, retourner une réponse d'erreur au format JSON
        result = {
            "status": False,
            "error": f"Erreur de syntaxe a la ligne {e.line}, colonne {e.col}: {e.message}"
        }

        # Convertissez le dictionnaire en une chaîne JSON
        json_result = json.dumps(result, indent=4)

        # Affichez ou sauvegardez le JSON
        #print(json_result)
        return json_result

#if __name__ == "__main__":
#    query = "SELECT * FROM table1;"
#    is_valid_sql(query)