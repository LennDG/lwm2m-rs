use coap::CoAPClient;

fn main() {
    let url = "coap://127.0.0.1:5683/hello";
    println!("Client request: {}", url);

    let response = CoAPClient::get(url).unwrap();
    println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());

    let url = "coap://127.0.0.1:5683/rd?ep=device123&lt=3600&b=U&lwm2m=1.1";
    println!("Client request: {}", url);

    let response = CoAPClient::post(url, Vec::new()).unwrap();
    println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());

    let url = "coap://127.0.0.1:5683/rd";
    println!("Client request: {}", url);

    let response = CoAPClient::post(url, Vec::new()).unwrap();
    println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());

    let url = "coap://127.0.0.1:5683/rd?ep=device123&lt=aaa&b=U&lwm2m=1.1";
    println!("Client request: {}", url);

    let response = CoAPClient::post(url, Vec::new()).unwrap();
    println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());

    let url = "coap://127.0.0.1:5683/rd?ep=device123&lt=3600&b=U&lwm2m=1.3";
    println!("Client request: {}", url);

    let response = CoAPClient::post(url, Vec::new()).unwrap();
    println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());
}