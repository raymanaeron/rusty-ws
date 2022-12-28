# What is this project about?
This project is a web service in RUST that can handle HTTP GET, POST, and PUT requests with different types of payloads (query parameters, JSON body). 

I have also implemented a Middleware implementation that checks for an Authorization header in the request and logs its value. The Middleware implementation is applied to all routes in the server by default. If you want to apply the Middleware implementation only to specific routes, you can use the "with" method on those routes as shown in the code.

# How to test?
Use this online tool https://reqbin.com/. You will likely have to install their chrome browser extension to test your "localhost".

### GET test
```
URL: http://localhost:8080/search?q=Hello
```
Response: 
```
Received GET request with query parameter q = "Hello"
```

### POST test
```
URL: http://localhost:8080/submit
```

Pass in a JSON body, for example:
```
{
  "name": "bob",
  "age": 23
}
```
Response: 
```
Received POST request with name = bob, age = 23
```

### PUT test
```
URL: http://localhost:8080/update
```

Pass in a JSON body, for example:
```
{
  "name": "bob",
  "age": 23
}
```
Response: 
```
Received PUT request with name = bob, age = 23
```