use std::net::SocketAddr;

use coap_server::app::{CoapError, Request, Response};
use coap_server::{app, CoapServer, FatalServerError, UdpTransport};
use crate::lwm2m_requests::LWM2MRegistrationRequest;

pub mod lwm2m_requests;

#[tokio::main]
async fn main() -> Result<(), FatalServerError> {
    let server = CoapServer::bind(UdpTransport::new("0.0.0.0:5683")).await?;
    server
        .serve(
            app::new()
                .resource(app::resource("/hello").get(handle_get_hello))
                .resource(app::resource("/rd").post(handle_register_device)),
        )
        .await
}
async fn handle_register_device(request: Request<SocketAddr>) -> Result<Response, CoapError> {
    let registration_request = LWM2MRegistrationRequest::try_from(request.clone());
    
    match registration_request {
        Err(e) => Err(e),
        Ok(_) => Ok(request.new_response())
    }
}
async fn handle_get_hello(request: Request<SocketAddr>) -> Result<Response, CoapError> {
    let whom = request
        .unmatched_path
        .first()
        .cloned()
        .unwrap_or_else(|| "world".to_string());

    let mut response = request.new_response();
    response.message.payload = format!("Hello, {whom}").into_bytes();
    Ok(response)
}
