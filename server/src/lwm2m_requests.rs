use coap_lite::link_format::{ErrorLinkFormat, LinkAttributeParser};
use coap_lite::CoapOption;
use coap_lite::{link_format::LinkFormatParser, option_value::OptionValueString};
use coap_server::app::{CoapError, Request};
use serde::Deserialize;
use serde_querystring::from_str;
use std::net::SocketAddr;
use std::str;

#[derive(Debug)]
enum LWM2MRequest {
    Registration(LWM2MRegistrationRequest),
}

#[derive(Debug, Default)]
pub struct LWM2MObjects {
    objects: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "lwm2m")]
pub enum LWM2MVersion {
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
pub enum LWM2MBindMode {
    #[serde(alias = "u")]
    #[serde(alias = "U")]
    Udp,
    #[serde(alias = "t")]
    #[serde(alias = "T")]
    Tcp,
}

#[derive(Debug, Deserialize)]
pub struct LWM2MRegistrationRequest {
    #[serde(rename = "ep")]
    endpoint: String,
    #[serde(rename = "lt")]
    lifetime: i32,
    #[serde(rename = "lwm2m")]
    version: LWM2MVersion,
    #[serde(rename = "b")]
    binding_mode: LWM2MBindMode,
    #[serde(skip)]
    objects: LWM2MObjects,
}

impl TryFrom<Request<SocketAddr>> for LWM2MRegistrationRequest {
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

        // Get the payload and convert it into a LWM2MObjects struct
        let link_string = String::from_utf8(payload).map_err(|_| CoapError {
            code: Some(coap_lite::ResponseType::UnprocessableEntity),
            message: String::from("Unreadable utf8 link-format content"),
        })?;

        // TODO: discover how to use the link_format parser in coaplite
        let objects = LWM2MObjects {
            objects: link_string
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        };

        // Deserialize the options into a request
        let mut regreq: LWM2MRegistrationRequest =
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

impl TryFrom<&str> for LWM2MObjects {
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
