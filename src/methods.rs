#[derive(Clone)]
pub enum Methods {
  POST,
  GET,
  PATCH,
  PUT,
  DELETE
}

impl Methods {
  pub fn to_string(&self) -> &str {
    match self {
      Methods::POST => "POST",
      Methods::GET => "GET",
      Methods::PATCH => "PATCH",
      Methods::PUT => "PUT",
      Methods::DELETE => "DELETE"
    }
  }
}
