use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ObjectParserError {
    message: String,
}

impl ObjectParserError {
    pub fn new(message: &str) -> Self {
        ObjectParserError {
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for ObjectParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ObjectParserError {}
