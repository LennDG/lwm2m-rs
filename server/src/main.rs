#![allow(dead_code, unused_variables)]

use std::net::SocketAddr;

use crate::lwm2m_requests::LWM2MRegistrationRequest;
use coap_server::app::{CoapError, Request, Response};
use coap_server::{app, CoapServer, FatalServerError, UdpTransport};

mod lwm2m_requests;
mod registration;

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
    let registration_request = LWM2MRegistrationRequest::try_from(request.clone())?;
    Ok(request.new_response())
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
