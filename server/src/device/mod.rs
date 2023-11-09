use chrono::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::time::Duration;

use crate::lwm2m_requests::registration_request::{
    Lwm2mRegistrationObject, Lwm2mRegistrationRequest, Lwm2mVersion,
};

pub struct Device {
    model: DeviceModel,
    device_endpoint: String,
    server_endpoint: String,
    lifetime: Duration,
    last_seen: DateTime<Utc>,
}

impl Device {
    pub fn new(new_reg: Lwm2mRegistrationRequest) -> Self {
        Self {
            last_seen: Utc::now(),
            model: DeviceModel::new(new_reg.objects),
            device_endpoint: new_reg.device_endpoint,
            lifetime: Duration::from_secs(new_reg.lifetime),
            server_endpoint: Self::new_endpoint(),
        }
    }

    pub fn new_endpoint() -> String {
        // Make a 20 character long alphanumeric string.
        // This gets us e35 possible strings so the chances for collisions are VERY low.
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }
}

#[derive(Debug)]
pub struct DeviceModel {
    objects: HashMap<u64, Object>,
}

impl DeviceModel {
    pub fn new(lwm2m_objects: Vec<Lwm2mRegistrationObject>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
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
#[derive(Debug)]
pub struct ObjectInstance {
    resources: Vec<Resource>,
}
#[derive(Debug)]
pub struct Resource {
    id: u16,
    instances: HashMap<u64, ResourceInstance>,
    mandatory: bool,
    name: String,
    description: String,
    range: Option<ResourceRange>,
    units: String, // No restrictions
    operations: Vec<ResourceOperation>,
}
#[derive(Debug)]
pub struct ResourceInstance {
    value: ResourceType,
}
#[derive(Debug)]
pub enum ResourceOperation {
    Read,
    Write,
    Execute,
}

// From https://www.openmobilealliance.org/release/LightweightM2M/V1_2-20201110-A/HTML-Version/OMA-TS-LightweightM2M_Core-V1_2-20201110-A.html#11-0-Appendix-C-Data-Types-Normative
#[derive(Debug)]
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
    None, // In case of Execute operation
}
#[derive(Debug)]
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

    use super::Device;

    #[test]
    fn get_endpoint() {
        println!("New device endpoint: {}", Device::new_endpoint());
    }
}
