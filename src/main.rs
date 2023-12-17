use std::env;
use std::io::Error;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url_parts: Vec<&str> = args[1].split('/').collect();
        let host = url_parts[0];
        let path = if url_parts.len() > 1 { format!("/{}", url_parts[1..].join("/")) } else { String::from("") };
        println!("Connecting to host: {host}");
        println!("Path: {path}");
        let stream = TcpStream::connect(format!("{host}:80")).unwrap();
        Ok(())
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "You must enter a URI."))
    }
}
