use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FormatResult};


lazy_static! {
    pub static ref MESSAGE_KEY_CONSTRUCTION_REGEX: Regex = Regex::new( r"(\.)" ).unwrap();
}

pub struct MessageKey {
    path: Vec<String>,
}

impl MessageKey {
    pub fn new(string: String) -> Self {
        let mut path: Vec<String> = Vec::new();
        for s_t_r in MESSAGE_KEY_CONSTRUCTION_REGEX.split(&string) {
            path.push(s_t_r.to_string());
        }
        MessageKey { path: path }
    }

    pub fn get_path(&self) -> &Vec<String> {
        &self.path
    }
}
impl Display for MessageKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let mut output: String = "messagekey: ".to_string();
        for i in self.get_path().iter() {
            output += format!("{},", i).as_str();
        }
        output = output.strip_suffix(",").unwrap().to_string();
        write!(f, "{}", output)
    }
}