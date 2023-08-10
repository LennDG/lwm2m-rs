use coap::CoAPClient;

fn main() {
    let url = "coap://127.0.0.1:5683/hello";
    println!("Client request: {}", url);

    let response = CoAPClient::get(url).unwrap();
    println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());
}