extern crate hyper;
use hyper::Client;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

fn main() {
    println!("Getting data from foobot...");
    let http_client = Client::new();
    let username = "foobot@cyplo.net";
    let url = format!("https://api.foobot.io/v2/user/{}/login/", username);
    let accepted_charsets = vec![(Attr::Charset, Value::Utf8)];
    let app_json_mediatype = qitem(Mime(TopLevel::Application, SubLevel::Json, accepted_charsets));
    let accept_header = Accept(vec![app_json_mediatype]);

    let mut headers = Headers::new();
    headers.set(accept_header);

}


