use coap_lite::link_format::{LinkAttributeParser, Unquote};
use coap_lite::CoapOption;
use coap_lite::{link_format::LinkFormatParser, option_value::OptionValueString};
use coap_server::app::{CoapError, Request};
use serde::Deserialize;
use serde_querystring::from_str;
use std::net::SocketAddr;
use std::str::{self};

#[derive(Debug)]
enum Lwm2mRequest {
    Registration(Lwm2mRegistrationRequest),
}

// Based on https://www.openmobilealliance.org/release/LightweightM2M/V1_2-20201110-A/HTML-Version/OMA-TS-LightweightM2M_Core-V1_2-20201110-A.html#5-1-0-51-Attributes
#[derive(Debug)]
enum Lwm2mAttribute {
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
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Dimension value {} should be 0-255", attr_value),
                    })
                }),
            "ssid" => attr_value
                .parse::<u16>()
                .map(|parsed_value| Ok(Lwm2mAttribute::Ssid(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
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
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("LWM2M Version {} is not supported.", attr_value),
                    })
                }),
            "pmin" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MinPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Minimum period {} should be u64", attr_value),
                    })
                }),
            "pmax" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MaxPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Maximum period {} should be u64", attr_value),
                    })
                }),
            "gt" => attr_value
                .parse::<f64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::GreaterThan(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Greater than value {} should be f64", attr_value),
                    })
                }),
            "lt" => attr_value
                .parse::<f64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::LessThan(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Less than value {} should be f64", attr_value),
                    })
                }),
            "st" => attr_value
                .parse::<f64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::Step(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Step value {} should be f64", attr_value),
                    })
                }),
            "epmin" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MinEvalPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Minimum evaluation period {} should be u64", attr_value),
                    })
                }),
            "epmax" => attr_value
                .parse::<u64>()
                .map(|parsed_value| Ok(Lwm2mAttribute::MaxEvalPeriod(parsed_value)))
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Maximum evaluation period {} should be u64", attr_value),
                    })
                }),
            "edge" => attr_value
                .parse::<u8>()
                .map(|parsed_value| match parsed_value {
                    0 => Ok(Lwm2mAttribute::Edge(false)),
                    1 => Ok(Lwm2mAttribute::Edge(true)),
                    _ => Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Edge parameter {} should be a 0 or 1", attr_value),
                    }),
                })
                .unwrap_or_else(|_| {
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Edge parameter {} should be a 0 or 1", attr_value),
                    })
                }),
            "con" => attr_value
                .parse::<u8>()
                .map(|parsed_value| match parsed_value {
                    0 => Ok(Lwm2mAttribute::Confirmable(false)),
                    1 => Ok(Lwm2mAttribute::Confirmable(true)),
                    _ => Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!(
                            "Confirmable Notification {} should be a 0 or 1",
                            attr_value
                        ),
                    }),
                })
                .unwrap_or_else(|_| {
                    Err(CoapError {
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
                    Err(CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("Maximum historical queue {} should be u64", attr_value),
                    })
                }),
            _ => Ok(Lwm2mAttribute::Unknown(attr_value)),
        }
    }
}

#[derive(Debug, Default)]
pub struct Lwm2mObject {
    object: String, //TODO: this eventually needs to have much more data w.r.t the LWM2M Object Model, for now just a str
    attributes: Vec<Lwm2mAttribute>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Lwm2m")]
pub enum Lwm2mVersion {
    #[serde(alias = "v1.0")]
    #[serde(alias = "1.0")]
    V10,
    #[serde(alias = "v1.1")]
    #[serde(alias = "1.1")]
    V11,
    #[serde(alias = "v1.2")]
    #[serde(alias = "1.2")]
    V12,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "b")]
pub enum Lwm2mBindMode {
    #[serde(alias = "u")]
    #[serde(alias = "U")]
    Udp,
    #[serde(alias = "t")]
    #[serde(alias = "T")]
    Tcp,
}

#[derive(Debug, Deserialize)]
pub struct Lwm2mRegistrationRequest {
    #[serde(rename = "ep")]
    endpoint: String,
    #[serde(rename = "lt")]
    lifetime: i32,
    #[serde(rename = "Lwm2m")]
    version: Lwm2mVersion,
    #[serde(rename = "b")]
    binding_mode: Lwm2mBindMode,
    #[serde(skip)]
    objects: Vec<Lwm2mObject>,
}

impl TryFrom<Request<SocketAddr>> for Lwm2mRegistrationRequest {
    type Error = CoapError;

    fn try_from(request: Request<SocketAddr>) -> Result<Self, Self::Error> {
        // Get the URL query parameters
        let options = request
            .original
            .message
            .get_first_option_as::<OptionValueString>(CoapOption::UriQuery)
            .ok_or_else(|| CoapError {
                code: Some(coap_lite::ResponseType::BadOption),
                message: String::from("Missing all URL query parameters"),
            })?;

        // Try to read the options
        let option = options.map_err(|_| CoapError {
            code: Some(coap_lite::ResponseType::InternalServerError),
            message: String::from("Failed to read options"),
        })?;

        let content_type = request.original.message.get_content_format();
        let payload = request.original.message.payload;
        let payload_str = str::from_utf8(&payload).map_err(|_| CoapError {
            code: Some(coap_lite::ResponseType::UnprocessableEntity),
            message: String::from("Unreadable utf8 content"),
        })?;
        let objects = parse_link_format(payload_str)?;

        // Check if the content type is application/link-format
        match content_type {
            // If no content-type specified, check if it is valid link-format
            None => {
                if payload_str.trim().is_empty() {
                    return Err(CoapError {
                        code: Some(coap_lite::ResponseType::UnprocessableEntity),
                        message: String::from("Registration requires objects in payload"),
                    });
                }
            }
            Some(coap_lite::ContentFormat::ApplicationLinkFormat) => (),
            _ => {
                return Err(CoapError {
                    code: Some(coap_lite::ResponseType::UnsupportedContentFormat),
                    message: String::from("Content Type unsupported"),
                });
            }
        };

        // Deserialize the options into a request
        let mut regreq: Lwm2mRegistrationRequest =
            from_str(option.0.as_str(), serde_querystring::ParseMode::UrlEncoded).map_err(
                |_| CoapError {
                    code: Some(coap_lite::ResponseType::UnprocessableEntity),
                    message: String::from("Incorrect URL query format"),
                },
            )?;
        regreq.objects = objects;
        Ok(regreq)
    }
}

fn parse_link_format(payload: &str) -> Result<Vec<Lwm2mObject>, CoapError> {
    let mut parser = LinkFormatParser::new(payload);

    let objects = parser.try_fold(vec![], |mut acc, link_result| {
        link_result
            .map_err(|err| CoapError {
                code: Some(coap_lite::ResponseType::UnprocessableEntity),
                message: format! {"{:?}", err},
            })
            .and_then(|link| {
                let (link_str, attr_parser) = link;
                // Parse attributes for the current link and append to acc
                let object = parse_attributes(link_str, attr_parser)?;
                acc.push(object);
                Ok(acc)
            })
    })?;

    Ok(objects)
}

fn parse_attributes(
    object: &str,
    mut attribute_parser: LinkAttributeParser,
) -> Result<Lwm2mObject, CoapError> {
    let attributes = attribute_parser.try_fold(
        vec![],
        |mut acc, attr| -> Result<Vec<Lwm2mAttribute>, CoapError> {
            let attribute = Lwm2mAttribute::try_from(attr)?;
            acc.push(attribute);
            Ok(acc)
        },
    )?;

    let object = Lwm2mObject {
        object: object.to_owned(),
        attributes,
    };

    Ok(object)
}
