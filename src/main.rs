use std::env;
use std::io::Error;

use http::fetch::fetch;
use http::protocol::Protocol;
use http::request::Request;

fn main() -> std::io::Result<()> {
    let request = get_request_from_args().unwrap();
    println!("Connecting to host: {}", request.host);
    println!("Path: {}", request.path);

    let response_buffer = fetch(&request);

    println!("{response_buffer}");
    Ok(())
}

fn get_request_from_args() -> Result<Request, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let url = &args[1];
        let protocol_string = if url.starts_with("http://") { "http" } else { "https" };
        let url_without_protocol = url.replace(format!("{protocol_string}://").as_str(), "");
        let host_and_path: Vec<&str> = url_without_protocol.split('/').collect();

        Ok(Request {
            protocol: if protocol_string == "http" { Protocol::Http } else { Protocol::Https },
            host: String::from(host_and_path[0]),
            path: String::from(
                if host_and_path.len() > 1 { format!("/{}", host_and_path[1..].join("/")) } else { String::from("/") }
            ),
        })
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "You must enter a URI."))
    }
}
