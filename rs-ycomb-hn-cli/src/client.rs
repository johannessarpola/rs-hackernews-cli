use hyper::{Client, Uri, Method, Chunk, Error, StatusCode};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::client::{Request, Response, FutureResponse};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use futures::future;
use serde_json;
use slog::*;
use serde::Deserialize;

use models::*;
use endpoint::HnNewsEndpoint;
use app::Main;
use utils::*;

pub fn get_top_story_ids(main: &mut Main) -> Result<HnTopStories, Error> {
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
        .map(|chunks| 
            deserialize::<HnTopStories>(chunks)
        );
    let result = main.core.run(work);
    result
}

fn deserialize<T:Deserialize>(chunks:Vec<u8>) -> T {
    let s = String::from_utf8(chunks).unwrap();
    let deserialized: T = serde_json::from_str(&s).unwrap();
    deserialized
}

///
/// Gets HnItem wrapped in Result 
///
pub fn get_item_by_id(item: &str, main: &mut Main) -> Result<HnItem, Error> {
    let logger = &main.logger; // These need to be here as otherwise it'll cause mutable<>immutable borrow error
    let endpoint = &main.endpoint;
    let client = &main.client;
    let work = request_item(&item, &client, &endpoint)
        .and_then(|res| {
            log_response_status(&logger, &endpoint.get_item_path(&item), &res.status());
            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                })
        })
        .map(|chunks| {
            deserialize::<HnItem>(chunks)
        });
    let result = main.core.run(work);
    result
}

fn get_comments_for_a_story(main: &mut Main, story_id: i32) {}

fn log_response_status(logger: &Logger, url: &str, status: &StatusCode) {
    info!(logger,
          format!("Request to {} finished with status {}", url, status));
}

fn request_top_story_ids(client: &Client<HttpsConnector>,
                         endpoints: &HnNewsEndpoint)
                         -> FutureResponse {
    let url = parse_url_from_str(&endpoints.get_top_stories_path());
    let mut request = Request::new(Method::Get, url);
    common_headers(&mut request);
    client.request(request)
}
fn request_item(item: &str,
                client: &Client<HttpsConnector>,
                endpoints: &HnNewsEndpoint)
                -> FutureResponse {
    let url = parse_url_from_str(&endpoints.get_item_path(item));
    let mut request = Request::new(Method::Get, url);
    common_headers(&mut request);
    client.request(request)
}

fn common_headers(req: &mut Request) {
    req.headers_mut().set(UserAgent::new("hyper"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use app::*;

    #[test]
    fn request_item_test() {
        let mut main = create_main();
        let s = String::from("8000");
        let response = main.core.run(request_item(&s, &main.client, &main.endpoint)).unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }
    #[test]
    fn request_top_stories_test() {
        let mut main = create_main();
        let response = main.core.run(request_top_story_ids(&main.client, &main.endpoint)).unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }

    #[test]
    fn get_item_by_id_test() {
        let mut main = create_main();
        let s = String::from("126809");
        let hnitem: HnItem = get_item_by_id(&s, &mut main).unwrap();
        assert!(hnitem.score != 0);
        assert!(hnitem.id != 0);
        assert!(!hnitem.type_str.is_empty());
        assert!(!hnitem.title.is_empty());
        assert_eq!(126809, hnitem.id);
    }
    #[test]
    fn get_top_stories_test() {
        let mut main = create_main();
        let top_stories: HnTopStories = get_top_story_ids(&mut main).unwrap();
        assert!(top_stories.values.len() != 0);
    }
}
