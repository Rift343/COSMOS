from flask import Flask, request, jsonify, send_file
from flask_cors import CORS
import socket

app = Flask(__name__)
CORS(app) #On inclus ça car de base les protocoles http n'autorisent plus les requêtes cross-origin (Same Origin Policy)


def send_query(server_socket, query):
    # Convert query to bytes
    query_bytes = query.encode('utf-8')
    query_len = len(query_bytes).to_bytes(4, byteorder='big')  # Length of the query as 4 bytes in big-endian order

    # Send the length of the query to the server
    server_socket.sendall(query_len)

    # Write the query to the server
    server_socket.sendall(query_bytes)

def receive_response(server_socket):
    # Read the response length from the server
    len_bytes = server_socket.recv(4)
    response_len = int.from_bytes(len_bytes, byteorder='big')

    # Receive the response from the server
    response_bytes = b''
    while len(response_bytes) < response_len:
        chunk = server_socket.recv(response_len - len(response_bytes))
        if not chunk:
            raise ConnectionError("Connection to server closed unexpectedly")
        response_bytes += chunk

    # Convert response bytes to String
    response = response_bytes.decode('utf-8')
    return response


@app.route('/')
def serve_html():
    return send_file('index.html')

@app.route('/submit', methods=['POST'])
def handle_message():
    data = request.get_json()
    message = data['message']
    print("Message reçu:", message)
    
    # Connexion au serveur Rust
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
        server_socket.connect(('127.0.0.1', 8880))

        # Envoi de la requête au serveur Rust
        send_query(server_socket, message)

        # Réception de la réponse du serveur Rust
        response = receive_response(server_socket)

    return jsonify({"response": response})


if __name__ == '__main__':
    app.run(host= '51.75.26.110', port=80, debug=True)
