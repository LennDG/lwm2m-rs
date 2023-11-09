#![allow(dead_code, unused_variables)]
use core_link::CoreLink;
use err::{ModelNotFoundError, ObjectParserError};
use object_link::ObjectLink;
use std::path::Path;
use std::{collections::HashMap, hash::Hash};

mod core_link;
mod display;
mod err;
mod object_link;
mod xml_parser;

pub enum Model {
    Object(ObjectModel),
    Resource(ResourceModel),
}

pub struct ObjectModelStore {
    models: HashMap<u16, ObjectModelVersions>,
}

impl ObjectModelStore {
    pub fn init_models_from_dir(&mut self, path: &Path) -> Result<(), ObjectParserError> {
        self.models = xml_parser::get_models_from_dir(path)?;
        Ok(())
    }

    pub fn get_model(
        &self,
        link: CoreLink,
        version: Option<Version>,
    ) -> Result<Model, ModelNotFoundError> {
        let object_model = self
            .models
            .get(&link.object_id)
            .ok_or(ModelNotFoundError::IdNotFound(link.clone()))?;

        let versioned_object_model = match version {
            Some(version) => {
                object_model
                    .versions
                    .get(&version)
                    .ok_or(ModelNotFoundError::VersionNotFound {
                        version: version.clone(),
                        link: link.clone(),
                    })
            }
            None => todo!(),
        }?;

        match link.model_type {
            core_link::ModelType::Object => todo!(),
            core_link::ModelType::Resource => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct ObjectModelVersions {
    versions: HashMap<Version, ObjectModel>,
}

// Derive builder: https://docs.rs/derive_builder/latest/derive_builder/
#[derive(Debug, derive_builder::Builder, Clone)]
pub struct ObjectModel {
    id: u16,
    mandatory: bool,
    name: String,
    #[builder(default = "None")]
    description: Option<String>,
    #[builder(default = "None")]
    description2: Option<String>,
    #[builder(default = "Default::default()")]
    version: Version,
    #[builder(default = "Default::default()")]
    lwm2m_version: Version,
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

#[derive(Debug, Clone, Copy)]
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
    ObjectLink(Option<ObjectLink>), //e.g. <1/0/3>
    Time(Option<u64>),              // In case of Execute operation
    CoreLink(Option<CoreLink>),
}

#[derive(Debug, Clone)]
pub enum ResourceRange {
    Numerical(i64, i64),         //start..end  or start-end INCLUSIVE
    NumericalDiscrete(Vec<i64>), //a,b,c, ...
    DiscreteLength(Vec<u64>),    //specific byte lengths
    Length(u64, u64),            //min..max string bytes
    StringEnum(Vec<String>),     //Possible values for the string
    Other(String),               //If enumeration is not able to be determined
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    oma_version: String,
}

impl TryFrom<&str> for Version {
    type Error = ObjectParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use regex::Regex;
        let re = Regex::new(r"^[0-9]\.[0-9]$").unwrap();
        if re.is_match(value) {
            Ok(Version {
                oma_version: value.to_owned(),
            })
        } else {
            Err(ObjectParserError::new(
                "Version is not in format DIGIT.DIGIT",
            ))
        }
    }
}
impl Default for Version {
    fn default() -> Self {
        Self {
            oma_version: String::from("1.0"),
        }
    }
}
