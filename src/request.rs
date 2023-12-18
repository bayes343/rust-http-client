use crate::{
  protocol::Protocol,
  methods::Methods
};

pub struct Request {
  pub protocol: Protocol,
  pub host: String,
  pub path: String,
  pub method: Methods
}

impl Request {
  pub fn get_request_data(&self) -> String {
      let mut request_data = String::new();

      request_data.push_str(format!("{} {} HTTP/1.0", self.method.to_string(), self.path).as_str());
      request_data.push_str("\r\n");
      request_data.push_str(format!("Host: {}", self.host).as_str());
      request_data.push_str("\r\n");
      request_data.push_str("Connection: close");
      request_data.push_str("\r\n");
      request_data.push_str("\r\n");

      request_data
  }
}
