use crate::lwm2m_requests::{LWM2MBindMode, LWM2MRegistrationRequest, LWM2MVersion};

pub struct Registration {
    endpoint: String,
    lifetime: i32,
    version: LWM2MVersion,
    binding_mode: LWM2MBindMode,
    last_seen: i64,
}

impl From<LWM2MRegistrationRequest> for Registration {
    fn from(value: LWM2MRegistrationRequest) -> Self {
        todo!()
    }
}
