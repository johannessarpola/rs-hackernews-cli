// FIXME Remove once not dev anymore
#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate slog;
extern crate slog_term;

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate time;


use std::env;
use std::io::{self, Write};

use futures::{Future, Stream};
use futures::future;
use slog::*;
use hyper::{Client, Uri, Method, Chunk, Error};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::client::Request;
use hyper::client::FutureResponse;
use tokio_core::reactor::Core;

fn main() {
    let logger = create_loggers();
    info!(logger, "Application started");

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        // Does not check the validity of certificate
        .connector(hyper_tls::HttpsConnector::new(4, &handle))
        .build(&handle);
    let endpoint = HnNews::build_default();

    let work = endpoint.request_top_story_ids(&client)
        .and_then(|res| {
            info!(logger,
                  format!("Request to {} finished with status {}",
                          endpoint.get_top_stories_path(),
                          res.status()));
            // Consists of Chunks which is basically a vector of bytes (Vec<u8>)
            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    // _ = It's a placeholder. In this context, it means that there isn't enough information for the compiler to infer a type.
                    // http://stackoverflow.com/questions/37215739/what-does-it-mean-to-instantiate-a-rust-generic-with-an-underscore
                    future::ok::<_, Error>(v)
                })
                .map(|chunks| String::from_utf8(chunks).unwrap())
        });
    let result = core.run(work).unwrap();
    println!("{}", result);
}
fn common_headers(req: &mut Request) {
    req.headers_mut().set(UserAgent(String::from("rust-api")))
}
fn consume_request(client: &Client<hyper_tls::HttpsConnector>, request: Request) -> FutureResponse {
    client.request(request)
}

fn create_loggers() -> Logger {
    let drain = slog_term::streamer().build().fuse();
    let root_logger = Logger::root(drain, o!());
    return root_logger;
}

struct HnNews {
    base_url: String,
    top_news_suffix: String,
    item_suffix: String,
    max_item_suffix: String,
    json_suffix: String,
}

impl HnNews {
    pub fn build_default() -> HnNews {
        let e = HnNews {
            base_url: String::from("https://hacker-news.firebaseio.com/v0/"),
            top_news_suffix: String::from("topstories"),
            item_suffix: String::from("item/"),
            max_item_suffix: String::from("maxitem"),
            json_suffix: String::from(".json"),
        };
        e
    }

    pub fn get_top_stories_path(&self) -> String {
        combine_strings(vec![&self.base_url, &self.top_news_suffix, &self.json_suffix])
    }
    pub fn get_max_item_path(&self) -> String {
        combine_strings(vec![&self.base_url, &self.max_item_suffix, &self.json_suffix])
    }
    pub fn get_item_path(&self, id: &str) -> String {
        combine_strings(vec![&self.base_url, &self.item_suffix, id, &self.json_suffix])
    }

    fn request_top_story_ids(&self, client: &Client<hyper_tls::HttpsConnector>) -> FutureResponse {
        let url = parse_url_from_str(&self.get_top_stories_path());
        let mut request = Request::new(Method::Get, url);
        common_headers(&mut request);
        client.request(request)
    }
}

fn combine_strings(strings: Vec<&str>) -> String {
    let combine = strings.join("");
    combine
}

fn parse_url_from_str(url_str: &str) -> Uri {
    let url_str = String::from(url_str);
    let url = url_str.parse::<hyper::Uri>().unwrap();
    url
}


#[test]
fn parse_url_from_str_test() {
    let url = parse_url_from_str("http://www.gooogle.fi");
    assert_eq!("http", url.scheme().unwrap());
    assert_eq!("www.google.fi", url.authority().unwrap());
}

#[test]
fn combine_strings_test() {
    let a = "Abc";
    let b = "Abc";
    let mut vec = Vec::new();
    vec.push(a);
    vec.push(b);
    assert_eq!("AbcAbc", combine_strings(vec));
    assert!(a.len() > 1);
    assert!(b.len() > 1);
}
