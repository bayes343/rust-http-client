use std::env;
use std::io::{Error, Read, Write};
use std::sync::Arc;


fn main() -> std::io::Result<()> {
    static mut args: Vec<String> = vec![];
    unsafe {
        args = env::args().collect();
        if args.len() > 1 {
            let url_parts: Vec<&str> = args[1].split('/').collect();
            let host = url_parts[0];
            let path = if url_parts.len() > 1 { format!("/{}", url_parts[1..].join("/")) } else { String::from("/") };
            println!("Connecting to host: {host}");
            println!("Path: {path}");

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
            let example_com = host.try_into().unwrap();
            let mut client = rustls::ClientConnection::new(rc_config, example_com).unwrap();
            let mut socket = std::net::TcpStream::connect(format!("{host}:443")).unwrap();

            let mut request_data = String::new();
            request_data.push_str(format!("GET {path} HTTP/1.0").as_str());
            request_data.push_str("\r\n");
            request_data.push_str(format!("Host: {host}").as_str());
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
        } else {
            Err(Error::new(std::io::ErrorKind::InvalidData, "You must enter a URI."))
        }
    }
}
