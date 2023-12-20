use std::{
  collections::HashMap,
  io::Error
};

pub struct Response {
  pub status: usize,
  pub status_text: String,
  pub headers: HashMap<String, String>,
  pub body: Option<String>,
  pub raw: String
}

impl Response {
  pub fn from(raw_response: String) -> Result<Response, Error> {
    let lines: Vec<&str> = raw_response.split('\n').collect();
    if lines.len() >= 1 {
      let version_status_text: Vec<&str> = lines[0].split(' ').collect();
      let remaining_lines: Vec<&str> = lines[1..].to_vec();
      let mut headers = HashMap::new();
      let mut body: Option<String> = None;

      let mut newline_hit = false;
      for line in remaining_lines {
        if line.trim().len() > 0 {
          if newline_hit {
            body = Some(line.to_string());
          } else {
            let key_value: Vec<&str> = line.split(": ").collect();
            headers.insert(key_value[0].to_string(), key_value[1].to_string());
          }
        } else {
          newline_hit = true;
        }
      }

      Ok(Response{
        status: version_status_text[1].parse::<usize>().unwrap(),
        status_text: version_status_text[2].to_string(),
        headers,
        body,
        raw: raw_response.to_string(),
      })
    } else {
      Err(Error::new(
        std::io::ErrorKind::InvalidData,
        "Response was empty"))
    }
  }
}
