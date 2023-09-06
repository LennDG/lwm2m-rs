use super::registration_request::Lwm2mVersion;
use coap_lite::link_format::Unquote;
use coap_server::app::CoapError;

// Based on https://www.openmobilealliance.org/release/LightweightM2M/V1_2-20201110-A/HTML-Version/OMA-TS-LightweightM2M_Core-V1_2-20201110-A.html#5-1-0-51-Attributes
#[derive(Debug)]
pub enum Lwm2mAttribute {
    Dimension(u8),
    Ssid(u16),
    Uri(String),
    ObjectVersion(String),
    Lwm2mVersion(Lwm2mVersion),
    MinPeriod(u64),
    MaxPeriod(u64),
    GreaterThan(f64),
    LessThan(f64),
    Step(f64),
    MinEvalPeriod(u64),
    MaxEvalPeriod(u64),
    Edge(bool),
    Confirmable(bool),
    MaxHistoricalQueue(u64),
    Unknown(String),
}

// This type comes from the LinkAttributeParser when it is consumed.
impl TryFrom<(&str, Unquote<'_>)> for Lwm2mAttribute {
    type Error = CoapError;

    fn try_from(value: (&str, Unquote)) -> Result<Self, Self::Error> {
        let (attr, u) = value;
        let attr_value = u.to_string();
        match attr {
            "dim" => attr_value
                .parse::<u8>()
                .map(|parsed_value| Ok(Lwm2mAttribute::Dimension(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Dimension value {} should be 0-255", attr_value),
                    })
                }),
            "ssid" => attr_value
                .parse::<u16>()
                .map(|parsed_value| Ok(Lwm2mAttribute::Ssid(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!(
                            "Short Server ID (SSID) value {} should be 0-65534",
                            attr_value
                        ),
                    })
                }),
            "uri" => Ok(Lwm2mAttribute::Uri(attr_value)),
            "ver" => Ok(Lwm2mAttribute::ObjectVersion(attr_value)),
            "Lwm2m" => serde_plain::from_str(attr_value.as_str())
                .map(|parsed_value| Ok(Lwm2mAttribute::Lwm2mVersion(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("LWM2M Version {} is not supported.", attr_value),
                    })
                }),
            "pmin" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MinPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Minimum period {} should be u64", attr_value),
                    })
                }),
            "pmax" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MaxPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Maximum period {} should be u64", attr_value),
                    })
                }),
            "gt" => attr_value
                .parse::<f64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::GreaterThan(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Greater than value {} should be f64", attr_value),
                    })
                }),
            "lt" => attr_value
                .parse::<f64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::LessThan(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Less than value {} should be f64", attr_value),
                    })
                }),
            "st" => attr_value
                .parse::<f64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::Step(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Step value {} should be f64", attr_value),
                    })
                }),
            "epmin" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MinEvalPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Minimum evaluation period {} should be u64", attr_value),
                    })
                }),
            "epmax" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MaxEvalPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Maximum evaluation period {} should be u64", attr_value),
                    })
                }),
            "edge" => attr_value
                .parse::<u8>()
                .map(|parsed_value| match parsed_value {
                    0 => Ok(Lwm2mAttribute::Edge(false)),
                    1 => Ok(Lwm2mAttribute::Edge(true)),
                    _ => Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Edge parameter {} should be a 0 or 1", attr_value),
                    }),
                })
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Edge parameter {} should be a 0 or 1", attr_value),
                    })
                }),
            "con" => attr_value
                .parse::<u8>()
                .map(|parsed_value| match parsed_value {
                    0 => Ok(Lwm2mAttribute::Confirmable(false)),
                    1 => Ok(Lwm2mAttribute::Confirmable(true)),
                    _ => Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!(
                            "Confirmable Notification {} should be a 0 or 1",
                            attr_value
                        ),
                    }),
                })
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!(
                            "Confirmable Notification  {} should be a 0 or 1",
                            attr_value
                        ),
                    })
                }),
            "hqmax" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MaxHistoricalQueue(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(Self::Error {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Maximum historical queue {} should be u64", attr_value),
                    })
                }),
            _ => Ok(Lwm2mAttribute::Unknown(attr_value)),
        }
    }
}
