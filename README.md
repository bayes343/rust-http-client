# Rust HTTP Client

Experimental Rust based HTTP client.

## Local Setup

TODO

## Example Usage

### Local Dev

- `cargo run GET api.chucknorris.io/jokes/random` default to https protocol
- `cargo run GET https://api.chucknorris.io/jokes/random` explicit use of https protocol
- `cargo run GET http://api.chucknorris.io/jokes/random` explicit use of http protocol
- `cargo run GET api.chucknorris.io/jokes/random "Content-Type: application/json"` adding a custom header - each individual header should be wrapped in double quotes.
