use chrono::prelude::*;
use object_model;
use rand::{distributions::Alphanumeric, Rng};
use std::time::Duration;

mod registration_tracker;

use crate::lwm2m_requests::registration_request::{
    Lwm2mRegistrationObject, Lwm2mRegistrationRequest, Lwm2mVersion,
};

pub struct Device {
    device_endpoint: String,
    server_endpoint: String,
    lifetime: Duration,
    last_seen: DateTime<Utc>,
}

impl Device {
    pub fn new(new_reg: Lwm2mRegistrationRequest) -> Self {
        Self {
            last_seen: Utc::now(),
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

#[cfg(test)]
mod tests {

    use super::Device;

    #[test]
    fn get_endpoint() {
        println!("New device endpoint: {}", Device::new_endpoint());
    }
}
