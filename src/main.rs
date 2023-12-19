use std::env;
use std::fs;
use std::io::Error;

use http::{
    fetch::fetch,
    protocol::Protocol,
    request::Request,
    methods::Methods
};

fn main() {
    let request = get_request_from_args().unwrap();
    let response = fetch(&request);
    println!("{response}");
}

fn get_request_from_args() -> Result<Request, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let method_string = &args[1];
        let url = &args[2];
        let protocol_string = if url.starts_with("http://") { "http" } else { "https" };
        let url_without_protocol = url.replace(format!("{protocol_string}://").as_str(), "");
        let host_and_path: Vec<&str> = url_without_protocol.split('/').collect();
        let remaining_args = if args.len() > 3 { args[3..].to_vec() } else { vec![] };
        let body_candidates: Vec<String> = remaining_args.clone().into_iter().filter(|e| !e.contains(": ")).collect();

        Ok(Request {
            protocol: if protocol_string == "http" { Protocol::Http } else { Protocol::Https },
            method: get_method_from_method_string(&method_string),
            host: String::from(host_and_path[0]),
            path: String::from(
                if host_and_path.len() > 1 { format!("/{}", host_and_path[1..].join("/")) } else { String::from("/") }
            ),
            headers: remaining_args.into_iter().filter(|e| e.contains(": ")).collect(),
            body: body_candidates.first().cloned()
        })
    } else if args.len() == 2 {
        get_request_from_file(&args[1])
    } else {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "You must submit a file or an HTTP Method (ex. GET) followed by a URI (ex. somewebsite.com)."))
    }
}

fn get_method_from_method_string(method_string: &str) -> Methods {
    match method_string {
        _ => {
            let methods: Vec<Methods> = vec![Methods::GET, Methods::POST, Methods::PATCH, Methods::PUT, Methods::DELETE];
            let matching_method = methods.iter().find(|&e| e.to_string() == method_string);

            if let Some(m) = matching_method {
                m.clone()
            } else {
                println!("Unknown method: \"{method_string}\" - defaulting to \"GET\"");
                Methods::GET
            }
        }
    }
}

fn get_request_from_file(file_path: &str) -> Result<Request, Error> {
    let file_contents = fs::read_to_string(file_path).expect("Unable to read given file.");
    println!("{:?}", file_contents);
    let lines: Vec<&str> = file_contents.split('\n').filter(|&e| e.len() > 0 ).collect();

    if lines.len() > 0 {
        let method_url: Vec<&str> = lines[0].split(' ').collect();
        let method = get_method_from_method_string(method_url[0]);
        let protocol_string = if method_url[1].starts_with("http://") { "http" } else { "https" };
        let url_without_protocol = method_url[1].replace(format!("{protocol_string}://").as_str(), "");
        let host_and_path: Vec<&str> = url_without_protocol.split('/').collect();
        let remaining_args = if lines.len() > 1 { lines[1..].to_vec() } else { vec![] };
        let body_candidates: Vec<&str> = remaining_args.clone().into_iter().filter(|&e| !e.contains(": ")).collect();
        let headers: Vec<&str> = remaining_args.into_iter().filter(|&e| e.contains(": ")).collect();

        Ok(Request {
            method,
            protocol: if protocol_string == "http" { Protocol::Http } else { Protocol::Https },
            host: String::from(host_and_path[0]),
            path: String::from(
                if host_and_path.len() > 1 { format!("/{}", host_and_path[1..].join("/")) } else { String::from("/") }
            ),
            headers: headers.iter().map(|&f| f.into()).collect(),
            body: if let Some(&s) = body_candidates.first() { Some(s.to_string()) } else { None }
        })
    } else {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "You must submit a file or an HTTP Method (ex. GET) followed by a URI (ex. somewebsite.com)."))
    }
}
