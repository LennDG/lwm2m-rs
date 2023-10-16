#![allow(dead_code, unused_variables)]
use err::ObjectParserError;
use roxmltree::{Document, Node};
use std::{collections::HashMap, path::PathBuf};
use walkdir::WalkDir;

mod err;

pub fn get_models_from_dir(
    filepath: &PathBuf,
) -> Result<HashMap<u16, ObjectModel>, ObjectParserError> {
    if filepath.is_dir() {
        WalkDir::new(filepath)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_string_lossy().to_string();
                if entry.file_type().is_file()
                    && file_name.ends_with(".xml")
                    && file_name
                        .strip_suffix(".xml")
                        .unwrap() //can unwrap because previous condition checks for .xml
                        .chars()
                        .all(char::is_numeric)
                {
                    Some(entry.path().to_owned())
                } else {
                    None
                }
            })
            .try_fold(
                HashMap::new(),
                |mut acc, file| -> Result<HashMap<u16, ObjectModel>, ObjectParserError> {
                    let model = parse_model(&file)?;
                    acc.insert(model.id, model);
                    Ok(acc)
                },
            )
    } else {
        Err(ObjectParserError::new("Path is not a directory"))
    }
}

fn parse_model(filepath: &PathBuf) -> Result<ObjectModel, ObjectParserError> {
    let object_model = ObjectModelBuilder::default();
    let txt = std::fs::read_to_string(filepath).unwrap();
    let doc = Document::parse(txt.as_str()).unwrap();
    if let Some(object_node) = doc
        .descendants()
        .find(|node| node.tag_name().name() == "Object")
    {
        parse_object(object_node, object_model)
    } else {
        Err(ObjectParserError::new("No Object found in file"))
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
                None => Err(ObjectParserError::new("No object name found")),
            },
            "Description1" => match child.text() {
                Some(value) => Ok(object_model.description(Some(value.to_owned()))),
                None => Ok(&mut object_model),
            },
            "Description2" => match child.text() {
                Some(value) => Ok(object_model.description2(Some(value.to_owned()))),
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
                Some("Multiple") => Ok(object_model.multiple(true)),
                Some("Single") => Ok(object_model.multiple(false)),
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
            "Resources" => {
                let mut resources = HashMap::new();
                for resource_child in child.children() {
                    if resource_child.tag_name().name() == "Item" {
                        let mut resource_model = ResourceModelBuilder::default();
                        let id = match resource_child.attribute("ID") {
                            Some(id) => id
                                .parse()
                                .map_err(|_| ObjectParserError::new("Error parsing Resource ID")),
                            None => Err(ObjectParserError::new("No Resource ID found")),
                        }?;
                        resource_model.id(id);
                        let resource = parse_resource(resource_child, resource_model)?;
                        resources.insert(id, resource);
                    }
                }
                Ok(object_model.resources(resources))
            }
            _ => Ok(&mut object_model),
        }?;
    }
    object_model
        .build()
        .map_err(|err| ObjectParserError::new(err.to_string().as_str()))
}

fn parse_resource(
    resource_node: Node,
    mut resource_model: ResourceModelBuilder,
) -> Result<ResourceModel, ObjectParserError> {
    for child in resource_node.children() {
        match child.tag_name().name() {
            "Name" => match child.text() {
                Some(value) => Ok(resource_model.name(value.to_owned())),
                None => Err(ObjectParserError::new("No resource name found")),
            },
            "Operations" => match child.text() {
                Some("R") => Ok(resource_model.operations(Some(ResourceOperation::Read))),
                Some("W") => Ok(resource_model.operations(Some(ResourceOperation::Write))),
                Some("RW") => Ok(resource_model.operations(Some(ResourceOperation::ReadWrite))),
                Some("E") => Ok(resource_model.operations(Some(ResourceOperation::Execute))),
                Some("") => Ok(resource_model.operations(None)),
                None => Ok(&mut resource_model),
                Some(value) => Err(ObjectParserError::new(
                    format!("Operations needs to be R, W, RW, E or empty, is: {}", value).as_str(),
                )),
            },
            "MultipleInstances" => match child.text() {
                Some("Multiple") => Ok(resource_model.multiple(true)),
                Some("Single") => Ok(resource_model.multiple(false)),
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
                Some("Mandatory") => Ok(resource_model.mandatory(true)),
                Some("Optional") => Ok(resource_model.mandatory(false)),
                Some(value) => Err(ObjectParserError::new(
                    format!("Mandatory needs to be Mandatory or Optional, is: {}", value).as_str(),
                )),
                None => Err(ObjectParserError::new(
                    "Mandatory needs to be Mandatory or Optional, is empty",
                )),
            },
            "Type" => match child.text() {
                Some("String") => Ok(resource_model.resourcetype(Some(ResourceType::String(None)))),
                Some("Integer") => Ok(resource_model.resourcetype(Some(ResourceType::Integer(None)))),
                Some("Unsigned Integer") => Ok(resource_model.resourcetype(Some(ResourceType::UnsignedInteger(None)))),
                Some("Float") => Ok(resource_model.resourcetype(Some(ResourceType::Float(None)))),
                Some("Boolean") => Ok(resource_model.resourcetype(Some(ResourceType::Boolean(None)))),
                Some("Opaque") => Ok(resource_model.resourcetype(Some(ResourceType::Opaque(None)))),
                Some("Time") => Ok(resource_model.resourcetype(Some(ResourceType::Time(None)))),
                Some("Objlnk") => Ok(resource_model.resourcetype(Some(ResourceType::ObjectLink(None)))),
                Some("Corelnk") => Ok(resource_model.resourcetype(Some(ResourceType::CoreLink(None)))),
                None => Ok(&mut resource_model),
                Some(value) => Err(ObjectParserError::new(
                    format!("Resource Type can be String, Integer, Float, Boolean, Opaque, Time, Objlnk or empty, is: {}", value).as_str(),
                )),
            },
            "Description" => match child.text() {
                Some(value) => Ok(resource_model.description(Some(value.to_owned()))),
                None => Ok(&mut resource_model),
            },
            "Units" => match child.text() {
                Some(value) => Ok(resource_model.units(Some(value.to_owned()))),
                None => Ok(&mut resource_model),
            },
            "RangeEnumeration" => match child.text() {
                Some(value) => Ok(resource_model.range(Some(parse_range_enumeration(value)))),
                None => Ok(&mut resource_model),
            },
            _ => Ok(&mut resource_model),
        }?;
    }

    resource_model
        .build()
        .map_err(|err| ObjectParserError::new(err.to_string().as_str()))
}

fn parse_range_enumeration(enumeration: &str) -> ResourceRange {
    // TODO: parse the enumeration
    ResourceRange::Other(enumeration.to_owned())
}

// Derive builder: https://docs.rs/derive_builder/latest/derive_builder/
#[derive(Debug, derive_builder::Builder)]
pub struct ObjectModel {
    id: u16,
    mandatory: bool,
    name: String,
    #[builder(default = "None")]
    description: Option<String>,
    #[builder(default = "None")]
    description2: Option<String>,
    #[builder(default = "None")]
    version: Option<String>,
    #[builder(default = "None")]
    lwm2m_version: Option<String>,
    urn: String,
    multiple: bool,
    #[builder(default = "HashMap::new()")]
    resources: HashMap<u16, ResourceModel>,
}

#[derive(Debug, Clone, derive_builder::Builder)]
pub struct ResourceModel {
    id: u16,
    mandatory: bool,
    name: String,
    #[builder(default = "None")]
    description: Option<String>,
    #[builder(default = "None")]
    range: Option<ResourceRange>,
    #[builder(default = "None")]
    units: Option<String>, // No restrictions
    #[builder(default = "None")]
    operations: Option<ResourceOperation>,
    #[builder(default = "None")]
    resourcetype: Option<ResourceType>,
    multiple: bool,
}

#[derive(Debug, Clone)]
pub enum ResourceOperation {
    Read,
    Write,
    ReadWrite,
    Execute,
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
    Time(Option<u64>),          // In case of Execute operation
    CoreLink(Option<String>),
}
#[derive(Debug, Clone)]
pub enum ResourceRange {
    Numerical(i64, i64),         //start..end  or start-end INCLUSIVE
    NumericalDiscrete(Vec<i64>), //a,b,c, ...
    ByteLength(u64, u64),        //min..max bytes
    ByteDiscrete(Vec<u64>),      //specific byte lengths
    StringLength(u64, u64),      //min..max string bytes
    StringEnum(Vec<String>),     //Possible values for the string
    Other(String),               //If enumeration is not able to be determined
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn parse_all() {
        let directory_path = "/home/lenndg/projects/lwm2m-registry";
        let result = super::get_models_from_dir(&PathBuf::from(directory_path));
        assert!(result.is_ok())
    }
}
