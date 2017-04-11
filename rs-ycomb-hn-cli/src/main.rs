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
use hyper::{Client, Uri, Method, Chunk, Error, StatusCode};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::client::{Request, Response, FutureResponse};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
mod utils;

///
/// 'Main' struct which have relevant parts which are use as core elements of the application
///
struct Main {
    core: Core,
    endpoint: HnNews,
    client: Client<hyper_tls::HttpsConnector>,
    logger: Logger,
}

fn main() {
    let logger = create_loggers();
    info!(logger, "Application started");

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        // Does not check the validity of certificate
        .connector(HttpsConnector::new(4, &handle))
        .build(&handle);
    let endpoint = HnNews::build_default();

    let mut app = Main {
        core: core,
        endpoint: endpoint,
        client: client,
        logger: logger,
    };

    let response = create_top_stories_closure(&mut app);
    println!("{}", response.unwrap());
}
fn get_comments_for_a_story(main: &mut Main) {}

fn create_top_stories_closure(main: &mut Main) -> Result<String, hyper::Error> {
    let logger = &main.logger; // These need to be here as otherwise it'll cause mutable<>immutable borrow error
    let endpoint = &main.endpoint;
    let client = &main.client;
    let work = request_top_story_ids(&client, &endpoint)
        .and_then(|res| {
            log_response_status(&logger, &endpoint.get_top_stories_path(), &res.status());
            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                })
        })
        .map(|chunks| String::from_utf8(chunks).unwrap());

    let result = main.core.run(work);
    result
}

fn log_response_status(logger: &Logger, url: &str, status: &StatusCode) {
    info!(logger,
          format!("Request to {} finished with status {}", url, status));
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

fn request_top_story_ids(client: &Client<hyper_tls::HttpsConnector>,
                         endpoints: &HnNews)
                         -> FutureResponse {
    let url = utils::parse_url_from_str(&endpoints.get_top_stories_path());
    let mut request = Request::new(Method::Get, url);
    common_headers(&mut request);
    client.request(request)
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
        utils::combine_strings(vec![&self.base_url, &self.top_news_suffix, &self.json_suffix])
    }
    pub fn get_max_item_path(&self) -> String {
        utils::combine_strings(vec![&self.base_url, &self.max_item_suffix, &self.json_suffix])
    }
    pub fn get_item_path(&self, id: &str) -> String {
        utils::combine_strings(vec![&self.base_url, &self.item_suffix, id, &self.json_suffix])
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

#[cfg(test)]
mod tests {
    use super::*;

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
