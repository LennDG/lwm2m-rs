use coap_lite::link_format::LinkAttributeParser;
use coap_lite::CoapOption;
use coap_lite::{link_format::LinkFormatParser, option_value::OptionValueString};
use coap_server::app::{CoapError, Request};
use serde::Deserialize;
use serde_querystring::from_str;
use std::net::SocketAddr;
use std::str::{self};

use super::attributes::Lwm2mAttribute;

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
    #[serde(rename = "lwm2m")]
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

        // Check if the content type is application/link-format
        match content_type {
            // If no content-type specified, check if at least not empty.
            // Determining correct format is done when the content is parsed
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
                |err| CoapError {
                    code: Some(coap_lite::ResponseType::UnprocessableEntity),
                    message: format!("Incorrect URL query format: {}", err.message),
                },
            )?;

        regreq.objects = parse_link_format(payload_str)?;
        Ok(regreq)
    }
}

fn parse_link_format(payload: &str) -> Result<Vec<Lwm2mObject>, CoapError> {
    let mut parser = LinkFormatParser::new(payload);

    parser.try_fold(vec![], |mut acc, link_result| {
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
    })
}

fn parse_attributes(
    object_name: &str,
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

    Ok(Lwm2mObject {
        object: object_name.to_owned(),
        attributes,
    })
}
