use hyper::{Uri, Client, Method, Error, StatusCode};
use hyper::header::UserAgent;
use hyper::client::{Request, FutureResponse};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use futures::future;
use serde_json;
use serde::Deserialize;

use std::fs::{File, OpenOptions}; // TODO file writing to utils.rs
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

use curl::easy::Easy;
use curl::easy;
use utils::{parse_url_from_str, log_response_status};
use utils;
use models::*;
use endpoint::HnNewsEndpoint;
use app::{AppDomain, AppStates, AppStateMachine};

pub fn get_top_story_ids(app_domain: &mut AppDomain,
                         state: &mut AppStateMachine)
                         -> Result<HnListOfItems, Error> {
    let logger = &app_domain.logger; // These need to be here as otherwise it'll cause mutable<>immutable borrow error
    let endpoint = &app_domain.endpoint;
    let client = &app_domain.client;
    state.current_state = AppStates::RetrievingResults;
    let work = request_top_story_ids(&client, &endpoint)
        .and_then(|res| {
            log_response_status(&logger, &endpoint.get_top_stories_path(), &res.status());
            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                })
        })
        .map(|chunks| deserialize::<HnListOfItems>(chunks));
    let result = app_domain.core.run(work);
    state.current_state = AppStates::DoingLocalWork;
    result
}

fn deserialize<T: Deserialize>(chunks: Vec<u8>) -> T {
    let s = String::from_utf8(chunks).unwrap();
    let deserialized: T = serde_json::from_str(&s).unwrap();
    deserialized
}

///
/// Gets HnItem wrapped in Result
///
pub fn get_item_by_id(item: &str,
                      app_domain: &mut AppDomain,
                      state: &mut AppStateMachine)
                      -> Result<HnItem, Error> {
    // note kids in item are comments, parts not sure what it is
    let logger = &app_domain.logger; // These need to be here as otherwise it'll cause mutable<>immutable borrow error
    let endpoint = &app_domain.endpoint;
    let client = &app_domain.client;
    state.current_state = AppStates::RetrievingResults;
    let work = request_item(&item, &client, &endpoint)
        .and_then(|res| {
            log_response_status(&logger, &endpoint.get_item_path(&item), &res.status());
            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                })
        })
        .map(|chunks| deserialize::<HnItem>(chunks));
    state.current_state = AppStates::DoingLocalWork;
    let result = app_domain.core.run(work);
    result
}

pub fn get_comments_for_item(item: &HnItem,
                             app_domain: &mut AppDomain,
                             state: &mut AppStateMachine)
                             -> Option<Vec<HnItem>> {
    // This does not take in account comments to comments
    match item.kids {
        Some(ref kids) => {
            let core = &mut app_domain.core;
            let logger = &mut app_domain.logger; // These need to be mutable as otherwise it'll cause mutable<>immutable borrow error
            let endpoint = &mut app_domain.endpoint;
            let client = &mut app_domain.client;
            state.current_state = AppStates::RetrievingResults;

            let comments: &Vec<i32> = &kids;
            info!(&logger,
                  format!("Retrieving comments for {} with {} comments",
                          &item.id,
                          kids.len()));
            let raw_items = comments.iter()
                .map(|item_id| {
                    (item_id.to_string(), request_item(&item_id.to_string(), &client, &endpoint))
                })
                .map(|(item_id, request_work)| {
                    let subtask = request_work.and_then(|response| {
                        response.body()
                            .fold(Vec::new(), |mut v, chunk| {
                                v.extend(&chunk[..]);
                                future::ok::<_, Error>(v)
                            })
                    });
                    core.run(subtask).unwrap()
                })
                .collect::<Vec<Vec<_>>>();
            let items = raw_items.into_iter().map(|chunks| deserialize::<HnItem>(chunks)).collect();
            state.current_state = AppStates::DoingLocalWork;
            Some(items)
        }
        None => None,
    }
}



fn request_top_story_ids(client: &Client<HttpsConnector>,
                         endpoints: &HnNewsEndpoint)
                         -> FutureResponse {
    let url = parse_url_from_str(&endpoints.get_top_stories_path());
    create_get_request(url, &client)
}

fn request_best_stories_ids(client: &Client<HttpsConnector>,
                            endpoints: &HnNewsEndpoint)
                            -> FutureResponse {
    let url = parse_url_from_str(&endpoints.get_best_stories_path());
    create_get_request(url, &client)
}

fn request_item(item: &str,
                client: &Client<HttpsConnector>,
                endpoints: &HnNewsEndpoint)
                -> FutureResponse {
    let url = parse_url_from_str(&endpoints.get_item_path(item));
    create_get_request(url, &client)
}

pub fn download_page_from_item(item: &HnItem,
                               app_domain: &mut AppDomain,
                               state: &mut AppStateMachine)
                               -> Result<String, String> {
    match item.url {
        Some(ref url) => {
            let core = &mut app_domain.core;
            let logger = &mut app_domain.logger;
            let uri = utils::parse_url_from_str(url);
            let mut filename: String = utils::combine_strings(vec![&uri.path().replace("/", "_"),
                                                                   ".html"]); // TODO Improve naming at some point
            let path = Path::new(&filename);
            let mut file: File =
                OpenOptions::new().write(true).create(true).open(path.as_os_str()).unwrap();

            let v = curl_req(url);
            let s = String::from_utf8(v).unwrap(); // TODO Handle errs
            file.write_all(s.as_bytes());
            Ok(String::from(path.as_os_str().to_str().unwrap())) // should probably be path to string
        }
        None => Err(String::from("No url")),
    }

}

fn curl_req(url: &String) -> Vec<u8> {
    let mut vecced: Vec<u8> = Vec::new();
    let mut easy = Easy::new();
    easy.get(true).unwrap();
    easy.url(url).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
                vecced.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    vecced
}

fn create_get_request(url: Uri, client: &Client<HttpsConnector>) -> FutureResponse {
    let mut request = Request::new(Method::Get, url);
    common_headers(&mut request);
    client.request(request)
}

fn common_headers(req: &mut Request) {
    req.headers_mut().set(UserAgent::new("rs-hackernews-cli"));
}


#[cfg(test)]
mod tests {
    use super::*;
    use app::*;

    #[test]
    fn request_item_test() {
        let mut app_domain = AppDomain::new();
        let s = String::from("8000");
        let response = app_domain.core
            .run(request_item(&s, &app_domain.client, &app_domain.endpoint))
            .unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }
    #[test]
    fn request_top_stories_test() {
        let mut app_domain = AppDomain::new();
        let response = app_domain.core
            .run(request_top_story_ids(&app_domain.client, &app_domain.endpoint))
            .unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }
    #[test]
    fn request_best_stories_test() {
        let mut app_domain = AppDomain::new();
        let response = app_domain.core
            .run(request_best_stories_ids(&app_domain.client, &app_domain.endpoint))
            .unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }

    #[test]
    fn get_item_by_id_test() {
        let mut app_domain = AppDomain::new();
        let mut app_sm = AppStateMachine::new();
        let s = String::from("126809");
        let hnitem: HnItem = get_item_by_id(&s, &mut app_domain, &mut app_sm).unwrap();
        assert!(hnitem.score.unwrap() != 0);
        assert!(hnitem.id != 0);
        assert!(!hnitem.type_str.is_empty());
        assert!(!hnitem.title.unwrap().is_empty());
        assert_eq!(126809, hnitem.id);
    }
    #[test]
    fn get_top_stories_test() {
        let mut app_domain = AppDomain::new();
        let mut app_sm = AppStateMachine::new();
        let top_stories: HnListOfItems = get_top_story_ids(&mut app_domain, &mut app_sm).unwrap();
        assert!(top_stories.values.len() != 0);
    }

    #[test]
    fn get_comments_test() {
        let mut app_domain = AppDomain::new();
        let mut app_sm = AppStateMachine::new();
        let s = String::from("14114235");
        let hnitem: HnItem = get_item_by_id(&s, &mut app_domain, &mut app_sm).unwrap();
        let comments: Vec<HnItem> = get_comments_for_item(&hnitem, &mut app_domain, &mut app_sm)
            .unwrap();
        assert!(comments.len() != 0);
    }
}
