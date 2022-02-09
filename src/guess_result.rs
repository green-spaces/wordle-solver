use super::error::{Error, ParseCause};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct GuessResult(String);

impl GuessResult {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn into_inner(&self) -> &str {
        &self.0
    }

    pub fn score(&self) -> usize {
        self.0.chars().enumerate().fold(0, |acc, (idx, c)| {
            let coeff = match c {
                'g' => 2,
                'y' => 1,
                _ => 0,
            };

            acc + (coeff * usize::pow(3, idx.try_into().unwrap()))
        })
    }
}

impl FromStr for GuessResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.len() != 5 {
            return Err(Self::Err::Parse(ParseCause::InvalidLength));
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
            assert!(matches!(
                parse_res,
                Err(Error::Parse(ParseCause::InvalidLength))
            ));
        }

        #[test]
        fn long_str() {
            let parse_res = "bbbbbb".parse::<GuessResult>();
            assert!(matches!(
                parse_res,
                Err(Error::Parse(ParseCause::InvalidLength))
            ));
        }

        #[test]
        fn all_b_str() {
            let parse_res = "bbbbb".parse::<GuessResult>().unwrap();
            assert_eq!(parse_res, GuessResult(String::from("bbbbb".to_string())));
        }

        #[test]
        fn all_y_str() {
            let parse_res = "yyyyy".parse::<GuessResult>().unwrap();
            assert_eq!(parse_res, GuessResult(String::from("yyyyy".to_string())));
        }

        #[test]
        fn all_g_str() {
            let parse_res = "ggggg".parse::<GuessResult>().unwrap();
            assert_eq!(parse_res, GuessResult(String::from("ggggg".to_string())));
        }

        #[test]
        fn valid_mixed_str() {
            let parse_res = "gbygb".parse::<GuessResult>().unwrap();
            assert_eq!(parse_res, GuessResult(String::from("gbygb".to_string())));
        }

        #[test]
        fn legal_str_trailing_whitespace() {
            let parse_res = "gbygb\n".parse::<GuessResult>().unwrap();
            assert_eq!(parse_res, GuessResult(String::from("gbygb".to_string())));
        }
    }

    mod score {
        use super::*;

        #[test]
        fn bbbbb() {
            let guess = GuessResult::new("bbbbb".to_string());
            assert_eq!(guess.score(), 0);
        }

        #[test]
        fn ggggg() {
            let guess = GuessResult::new("ggggg".to_string());
            assert_eq!(guess.score(), 242);
        }

        #[test]
        fn yyyyy() {
            let guess = GuessResult::new("yyyyy".to_string());
            assert_eq!(guess.score(), 121);
        }
    }
}
