use std::collections::HashMap;

use crate::lwm2m_requests::registration_request::Lwm2mVersion;

pub struct DeviceModel {
    objects: Vec<Object>,
}

pub struct Object {
    id: u16,
    instances: HashMap<u64, ObjectInstance>,
    mandatory: bool,
    name: String,
    description: String,
    version: String,
    lwm2m_version: Lwm2mVersion,
    urn: String,
}

pub struct ObjectInstance {
    resources: Vec<Resource>,
}

pub struct Resource {
    id: u16,
    instances: HashMap<u64, ResourceInstance>,
    mandatory: bool,
    name: String,
    description: String,
    datatype: Option<ResourceType>,
    range: Option<ResourceRange>,
    units: String, // No restrictions
    operations: Vec<ResourceOperation>,
}

pub struct ResourceInstance {
    value: Option<ResourceType>,
}

pub enum ResourceOperation {
    Read,
    Write,
    Execute,
}

// From https://www.openmobilealliance.org/release/LightweightM2M/V1_2-20201110-A/HTML-Version/OMA-TS-LightweightM2M_Core-V1_2-20201110-A.html#11-0-Appendix-C-Data-Types-Normative
pub enum ResourceType {
    String(String),
    Integer(i64),
    UnsignedInteger(u64),
    Opaque(Vec<u8>),
    Float(f64),
    Boolean(bool),
    ObjectLink(String), //e.g. <1/0/3>
    Time(u64),
    CoreLink(String),
}

pub enum ResourceRange {
    Numerical(u64, u64),         //start..end  or start-end INCLUSIVE
    NumericalDiscrete(Vec<u64>), //a,b,c, ...
    ByteLength(u64, u64),        //min..max bytes
    ByteDiscrete(Vec<u64>),      //specific byte lengths
    StringLength(u64, u64),      //min..max string bytes
    StringEnum(Vec<String>),     //Possible values for the string
}
