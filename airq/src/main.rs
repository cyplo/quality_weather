use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

extern crate rustc_serialize;
use rustc_serialize::base64::{ToBase64, URL_SAFE};
use rustc_serialize::json::Json;

#[macro_use] extern crate hyper;
use hyper::{Client, Url};
use hyper::client::IntoUrl;
use hyper::client::response::Response;
use hyper::header::{Headers, Accept, Authorization, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

header! { (ApiTokenHeader, "X-API-KEY-TOKEN") => [String] }
header! { (AuthToken, "X-AUTH-TOKEN") => [String] }

fn main() {
    let auth_token = get_auth_token();
    let uuid = get_first_device_uuid(auth_token.clone());
    let path = format!("device/{}/datapoint/0/last/0/", uuid);
    let url = create_url(path);
    let response = get_json(url, auth_token.clone());
    println!("{:?}", response);
}

fn get_json(url: Url, auth_token: AuthToken) -> Json {
    let mut response = send_request(url, auth_token);
    let mut response_body = String::new();
    response.read_to_string(&mut response_body);
    let response_json = Json::from_str(&response_body).unwrap();
    return response_json;
}

fn get_first_device_uuid(auth_token: AuthToken) -> String {
    let (username, _, _) = get_credentials();
    let path = format!("owner/{}/device/", username);
    let url = create_url(path);
    let response_json = get_json(url, auth_token);
    let devices = response_json.as_array().unwrap();
    let ref device = devices[0];
    let device_json_object = device.as_object().unwrap();
    let uuid_json = device_json_object.get("uuid").unwrap();
    let uuid_str = uuid_json.as_string().unwrap();
    let uuid = String::from(uuid_str);
    return uuid;
}

fn create_url(path: String) -> Url {
    return format!("https://api.foobot.io/v2/{}", path).into_url().unwrap();
}

fn send_request(url: Url, auth_token: AuthToken) -> Response {
    let mut headers = Headers::new();
    headers.set(auth_token);
    return send_raw_request(url, headers); 
}

fn send_raw_request(url: Url, headers: Headers) -> Response {
    let http_client = Client::new();
    println!("Sending GET request to: {}", url);
    let response = http_client.get(url).headers(headers).send().unwrap(); 
    println!("Got: {}", response.status); 
    assert_eq!(response.status, hyper::Ok);
    return response;
}

fn get_auth_token() -> AuthToken {
    let mut headers = Headers::new();
    let accept_header = get_accept_header();
    let authorization_header = get_authorization_header();
    let token_header = get_token_header();
    headers.set(accept_header);
    headers.set(authorization_header);
    headers.set(token_header);
    let (username, _, _) = get_credentials();
    let url = create_url(format!("user/{}/login/", username));

    let response = send_raw_request(url, headers);
    let ref headers = response.headers;
    let auth_token = headers.get::<AuthToken>().unwrap();
    return auth_token.clone();
}

fn get_token_header() -> ApiTokenHeader {
    let (_, _, token) = get_credentials(); 
    return ApiTokenHeader(token);
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

