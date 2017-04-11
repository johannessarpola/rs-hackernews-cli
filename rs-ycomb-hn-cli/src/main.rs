// FIXME Remove once not dev anymore
#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate slog;
extern crate slog_term;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate time;

use std::env;
use std::io::{self, Write};

use serde::{Serialize, Serializer, Deserialize, Deserializer};
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
    let response = create_top_stories_closure(&mut core, &endpoint, &client, &logger);
    println!("{}", response.unwrap());
}

fn create_top_stories_closure(core: &mut Core,
                              endpoint: &HnNews,
                              client: &Client<hyper_tls::HttpsConnector>,
                              logger: &Logger)
                              -> Result<String, hyper::Error> {
    let work = endpoint.start_request_top_story_ids(&client)
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
        })
       .map(|chunks| String::from_utf8(chunks).unwrap());
       core.run(work)
}

fn common_headers(req: &mut Request) {
    req.headers_mut().set(UserAgent::new("hyper"));
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

    fn start_request_top_story_ids(&self,
                                   client: &Client<hyper_tls::HttpsConnector>)
                                   -> FutureResponse {
        let url = parse_url_from_str(&self.get_top_stories_path());
        let mut request = Request::new(Method::Get, url);
        common_headers(&mut request);
        client.request(request)
    }
}

#[derive(Serialize)]
struct HnTopStories {
    values: Vec<i32>,
}

impl Deserialize for HnTopStories {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        Deserialize::deserialize(deserializer).map(|arr: Vec<i32>| HnTopStories { values: arr })
    }
}

#[derive(Serialize, Deserialize)]
struct HnItem {
    by: String,
    descendants: i32,
    id: i32,
    kids: Vec<i32>,
    title: String,
    score: i32,
    time: f64,
    #[serde(rename(deserialize = "type"))]
    type_str: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct HnUser {
    about: String,
    created: f64,
    id: String,
    karma: i32,
    submitted: Vec<i32>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_from_str_test() {
        let url = parse_url_from_str("http://www.google.fi");
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

    #[test]
    fn hn_item_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        assert_eq!(71, deserialized.descendants);
        assert_eq!("dhouston", deserialized.by);
        assert_eq!(8863, deserialized.id);
        assert_eq!(111, deserialized.score);
        assert_eq!(1175714200.0f64, deserialized.time);
        assert_eq!("My YC app: Dropbox - Throw away your USB drive",
                   deserialized.title);
        assert_eq!("story", deserialized.type_str);
        assert_eq!("http://www.getdropbox.com/u/2/screencast.html",
                   deserialized.url);
    }
    #[test]
    fn hn_top_stories_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/top-stories.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnTopStories = serde_json::from_str(&contents).unwrap();
        assert!(deserialized.values.len() > 3);
    }
    #[test]
    fn hn_user_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/user.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnUser = serde_json::from_str(&contents).unwrap();
        assert_eq!("This is a test", deserialized.about);
        assert_eq!(1173923446.0f64, deserialized.created);
        assert_eq!("jl", deserialized.id);
        assert_eq!(3496, deserialized.karma);
        assert!(deserialized.submitted.len() > 3);
    }

}
