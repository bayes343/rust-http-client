use std::{io::{Read, Write}, net::TcpStream, sync::Arc};

use crate::request::Request;

pub enum Protocol {
  Http,
  Https
}

impl Protocol {
  pub fn to_port_string(&self) -> &str {
    match self {
      Self::Http => "80",
      Self::Https => "443"
    }
  }

  pub fn connect(&self, request: &Request) -> String {
    match self {
      Self::Http => {
        let request_data = request.get_request_data();
        let mut stream = TcpStream::connect(format!("{}:{}", request.host, request.protocol.to_port_string())).unwrap();
        let _ = stream.write_all(request_data.as_bytes());

        let mut response_buffer = String::new();
        let _ = stream.read_to_string(&mut response_buffer);
        response_buffer
      },
      Self::Https => {
        let request_data = request.get_request_data();
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
        let _ = client.writer().write_all(request_data.as_bytes());
        let mut stream = rustls::Stream::new(&mut client, &mut socket);

        let mut response_buffer = String::new();
        let _ = stream.read_to_string(&mut response_buffer);
        response_buffer
      }
    }
  }
}
