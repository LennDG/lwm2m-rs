use coap_lite::link_format::{ErrorLinkFormat, LinkAttributeParser, Unquote};
use coap_lite::CoapOption;
use coap_lite::{link_format::LinkFormatParser, option_value::OptionValueString};
use coap_server::app::{CoapError, Request};
use serde::Deserialize;
use serde_querystring::from_str;
use std::net::SocketAddr;
use std::str::{self, FromStr};

#[derive(Debug)]
enum Lwm2mRequest {
    Registration(Lwm2mRegistrationRequest),
}

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
}

impl TryFrom<(&str, Unquote<'_>)> for Lwm2mAttribute {
    type Error = CoapError;

    fn try_from(value: (&str, Unquote)) -> Result<Self, Self::Error> {
        let (attr, u) = value;
        let attr_value = u.to_string();
        match attr {
            "dim" => {
                let dim = attr_value.parse::<u8>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Dimension value {} should be 0-255", attr_value),
                })?;
                Ok(Lwm2mAttribute::Dimension(dim))
            }
            "ssid" => {
                let ssid = attr_value.parse::<u16>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!(
                        "Short Server ID (SSID) value {} should be 0-65534",
                        attr_value
                    ),
                })?;
                Ok(Lwm2mAttribute::Ssid(ssid))
            }
            "uri" => Ok(Lwm2mAttribute::Uri(attr_value)),
            "ver" => Ok(Lwm2mAttribute::ObjectVersion(attr_value)),
            "Lwm2m" => {
                let lwm2m: Lwm2mVersion =
                    serde_plain::from_str(attr_value.as_str()).map_err(|_| CoapError {
                        code: Some(coap_lite::ResponseType::NotAcceptable),
                        message: format!("LWM2M Version {} is not supported.", attr_value),
                    })?;
                Ok(Lwm2mAttribute::Lwm2mVersion(lwm2m))
            }
            "pmin" => {
                let pmin = attr_value.parse::<u64>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Minimum period value {} should be u64", attr_value),
                })?;
                Ok(Lwm2mAttribute::MinPeriod(pmin))
            }
            "pmax" => {
                let pmax = attr_value.parse::<u64>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Maximum period {} should be u64", attr_value),
                })?;
                Ok(Lwm2mAttribute::MaxPeriod(pmax))
            }
            "gt" => {
                let gt = attr_value.parse::<f64>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Greater than {} should be f64", attr_value),
                })?;
                Ok(Lwm2mAttribute::GreaterThan(gt))
            }
            "lt" => {
                let lt = attr_value.parse::<f64>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Less than {} should be f64", attr_value),
                })?;
                Ok(Lwm2mAttribute::LessThan(lt))
            }
            "st" => {
                let st = attr_value.parse::<f64>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Step {} should be f64", attr_value),
                })?;
                Ok(Lwm2mAttribute::Step(st))
            }
            "epmin" => {
                let pmax = attr_value.parse::<u64>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Minumum evaluation period {} should be u64", attr_value),
                })?;
                Ok(Lwm2mAttribute::MinEvalPeriod(pmax))
            }
            "epmax" => {
                let pmax = attr_value.parse::<u64>().map_err(|_| CoapError {
                    code: Some(coap_lite::ResponseType::NotAcceptable),
                    message: format!("Maximum evaluation period {} should be u64", attr_value),
                })?;
                Ok(Lwm2mAttribute::MaxEvalPeriod(pmax))
            }
            _ => Err(CoapError {
                code: Some(coap_lite::ResponseType::UnprocessableEntity),
                message: format!("CoRE attribute {} not recognized", attr_value),
            }),
        }
    }
}

// TODO: support all different attributes
#[derive(Debug, Default)]
pub struct Lwm2mObjects {
    objects: Vec<String>,
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
    objects: Lwm2mObjects,
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
        let objects: Result<Vec<(&str, LinkAttributeParser<'_>)>, ErrorLinkFormat> =
            parse_link_format(payload_str);

        // Check if the content type is application/link-format
        match content_type {
            // If no content-type specified, check if it is valid link-format
            None => {
                if payload_str.trim().is_empty() || objects.is_err() {
                    return Err(CoapError {
                        code: Some(coap_lite::ResponseType::UnprocessableEntity),
                        message: String::from("Content type is not valid application/link-format"),
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

        // Get the payload and convert it into a Lwm2mObjects struct
        let link_string = String::from_utf8(payload).map_err(|_| CoapError {
            code: Some(coap_lite::ResponseType::UnprocessableEntity),
            message: String::from("Unreadable utf8 link-format content"),
        })?;

        // TODO: discover how to use the link_format parser in coaplite
        let objects = Lwm2mObjects {
            objects: link_string
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
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

impl TryFrom<&str> for Lwm2mObjects {
    type Error = CoapError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

fn parse_link_format(payload: &str) -> Result<Vec<(&str, LinkAttributeParser)>, ErrorLinkFormat> {
    let mut parser = LinkFormatParser::new(payload);

    parser.try_fold(vec![], |mut acc, link_result| {
        link_result.map(|link| {
            let (link_str, attr_parser) = link;
            acc.push((link_str, attr_parser));
            acc
        })
    })
}
