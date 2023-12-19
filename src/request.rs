use crate::{
  protocol::Protocol,
  methods::Methods
};

pub struct Request {
  pub method: Methods,
  pub protocol: Protocol,
  pub host: String,
  pub path: String,
  pub headers: Vec<String>,
  pub body: Option<String>
}

impl Request {
  pub fn get_request_data(&self) -> String {
      let mut request_data = String::new();

      request_data.push_str(format!("{} {} HTTP/1.0", self.method.to_string(), self.path).as_str());
      request_data.push_str("\r\n");
      request_data.push_str(format!("Host: {}", self.host).as_str());
      request_data.push_str("\r\n");
      request_data.push_str("Connection: close\r\n");
      self.headers.clone().into_iter().for_each(|e| request_data.push_str(format!("{}\r\n", e).as_str()));
      if let Some(b) = &self.body {
        request_data.push_str(format!("Content-Length: {}\r\n\r\n", &b.len()).as_str());
        request_data.push_str(&b)
      }
      request_data.push_str("\r\n");
      request_data.push_str("\r\n");

      println!("{:?}", request_data);

      request_data
  }
}
