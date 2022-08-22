# Eden

Eden is a REST web server primarily designed as an interface for Apple to fetch resources from locally hosted databases.

Why Rust? Because I wanted to learn how to make a web server in Rust!

If you want to run it on your localhost, you can use `curl` to send some request:
```
curl --header "Content-Type: application/json" --request POST --data '{"username":"xyz","password":"xyz"}' http://localhost:8080/db/quote/draw
```
(I am really just making a note of this because this happens to be the `curl` command with arbitrary data that I am using to test the application!)