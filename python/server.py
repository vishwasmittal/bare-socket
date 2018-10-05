# import the socket library
import socket

# A string predefined in the form of HTTP resopnse. (More on HTTP responses later)
server_message = """\
HTTP/1.1 200 OK\r
\r
<HTML>
<Head>
	<Title>Title: Bare Socket</Title>
</Head>

<Body>
	<center>
		<h1>
			Hello from server!
		</h1>
	</center>
</Body>
</HTML>
"""

# define a socket object, set some socket_options, bind it to localhost and start listening
server_sock = socket.socket(family=socket.AF_INET, type=socket.SOCK_STREAM, proto=0)
server_sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
server_sock.bind(('127.0.0.1', 8000))
server_sock.listen(5)

try:
	print('[+] listening on port 8000')
	# listen forever
	while True:
		# accept a connection. Returns a tuple. (sockek, client_address)
		conn, addr = server_sock.accept()
		# receive data sent by client
		client_message = conn.recv(1024)
		print(client_message.decode())
		# send data to client
		conn.sendall(server_message.encode())
		#close the socket after work is completed
		conn.close()
except Exception as e:
	# if any exception occurs, print it and close the server socket
	print(e)
	server_sock.close()
