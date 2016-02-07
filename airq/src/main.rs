extern crate rustc_serialize;
use rustc_serialize::base64::{ToBase64, URL_SAFE};

extern crate hyper;
use hyper::Client;
use hyper::header::{Headers, Accept, Authorization, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

fn main() {
    println!("Preparing data ...");
    let http_client = Client::new();

    let mut headers = Headers::new();
    let accept_header = get_accept_header();
    let authorization_header = get_authorization_header();
    headers.set(accept_header);
    headers.set(authorization_header);
    let username = get_username();
    let url = format!("https://api.foobot.io/v2/user/{}/login/", username);
    println!("Getting data from foobot ...");

}

fn get_accept_header() -> Accept {
    let accepted_charsets = vec![(Attr::Charset, Value::Utf8)];

    let app_json_mediatype = qitem(Mime(TopLevel::Application, SubLevel::Json, accepted_charsets));
    let accept_header = Accept(vec![app_json_mediatype]);
    return accept_header; 
}

fn get_authorization_header() -> Authorization<String> {
    let password = get_password();
    let username = get_username();
    let username_and_password = format!("{}:{}", username, password);
    let base64auth = username_and_password.as_bytes().to_base64(URL_SAFE);
    println!("hash is: {}", base64auth);
    
    let authorization_header = Authorization(base64auth);
    return authorization_header;
}

fn get_username() -> String {
    return "".to_string();
}

fn get_password() -> String {
    return "".to_string();
}

