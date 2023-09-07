use crate::lwm2m_requests::registration_request::Lwm2mVersion;

pub struct Lwm2mRegistration {}

pub struct Model {
    objects: Vec<Object>,
}

pub struct Object {
    id: u16,
    mandatory: bool,
    name: String,
    description: String,
    instances: Vec<ObjectInstance>,
    version: String,
    lwm2m_version: Lwm2mVersion,
    urn: String,
}

pub struct ObjectInstance {
    instance_id: u16,
    resources: Vec<Resource>,
}

pub struct Resource {
    id: u16,
    instances: Vec<ResourceInstance>,
    mandatory: bool,
    name: String,
    description: String,
    datatype: Option<ResourceType>,
    range: Option<ResourceRange>,
    units: String, // No restrictions
    operations: Vec<ResourceOperation>,
}

pub struct ResourceInstance {
    instance_id: u16,
    value: ResourceType,
}

pub enum ResourceOperation {
    Read,
    Write,
    Execute,
}

pub enum UnitsType {}

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
    ByteDiscrete(Vec<u64>),      //
    StringLength(u64, u64),      //min..max string bytes
    StringEnum(Vec<String>),     //Possible values for the string
}
