use std::{error::Error, fmt};

use crate::{core_link::CoreLink, Version};

#[derive(Debug)]
pub enum ObjectModelError {
    Parser(String),
    NotFound(String),
}

impl fmt::Display for ObjectModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectModelError::Parser(message) => write!(f, "{}", message),
            ObjectModelError::NotFound(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for ObjectModelError {}

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

#[derive(Debug)]
pub enum ModelNotFoundError {
    ObjectId(CoreLink),
    ResourceId(CoreLink),
    Version { version: Version, link: CoreLink },
}

impl fmt::Display for ModelNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ModelNotFoundError::ObjectId(link) => {
                write!(
                    f,
                    "Object model for link {} was not found in model registry",
                    link
                )
            }
            ModelNotFoundError::ResourceId(link) => {
                write!(
                    f,
                    "Resource model for link {} was not found in model registry",
                    link
                )
            }
            ModelNotFoundError::Version { version, link } => {
                write!(
                    f,
                    "Object version {} for link {} was not found in model registry",
                    version, link
                )
            }
        }
    }
}

impl std::error::Error for ModelNotFoundError {}
