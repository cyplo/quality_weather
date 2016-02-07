use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

extern crate rustc_serialize;
use rustc_serialize::base64::{ToBase64, URL_SAFE};

#[macro_use] extern crate hyper;
use hyper::Client;
use hyper::client::IntoUrl;
use hyper::header::{Headers, Accept, Authorization, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

header! { (TokenHeader, "X-API-KEY-TOKEN") => [String] }

fn main() {
    println!("Preparing data ...");

    let mut headers = Headers::new();
    let accept_header = get_accept_header();
    let authorization_header = get_authorization_header();
    let token_header = get_token_header();
    headers.set(accept_header);
    headers.set(authorization_header);
    headers.set(token_header);
    let (username, _, _) = get_credentials();
    let url = format!("https://api.foobot.io/v2/user/{}/login/", username).into_url().unwrap();

    println!("Getting data from foobot at ({})...", url);

    let http_client = Client::new();
    let response = http_client.get(url).headers(headers).send().unwrap(); 
    assert_eq!(response.status, hyper::Ok);
}

fn get_token_header() -> TokenHeader {
    let (_, _, token) = get_credentials(); 
    return TokenHeader(token);
}

fn get_accept_header() -> Accept {
    let accepted_charsets = vec![(Attr::Charset, Value::Utf8)];

    let app_json_mediatype = qitem(Mime(TopLevel::Application, SubLevel::Json, accepted_charsets));
    let accept_header = Accept(vec![app_json_mediatype]);
    return accept_header; 
}

fn get_authorization_header() -> Authorization<String> {
    let (username, password, _) = get_credentials();
    let username_and_password = format!("{}:{}", username, password);
    let base64auth = username_and_password.as_bytes().to_base64(URL_SAFE);
    
    let authorization_header = Authorization(base64auth);
    return authorization_header;
}

fn get_credentials() -> (String, String, String) {
    let file = match File::open("credentials") {
        Ok(handle) => handle,
        Err(_) => panic!("cannot load credentials file")
    };
    let file_reader = BufReader::new(&file);
    
    let mut text_lines = file_reader.lines().map( |line| { line.unwrap() });
    let username = text_lines.next().unwrap();
    let password = text_lines.next().unwrap();
    let token = text_lines.next().unwrap();
    return (username, password, token);
}

