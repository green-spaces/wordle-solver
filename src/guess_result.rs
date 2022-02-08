use std::str::FromStr;
use super::error::{Error, ParseCause};

#[derive(Debug, Clone, PartialEq)]
pub struct GuessResult(String);

impl GuessResult {
  pub fn into_inner(&self) -> &str {
    &self.0
  }
}

impl FromStr for GuessResult {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.trim();

    if s.len() != 5 {
      return Err(Self::Err::Parse(ParseCause::InvalidLength))
    }

    let mut res = String::new();
    for c in s.chars() {
      match c {
        'g' => res.push(c),
        'y' => res.push(c),
        'b' => res.push(c),
        _ => return Err(Self::Err::Parse(ParseCause::InvalidChar)),
      }
    }

    Ok(GuessResult(res))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from_str {
    use super::*;

    #[test]
    fn empty_str() {
      let parse_res = "".parse::<GuessResult>();
      assert!(matches!(parse_res, Err(Error::Parse(ParseCause::InvalidLength))));
    }

    #[test]
    fn long_str() {
      let parse_res = "bbbbbb".parse::<GuessResult>();
      assert!(matches!(parse_res, Err(Error::Parse(ParseCause::InvalidLength))));
    }

    #[test]
    fn all_b_str() {
      let parse_res = "bbbbb".parse::<GuessResult>().unwrap();
      assert_eq!(parse_res, GuessResult(String::from("bbbbb")));
    }

    #[test]
    fn all_y_str() {
      let parse_res = "yyyyy".parse::<GuessResult>().unwrap();
      assert_eq!(parse_res, GuessResult(String::from("yyyyy")));
    }

    #[test]
    fn all_g_str() {
      let parse_res = "ggggg".parse::<GuessResult>().unwrap();
      assert_eq!(parse_res, GuessResult(String::from("ggggg")));
    }

    #[test]
    fn valid_mixed_str() {
      let parse_res = "gbygb".parse::<GuessResult>().unwrap();
      assert_eq!(parse_res, GuessResult(String::from("gbygb")));
    }

    #[test]
    fn legal_str_trailing_whitespace() {
      let parse_res = "gbygb\n".parse::<GuessResult>().unwrap();
      assert_eq!(parse_res, GuessResult(String::from("gbygb")));
    }
  }
}
