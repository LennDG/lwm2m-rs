use std::collections::HashMap;

use roxmltree::{Document, Node};

pub fn parse_model() {
    let txt = std::fs::read_to_string("examples/3-1_0.xml").unwrap();
    let doc = Document::parse(txt.as_str()).unwrap();
    if let Some(object_node) = doc
        .descendants()
        .find(|node| node.tag_name().name() == "Object")
    {
        parse_object(object_node);
    }
}

fn parse_object(object_node: Node) {
    object_node
        .children()
        .for_each(|child| println!("{:?}", child.tag_name().name()))
}

#[derive(Debug, derive_builder::Builder)]
pub struct ObjectModel {
    id: u16,
    mandatory: bool,
    name: String,
    description: String,
    version: String,
    lwm2m_version: String,
    urn: String,
    multiple: MultipleInstances,
    #[builder(field(type = "HashMap<u64, ResourceModel>", build = "HashMap::new()"))]
    resources: HashMap<u64, ResourceModel>,
}

#[derive(Debug, Clone)]
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
