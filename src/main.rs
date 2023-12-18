use std::env;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use http::protocol::Protocol;
use http::request::Request;

fn main() -> std::io::Result<()> {
    let request = get_request_from_args().unwrap();
    println!("Connecting to host: {}", request.host);
    println!("Path: {}", request.path);

    let request_data = request.get_request_data();
    let mut response_buffer = String::new();

    match request.protocol {
        Protocol::Https => {
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
            let mut socket = std::net::TcpStream::connect(format!("{}:{}", request.host, request.protocol.to_port_string())).unwrap();
            client.writer().write_all(request_data.as_bytes())?;
            let mut stream = rustls::Stream::new(&mut client, &mut socket);
            stream.read_to_string(&mut response_buffer)?;
        },
        Protocol::Http => {
            let mut stream = TcpStream::connect(format!("{}:{}", request.host, request.protocol.to_port_string())).unwrap();
            stream.write_all(request_data.as_bytes())?;
            stream.read_to_string(&mut response_buffer)?;
        }
    }

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
