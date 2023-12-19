# Rust HTTP Client

Rust based HTTP client.

## Feature Plan

- Library Features
  - [x] Support HTTP and HTTPS protocols (HTTPS by default)
  - [x] Abstract Requests `src/request.rs`
  - [x] Public function for getting a response from a request `src/fetch.rs`
  - [ ] Abstract Responses
- CLI Features
  - [x] Support for making requests through command line arguments only
  - [x] Support for making requests through pointing to an http file
  - [ ] Support for saving the response to a file

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
});
println!("{response}");
```

The above would print something like...
```sh
HTTP/1.1 200 OK
Date: Tue, 19 Dec 2023 04:29:13 GMT
Content-Type: application/json;charset=UTF-8
Connection: close
Report-To: {"group":"heroku-nel","max_age":3600,"endpoints":[{"url":"https://nel.heroku.com/reports?ts=1702960152&sid=812dcc77-0bd0-43b1-a5f1-b25750382959&s=cMYo8nkSow5XyEmHGirtZ9Fmn3yp3adfn7Wx8bearKc%3D"}]}
Reporting-Endpoints: heroku-nel=https://nel.heroku.com/reports?ts=1702960152&sid=812dcc77-0bd0-43b1-a5f1-b25750382959&s=cMYo8nkSow5XyEmHGirtZ9Fmn3yp3adfn7Wx8bearKc%3D
Nel: {"report_to":"heroku-nel","max_age":3600,"success_fraction":0.005,"failure_fraction":0.05,"response_headers":["Via"]}
Via: 1.1 vegur
CF-Cache-Status: DYNAMIC
Server: cloudflare
CF-RAY: 837ce739a9316ac3-RIC
alt-svc: h3=":443"; ma=86400

{
    "categories": [],
    "created_at": "2020-01-05 13:42:28.664997",
    "icon_url": "https://assets.chucknorris.host/img/avatar/chuck-norris.png",
    "id": "9FqrXimEQ2-XoPbAx-UcBA",
    "updated_at": "2020-01-05 13:42:28.664997",
    "url": "https://api.chucknorris.io/jokes/9FqrXimEQ2-XoPbAx-UcBA",
    "value": "Adobe Flash runs on Chuck Norris' iPhone."
}
```

## CLI Examples

### Local Dev

- `cargo run GET https://api.chucknorris.io`
- `cargo run GET api.chucknorris.io "Content-Type: application/json"` each individual header should be wrapped in double quotes
- `cargo run POST api.chucknorris.io "Content-Type: application/json" "{"id": "test"}"` the first arg not containing ": " after the uri will be used as the request body
- Given a file like the below you can make requests by passing a filepath `cargo run ./example.http`

```sh
GET https://api.chucknorris.io
Some-Header: value

{
  "id": "test"
}
```

### Release Build

- Just replace `cargo run` with `{PATH_TO_EXE}/http`
