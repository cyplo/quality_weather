use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

extern crate rustc_serialize;
use rustc_serialize::base64::{ToBase64, URL_SAFE};

extern crate hyper;
use hyper::Client;
use hyper::header::{Headers, Accept, Authorization, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

fn main() {
    println!("Preparing data ...");

    let mut headers = Headers::new();
    let accept_header = get_accept_header();
    let authorization_header = get_authorization_header();
    headers.set(accept_header);
    headers.set(authorization_header);
    let (username, _) = get_username_and_password();
    let url = format!("https://api.foobot.io/v2/user/{}/login/", username);
    println!("Getting data from foobot ...");

    let http_client = Client::new();
    
}

fn get_accept_header() -> Accept {
    let accepted_charsets = vec![(Attr::Charset, Value::Utf8)];

    let app_json_mediatype = qitem(Mime(TopLevel::Application, SubLevel::Json, accepted_charsets));
    let accept_header = Accept(vec![app_json_mediatype]);
    return accept_header; 
}

fn get_authorization_header() -> Authorization<String> {
    let (username, password) = get_username_and_password();
    let username_and_password = format!("{}:{}", username, password);
    let base64auth = username_and_password.as_bytes().to_base64(URL_SAFE);
    
    let authorization_header = Authorization(base64auth);
    return authorization_header;
}

fn get_username_and_password() -> (String, String) {
    let file = match File::open("credentials") {
        Ok(handle) => handle,
        Err(_) => panic!("cannot load credentials file")
    };
    let file_reader = BufReader::new(&file);
    
    let mut text_lines = file_reader.lines().map( |line| { line.unwrap() });
    let username = text_lines.next().unwrap();
    let password = text_lines.next().unwrap();
    return (username, password);
}

