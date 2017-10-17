use base64::decode;
use std::str::{self, FromStr};

error_chain! {
  foreign_links {
    Base64Decode(::base64::DecodeError);
    Utf8(::std::str::Utf8Error);
  }
  errors {
    InvalidAuthorizationHeader(value: String) {
      description("invalid authorization header")
      display("invalid authorization header: '{}'", value)
    }
  }
}

#[derive(Debug)]
pub struct BasicAuth {
  pub user: String,
  pub pass: String
}

impl FromStr for BasicAuth {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let index = s.find("Basic ").ok_or_else(|| ErrorKind::InvalidAuthorizationHeader(s.to_owned()))?;
    if index > 0 {
      bail!(ErrorKind::InvalidAuthorizationHeader(s.to_owned()))
    }

    let vec = decode(&s["Basic ".len()..])?;
    let user_pass = vec.split(|&b| b == b':').collect::<Vec<_>>();
    if user_pass.len() != 2 {
      bail!(ErrorKind::InvalidAuthorizationHeader(s.to_owned()))
    }

    let user = str::from_utf8(user_pass[0])?.to_owned();
    let pass = str::from_utf8(user_pass[1])?.to_owned();

    Ok(BasicAuth {
      user: user,
      pass: pass
    })
  }
}
