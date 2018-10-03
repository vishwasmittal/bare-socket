# Bare-Socket
Simple server and corresponding client written in bare sockets in multiple languages.

## Requirements
### Server
- The server should be written in bare sockets and use of external libraries should be avoided.
- The server should be able to accept a client request and be able to serve a simple HTTP response. Example:
```html
HTTP/1.1 200 OK

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
```

### Client
- Client should be written in bare sockets and use of external libraries should be avoided.
- Client should be able to connect to the server, send a request message and print the response received.

## Testing and Usage
Servers returns the HTTP responses so these can also be tested and/or used using the browsers also.

## Contributions
Contributions and other suggestions are welcome. Submit a pull request.