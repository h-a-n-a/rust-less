use regex::Regex;
use std::string::String;

pub enum StrOrRegex {
  Regex(Regex),
  Str(String),
}