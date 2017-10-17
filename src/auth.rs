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
pub enum Auth {
  Basic(Basic),
  Bearer(Bearer),
}

impl FromStr for Auth {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    if let Ok(basic) = s.parse::<Basic>() {
      Ok(Auth::Basic(basic))
    } else if let Ok(bearer) = s.parse::<Bearer>() {
      Ok(Auth::Bearer(bearer))
    } else {
      Err(ErrorKind::InvalidAuthorizationHeader(s.to_owned()).into())
    }
  }
}

#[derive(Debug)]
pub struct Basic {
  pub user: String,
  pub pass: String,
}

impl FromStr for Basic {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let index = s.find("Basic ")
      .ok_or_else(|| ErrorKind::InvalidAuthorizationHeader(s.to_owned()))?;
    if index > 0 {
      bail!(ErrorKind::InvalidAuthorizationHeader(s.to_owned()))
    }

    let vec = decode(&s["Basic ".len()..])?;
    let user_pass = vec.split(|&b| b == b':').collect::<Vec<_>>();
    if user_pass.len() != 2 {
      bail!(ErrorKind::InvalidAuthorizationHeader(s.to_owned()))
    }

    let user = str::from_utf8(user_pass[0])?;
    let pass = str::from_utf8(user_pass[1])?;

    Ok(Basic {
      user: user.to_owned(),
      pass: pass.to_owned(),
    })
  }
}

#[derive(Debug)]
pub struct Bearer {
  pub token: String,
}

impl FromStr for Bearer {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let index = s.find("Bearer ")
      .ok_or_else(|| ErrorKind::InvalidAuthorizationHeader(s.to_owned()))?;
    if index > 0 {
      bail!(ErrorKind::InvalidAuthorizationHeader(s.to_owned()))
    }

    let token = &s["Bearer ".len()..];

    Ok(Bearer { token: token.to_owned() })
  }
}
