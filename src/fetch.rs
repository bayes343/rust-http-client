use std::{
  net::TcpStream,
  sync::Arc,
  io::{Write, Read}
};

use crate::{request::Request, protocol::Protocol};

pub fn fetch(request: &Request) -> String {
  match request.protocol {
    Protocol::Http => {
      let request_data = request.get_request_data();
      let mut stream = TcpStream::connect(format!("{}:{}", request.host, request.protocol.to_port_string())).unwrap();
      let _ = stream.write_all(request_data.as_bytes());

      let mut response_buffer = String::new();
      let _ = stream.read_to_string(&mut response_buffer);
      response_buffer
    },
    Protocol::Https => {
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
