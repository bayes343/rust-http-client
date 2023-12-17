use std::env;
use std::net::TcpStream;
use std::io::{
    Error,
    Read,
    Write
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url_parts: Vec<&str> = args[1].split('/').collect();
        let host = url_parts[0];
        let path = if url_parts.len() > 1 { format!("/{}", url_parts[1..].join("/")) } else { String::from("/") };
        println!("Connecting to host: {host}");
        println!("Path: {path}");

        let mut stream = TcpStream::connect(format!("{host}:80")).unwrap();

        let mut request_data = String::new();
        request_data.push_str(format!("GET {path} HTTP/1.0").as_str());
        request_data.push_str("\r\n");
        request_data.push_str(format!("Host: {host}").as_str());
        request_data.push_str("\r\n");
        request_data.push_str("Connection: close");
        request_data.push_str("\r\n");
        request_data.push_str("\r\n");

        stream.write_all(request_data.as_bytes())?;

        let mut buf = String::new();
        let result = stream.read_to_string(&mut buf)?;
        println!("result = {}", result);
        println!("buf = {}", buf);

        Ok(())
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "You must enter a URI."))
    }
}
