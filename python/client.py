# import the socket library
import socket
client_message = """GET / HTTP/1.1\r
Host: localhost:8000\r
Accept: */*\r
\r
"""
# create a client socket
client_sock = socket.socket(family=socket.AF_INET, type=socket.SOCK_STREAM, proto=0)
# connect to server listening on port 8000
client_sock.connect(('127.0.0.1', 8000))
# send (request) data to server
client_sock.send(client_message.encode())
# receive data from server
server_message = client_sock.recv(1024)
# print received message
print(server_message.decode())
# close the client socket
client_sock.close()
