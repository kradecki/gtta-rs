use derive_more::Display;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct ReportedTime(String);
#[derive(Debug)]
pub struct ReportedDate(String);

impl FromStr for ReportedTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"[0-9]+[h|m]").map_err(|_| ParseError::InvalidRegex)?;

        if re.is_match(s) {
            Ok(ReportedTime(s.to_string()))
        } else {
            Err(ParseError::InvalidTimeFormat)
        }
    }
}
impl FromStr for ReportedDate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([12]\d{3}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]))$")
            .map_err(|_| ParseError::InvalidRegex)?;

        if re.is_match(s) {
            Ok(ReportedDate(s.to_string()))
        } else {
            Err(ParseError::InvalidDateFormat)
        }
    }
}

impl fmt::Display for ReportedTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ReportedDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Display, Debug, PartialEq)]
pub enum ParseError {
    InvalidTimeFormat,
    InvalidDateFormat,
    InvalidRegex,
}
impl std::error::Error for ParseError {}
