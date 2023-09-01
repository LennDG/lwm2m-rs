use crate::lwm2m_requests::{Lwm2mBindMode, Lwm2mRegistrationRequest, Lwm2mVersion};

pub struct Registration {
    endpoint: String,
    lifetime: i32,
    version: Lwm2mVersion,
    binding_mode: Lwm2mBindMode,
    last_seen: i64,
}

impl From<Lwm2mRegistrationRequest> for Registration {
    fn from(value: Lwm2mRegistrationRequest) -> Self {
        todo!()
    }
}
