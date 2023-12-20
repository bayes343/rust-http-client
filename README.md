# Rust HTTP Client

Rust based HTTP client.

## Feature Plan

- Library Features
  - [x] Support HTTP and HTTPS protocols
  - [x] Abstract Requests `src/request.rs`
  - [x] Public function for getting a response from a request `src/fetch.rs`
  - [x] Abstract Responses
  - [ ] ...
- CLI Features
  - [x] Support for making requests through command line arguments only
  - [x] Support for making requests through pointing to an http file
  - [ ] Support for saving the response to a file
  - [ ] ...

## Local Setup

- Follow Rust installation instructions at https://doc.rust-lang.org/stable/book/ch01-01-installation.html
- Build / run the project with `cargo build` and `cargo run` respectively.
  - See below examples for appropriate usage.

## Programmatic Example

```rust
use http::{
    fetch::fetch,
    protocol::Protocol,
    request::Request,
    methods::Methods
};

let response = fetch(&Request{
    method: Methods::GET,
    protocol: Protocol::Https,
    host: String::from("api.chucknorris.io"),
    path: String::from("/jokes/random"),
    headers: vec![],
    body: None
}).unwrap();

println!("{} {}", response.status, response.status_text);
if let Some(b) = response.body {
    println!("{}", b)
}
```

The above would print something like...
```sh
200 OK
{"categories": [],"created_at": "2020-01-05 13:42:28.664997","icon_url": "https://assets.chucknorris.host/img/avatar/chuck-norris.png","id": "9FqrXimEQ2-XoPbAx-UcBA","updated_at": "2020-01-05 13:42:28.664997","url": "https://api.chucknorris.io/jokes/9FqrXimEQ2-XoPbAx-UcBA","value": "Adobe Flash runs on Chuck Norris' iPhone."}
```

## CLI Examples

### Local Dev

- `cargo run GET https://api.chucknorris.io/jokes/random`
- `cargo run GET api.chucknorris.io/jokes/random "Content-Type: application/json"` each individual header should be wrapped in double quotes
- `cargo run POST api.chucknorris.io/jokes/random "Content-Type: application/json" "{"id": "test"}"` the first arg not containing ": " after the uri will be used as the request body
- Given a file like the below you can make requests by passing a filepath `cargo run ./example.http`

```sh
GET https://api.chucknorris.io/jokes/random
Some-Header: value

{
  "id": "test"
}
```

### Release Build

- Just replace `cargo run` with `{PATH_TO_EXE}/http`
