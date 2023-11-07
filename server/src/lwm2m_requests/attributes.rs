use super::registration_request::Lwm2mVersion;
use coap_lite::link_format::Unquote;
use coap_server::app::CoapError;

// Based on https://www.openmobilealliance.org/release/LightweightM2M/V1_2-20201110-A/HTML-Version/OMA-TS-LightweightM2M_Core-V1_2-20201110-A.html#5-1-0-51-Attributes
#[derive(Debug)]
pub enum Lwm2mAttribute {
    Dimension(u64),
    Ssid(u64),
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
    ContentType(coap_lite::ContentFormat),
    Unknown(String),
}

// This type comes from the LinkAttributeParser when it is consumed.
impl Lwm2mAttribute {
    pub fn new(value: (&str, Unquote)) -> Result<Self, CoapError> {
        let (attr, u) = value;
        let attr_value = u.to_string();
        match attr {
            "dim" => parse_u64_attribute(attr, &attr_value, "Dimension"),
            "ssid" => parse_u64_attribute(attr, &attr_value, "Short Server ID (SSID)"),
            "uri" => Ok(Lwm2mAttribute::Uri(attr_value)),
            "ver" => Ok(Lwm2mAttribute::ObjectVersion(attr_value)),
            "lwm2m" => serde_plain::from_str(attr_value.as_str())
                .map(|parsed_value| Ok(Lwm2mAttribute::Lwm2mVersion(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("LWM2M Version {} is not supported.", attr_value),
                    })
                }),
            "pmin" => parse_u64_attribute(attr, &attr_value, "Minimum Period"),
            "pmax" => parse_u64_attribute(attr, &attr_value, "Maximum Period"),
            "gt" => parse_f64_attribute(attr, &attr_value, "Greater Than"),
            "lt" => parse_f64_attribute(attr, &attr_value, "Less Than"),
            "st" => parse_f64_attribute(attr, &attr_value, "Step"),
            "epmin" => parse_u64_attribute(attr, &attr_value, "Minimum Evaluation Period"),
            "epmax" => parse_u64_attribute(attr, &attr_value, "Maximum Evaluation Period"),
            "edge" => parse_bool_attribute(
                &attr_value,
                "Edge",
                Lwm2mAttribute::Edge(true),
                Lwm2mAttribute::Edge(false),
            ),
            "con" => parse_bool_attribute(
                &attr_value,
                "Confirmable Notification",
                Lwm2mAttribute::Confirmable(true),
                Lwm2mAttribute::Confirmable(false),
            ),
            "hqmax" => parse_u64_attribute(attr, &attr_value, "Maximum Historical Queue"),
            "ct" => {
                let ct = attr_value.parse::<usize>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: String::from("ct value should be an integer"),
                })?;
                let cf = coap_lite::ContentFormat::try_from(ct).map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("ct value {} not recognized as content format", ct),
                })?;
                Ok(Lwm2mAttribute::ContentType(cf))
            }
            _ => Ok(Lwm2mAttribute::Unknown(attr_value)),
        }
    }
}

fn parse_f64_attribute(
    attr_name: &str,
    attr_value: &str,
    error_message: &str,
) -> Result<Lwm2mAttribute, CoapError> {
    match attr_value.parse::<f64>() {
        Ok(parsed_value) => {
            match attr_name {
                "gt" => Ok(Lwm2mAttribute::GreaterThan(parsed_value)),
                "lt" => Ok(Lwm2mAttribute::LessThan(parsed_value)),
                "st" => Ok(Lwm2mAttribute::Step(parsed_value)),
                _ => {
                    // Handle other cases here
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::InternalServerError),
                        message: format!("Incorrect type (f64) used for parsing {}", attr_name),
                    })
                }
            }
        }
        Err(_) => Err(CoapError {
            code: Some(coap_lite::ResponseType::NotAcceptable),
            message: format!(
                "{} valuetype should be f64, is {}",
                error_message, attr_value
            ),
        }),
    }
}

fn parse_u64_attribute(
    attr_name: &str,
    attr_value: &str,
    error_message: &str,
) -> Result<Lwm2mAttribute, CoapError> {
    match attr_value.parse::<u64>() {
        Ok(parsed_value) => {
            match attr_name {
                "dim" => Ok(Lwm2mAttribute::Dimension(parsed_value)),
                "ssid" => Ok(Lwm2mAttribute::Ssid(parsed_value)),
                "pmin" => Ok(Lwm2mAttribute::MinPeriod(parsed_value)),
                "pmax" => Ok(Lwm2mAttribute::MaxPeriod(parsed_value)),
                "epmin" => Ok(Lwm2mAttribute::MinEvalPeriod(parsed_value)),
                "epmax" => Ok(Lwm2mAttribute::MaxEvalPeriod(parsed_value)),
                "hqmax" => Ok(Lwm2mAttribute::MaxHistoricalQueue(parsed_value)),
                _ => {
                    // Handle other cases here
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::InternalServerError),
                        message: format!("Incorrect type (u64) used for parsing {}", attr_name),
                    })
                }
            }
        }
        Err(_) => Err(CoapError {
            code: Some(coap_lite::ResponseType::NotAcceptable),
            message: format!(
                "{} valuetype should be u64, is {}",
                error_message, attr_value
            ),
        }),
    }
}

fn parse_bool_attribute(
    attr_value: &str,
    error_message: &str,
    true_variant: Lwm2mAttribute,
    false_variant: Lwm2mAttribute,
) -> Result<Lwm2mAttribute, CoapError> {
    match attr_value.parse::<u8>() {
        Ok(parsed_value) => match parsed_value {
            0 => Ok(false_variant),
            1 => Ok(true_variant),
            _ => Err(CoapError {
                code: Some(coap_lite::ResponseType::NotAcceptable),
                message: format!(
                    "{} parameter {} should be a 0 or 1",
                    error_message, attr_value
                ),
            }),
        },
        Err(_) => Err(CoapError {
            code: Some(coap_lite::ResponseType::NotAcceptable),
            message: format!(
                "{} parameter {} should be a 0 or 1",
                error_message, attr_value
            ),
        }),
    }
}
