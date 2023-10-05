#![allow(dead_code, unused_variables)]
use roxmltree::{Document, Node};
use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug)]
struct ObjectParserError {
    message: String,
}

impl ObjectParserError {
    fn new(message: &str) -> Self {
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

pub fn parse_model() {
    let object_model = ObjectModelBuilder::default();
    let txt = std::fs::read_to_string("examples/3-1_0.xml").unwrap();
    let doc = Document::parse(txt.as_str()).unwrap();
    if let Some(object_node) = doc
        .descendants()
        .find(|node| node.tag_name().name() == "Object")
    {
        parse_object(object_node, object_model);
    }
}

fn parse_object(
    object_node: Node,
    mut object_model: ObjectModelBuilder,
) -> Result<ObjectModel, ObjectParserError> {
    for child in object_node.children() {
        let tag_name = child.tag_name().name();
        match tag_name {
            "Name" => match child.text() {
                Some(value) => Ok(object_model.name(value.to_owned())),
                None => Err(ObjectParserError::new("No name found")),
            },
            "Description1" => match child.text() {
                Some(value) => Ok(object_model.description(value.to_owned())),
                None => Ok(&mut object_model),
            },
            _ => Ok(&mut object_model),
        }?;
    }
    object_model
        .build()
        .map_err(|err| ObjectParserError::new(err.to_string().as_str()))
}

// Derive builder: https://docs.rs/derive_builder/latest/derive_builder/
#[derive(Debug, derive_builder::Builder)]
pub struct ObjectModel {
    id: u16,
    mandatory: bool,
    name: String,
    description: String,
    description2: String,
    version: String,
    lwm2m_version: String,
    urn: String,
    multiple: MultipleInstances,
    #[builder(field(type = "HashMap<u64, ResourceModel>", build = "HashMap::new()"))]
    resources: HashMap<u64, ResourceModel>,
}

#[derive(Debug, Clone, derive_builder::Builder)]
pub struct ResourceModel {
    id: u16,
    mandatory: bool,
    name: String,
    description: String,
    range: Option<ResourceRange>,
    units: String, // No restrictions
    operations: Vec<ResourceOperation>,
    resourcetype: ResourceType,
    multiple: MultipleInstances,
}

#[derive(Debug, Clone)]
pub enum ResourceOperation {
    Read,
    Write,
    Execute,
}

#[derive(Debug, Clone)]
pub enum MultipleInstances {
    Single,
    Multiple,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    String(Option<String>),
    Integer(Option<i64>),
    UnsignedInteger(Option<u64>),
    Opaque(Option<Vec<u8>>),
    Float(Option<f64>),
    Boolean(Option<bool>),
    ObjectLink(Option<String>), //e.g. <1/0/3>
    Time(Option<u64>),
    CoreLink(Option<String>),
    None, // In case of Execute operation
}
#[derive(Debug, Clone)]
pub enum ResourceRange {
    Numerical(u64, u64),         //start..end  or start-end INCLUSIVE
    NumericalDiscrete(Vec<u64>), //a,b,c, ...
    ByteLength(u64, u64),        //min..max bytes
    ByteDiscrete(Vec<u64>),      //specific byte lengths
    StringLength(u64, u64),      //min..max string bytes
    StringEnum(Vec<String>),     //Possible values for the string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_model() {
        super::parse_model()
    }
}
