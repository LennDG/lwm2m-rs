use coap_lite::option_value::OptionValueString;
use coap_lite::CoapOption;
use coap_server::app::{CoapError, Request};
use serde::Deserialize;
use serde_querystring::from_str;
use std::net::SocketAddr;

#[derive(Debug)]
enum LWM2MRequest {
    Registration(LWM2MRegistrationRequest),
}
#[derive(Debug, Deserialize)]
#[serde(rename = "lwm2m")]
enum LWM2MVersion {
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
enum LWM2MBindMode {
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
}

impl TryFrom<Request<SocketAddr>> for LWM2MRegistrationRequest {
    type Error = CoapError;

    fn try_from(request: Request<SocketAddr>) -> Result<Self, Self::Error> {
        let options = request
            .original
            .message
            .get_first_option_as::<OptionValueString>(CoapOption::UriQuery);
        match options {
            None => Err(CoapError {
                code: Some(coap_lite::ResponseType::BadOption),
                message: String::from("Missing all URL query parameters"),
            }),
            Some(option) => match option {
                Err(_) => Err(CoapError {
                    code: Some(coap_lite::ResponseType::InternalServerError),
                    message: String::from("Failed to read options"),
                }),
                Ok(result) => {
                    println!("Options: {}", result.0);
                    let regreq: Result<LWM2MRegistrationRequest, serde_querystring::Error> =
                        from_str(result.0.as_str(), serde_querystring::ParseMode::UrlEncoded);
                    match regreq {
                        Err(_) => Err(CoapError {
                            code: Some(coap_lite::ResponseType::UnprocessableEntity),
                            message: String::from("Incorrect URL query format"),
                        }),
                        Ok(result) => Ok(result),
                    }
                }
            },
        }
    }
}
