# Rust HTTP Client

Experimental Rust based HTTP client.

## Local Setup

- Follow Rust installation instructions at https://doc.rust-lang.org/stable/book/ch01-01-installation.html
- Build / run the project with `cargo build` and `cargo run` respectively.
  - See below examples for appropriate use of args.

## CLI Examples

### Local Dev

- `cargo run GET api.chucknorris.io/jokes/random` default to https protocol
- `cargo run GET https://api.chucknorris.io/jokes/random` explicit use of https protocol
- `cargo run GET http://api.chucknorris.io/jokes/random` explicit use of http protocol
- `cargo run GET api.chucknorris.io/jokes/random "Content-Type: application/json"` adding a custom header - each individual header should be wrapped in double quotes
- `cargo run POST api.chucknorris.io/jokes/random "Content-Type: application/json" "{"fake": "example"}"` the first arg not containing ": " after the uri will be used as the request body
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
