#![allow(dead_code, unused_variables)]
use err::ObjectParserError;
use roxmltree::{Document, Node};
use std::{collections::HashMap, error::Error, fmt};

mod err;

pub fn parse_model() {
    let object_model = ObjectModelBuilder::default();
    let txt = std::fs::read_to_string("examples/3-1_0.xml").unwrap();
    let doc = Document::parse(txt.as_str()).unwrap();
    if let Some(object_node) = doc
        .descendants()
        .find(|node| node.tag_name().name() == "Object")
    {
        let parsed_object: ObjectModel = parse_object(object_node, object_model).unwrap();
        println!("Parsed object: {:?}", parsed_object)
    }
}

fn parse_object(
    object_node: Node,
    mut object_model: ObjectModelBuilder,
) -> Result<ObjectModel, ObjectParserError> {
    for child in object_node.children() {
        match child.tag_name().name() {
            "Name" => match child.text() {
                Some(value) => Ok(object_model.name(value.to_owned())),
                None => Err(ObjectParserError::new("No name found")),
            },
            "Description1" => match child.text() {
                Some(value) => Ok(object_model.description(value.to_owned())),
                None => Ok(&mut object_model),
            },
            "Description2" => match child.text() {
                Some(value) => Ok(object_model.description2(value.to_owned())),
                None => Ok(&mut object_model),
            },
            "ObjectID" => match child.text() {
                Some(value) => value
                    .parse()
                    .map_err(|_| ObjectParserError::new("Error parsing ObjectID"))
                    .map(|value| object_model.id(value)),
                None => Err(ObjectParserError::new("No ObjectID found")),
            },
            "ObjectURN" => match child.text() {
                Some(value) => Ok(object_model.urn(value.to_owned())),
                None => Ok(&mut object_model),
            },
            "LWM2MVersion" => match child.text() {
                Some(value) => Ok(object_model.lwm2m_version(Some(value.to_owned()))),
                None => Ok(&mut object_model),
            },
            "ObjectVersion" => match child.text() {
                Some(value) => Ok(object_model.version(Some(value.to_owned()))),
                None => Ok(&mut object_model),
            },
            "MultipleInstances" => match child.text() {
                Some("Multiple") => Ok(object_model.multiple(MultipleInstances::Multiple)),
                Some("Single") => Ok(object_model.multiple(MultipleInstances::Single)),
                Some(value) => Err(ObjectParserError::new(
                    format!(
                        "MultipleInstances needs to be Multiple or Single, is: {}",
                        value
                    )
                    .as_str(),
                )),
                None => Err(ObjectParserError::new(
                    "MultipleInstances needs to be Multiple or Single, is empty",
                )),
            },
            "Mandatory" => match child.text() {
                Some("Mandatory") => Ok(object_model.mandatory(true)),
                Some("Optional") => Ok(object_model.mandatory(false)),
                Some(value) => Err(ObjectParserError::new(
                    format!("Mandatory needs to be Mandatory or Optional, is: {}", value).as_str(),
                )),
                None => Err(ObjectParserError::new(
                    "Mandatory needs to be Mandatory or Optional, is empty",
                )),
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
    #[builder(default = "String::new()")]
    description: String,
    #[builder(default = "String::new()")]
    description2: String,
    #[builder(default = "None")]
    version: Option<String>,
    #[builder(default = "None")]
    lwm2m_version: Option<String>,
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
