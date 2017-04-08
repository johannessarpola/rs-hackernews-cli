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
use hyper::client::Request;
use hyper::client::FutureResponse;
use tokio_core::reactor::Core;

fn main() {
    let logger = create_loggers();
    info!(logger, "Application started");

    let urlStr = String::from("https://hacker-news.firebaseio.com/v0/item/2921983.json");
    let url = urlStr.parse::<hyper::Uri>().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle))
        .build(&handle);
    let endpoint = hn_news::build_default();

    let work = client.get(url).and_then(|res| {
        println!("Status: {}", res.status());
        println!("Headers:\n{}", res.headers());
        res.body().for_each(|chunk| {
            ::std::io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    });
    let url2 = String::from("https://httpbin.org/post").parse::<hyper::Uri>().unwrap();
    let request = Request::new(Method::Post, url2);
    core.run(consume_request(&client, request))
        .and_then(|res| {
            info!(logger,
                  format!("Request finished with status {}", res.status()));
            Ok("ok") // TODO This should handle failures
        });
    let work = endpoint.request_top_story_ids(&client)
        .and_then(|res| {
            info!(logger,
                  format!("Request to {} finished with status {}",
                          endpoint.get_top_stories_path(),
                          res.status()));
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
    // core.run(work).unwrap();
}
fn consume_request(client: &Client<hyper_tls::HttpsConnector>, request: Request) -> FutureResponse {
    client.request(request)
}

fn create_loggers() -> Logger {
    let drain = slog_term::streamer().build().fuse();
    let root_logger = Logger::root(drain, o!());
    return root_logger;
}

struct hn_news {
    base_url: String,
    top_news_suffix: String,
    item_suffix: String,
    max_item_suffix: String,
    json_suffix: String,
}

impl hn_news {
    pub fn build_default() -> hn_news {
        let e = hn_news {
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
        let request = Request::new(Method::Get, url);
        client.request(request)
    }
}

fn combine_strings(strings: Vec<&str>) -> String {
    let combine = strings.join("");
    combine
}

fn parse_url_from_str(urlStr: &str) -> Uri {
    let urlStr = String::from(urlStr);
    let url = urlStr.parse::<hyper::Uri>().unwrap();
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
