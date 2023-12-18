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
}
