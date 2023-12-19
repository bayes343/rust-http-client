# Rust HTTP Client

Experimental Rust based HTTP client.

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
  - See below examples for appropriate use of args.

## CLI Examples

### Local Dev

- `cargo run GET https://somewebsite.com`
- `cargo run GET somewebsite.com "Content-Type: application/json"` each individual header should be wrapped in double quotes
- `cargo run POST somewebsite.com "Content-Type: application/json" "{"id": "te"}"` the first arg not containing ": " after the uri will be used as the request body
- Given a file like the below you can make requests by passing a filepath `cargo run ./example.http`

```sh
GET https://somewebsite.com
Some-Header: value

{
  "id": "test"
}
```

### Release Build

- Just replace `cargo run` with `{PATH_TO_EXE}/http`
