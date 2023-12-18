use std::env;
use std::io::{Error, Read, Write};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let request = get_request_from_args().unwrap();

    println!("Connecting to host: {}", request.host);
    println!("Path: {}", request.path);

    let mut root_store = rustls::RootCertStore::empty();
    root_store.extend(
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
    );

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let rc_config = Arc::new(config);
    let host_name = request.host.clone().try_into().unwrap();
    let mut client = rustls::ClientConnection::new(rc_config, host_name).unwrap();
    let mut socket = std::net::TcpStream::connect(format!("{}:443", request.host)).unwrap();

    let mut request_data = String::new();
    request_data.push_str(format!("GET {} HTTP/1.0", request.path).as_str());
    request_data.push_str("\r\n");
    request_data.push_str(format!("Host: {}", request.host).as_str());
    request_data.push_str("\r\n");
    request_data.push_str("Connection: close");
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");

    client.writer().write_all(request_data.as_bytes())?;

    let mut stream = rustls::Stream::new(&mut client, &mut socket);

    let mut buf = String::new();
    let result = stream.read_to_string(&mut buf)?;
    println!("result = {}", result);
    println!("buf = {}", buf);

    Ok(())
}

struct Request {
    host: String,
    path: String
}

fn get_request_from_args() -> Result<Request, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url_parts: Vec<&str> = args[1].split('/').collect();
        Ok(Request {
            host: String::from(url_parts[0]),
            path: String::from(
                if url_parts.len() > 1 { format!("/{}", url_parts[1..].join("/")) } else { String::from("/") }
            ),
        })
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "You must enter a URI."))
    }
}
