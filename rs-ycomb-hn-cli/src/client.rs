use hyper::{Uri, Client, Method, Error, StatusCode};
use hyper::header::UserAgent;
use hyper::client::{Request, FutureResponse};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use futures::future;
use serde_json;
use slog::*;
use serde::Deserialize;

use utils::{parse_url_from_str, log_response_status};
use models::*;
use endpoint::HnNewsEndpoint;
use app::AppDomain;

pub fn get_top_story_ids(app_domain: &mut AppDomain) -> Result<HnListOfItems, Error> {
    let logger = &app_domain.logger; // These need to be here as otherwise it'll cause mutable<>immutable borrow error
    let endpoint = &app_domain.endpoint;
    let client = &app_domain.client;
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
pub fn get_item_by_id(item: &str, app_domain: &mut AppDomain) -> Result<HnItem, Error> {
    // note kids in item are comments, parts not sure what it is
    let logger = &app_domain.logger; // These need to be here as otherwise it'll cause mutable<>immutable borrow error
    let endpoint = &app_domain.endpoint;
    let client = &app_domain.client;
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
    let result = app_domain.core.run(work);
    result
}

fn get_comments_for_item(app_domain: &mut AppDomain, item: &HnItem) -> Option<Vec<HnItem>> {
    match item.kids {
        Some(ref kids) => {
            let core = &mut app_domain.core;
            let logger = &mut app_domain.logger; // These need to be mutable as otherwise it'll cause mutable<>immutable borrow error
            let endpoint = &mut app_domain.endpoint;
            let client = &mut app_domain.client;
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

fn create_get_request(url:Uri, client: &Client<HttpsConnector>) -> FutureResponse {
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
        let mut app_domain = create_app_domain();
        let s = String::from("8000");
        let response = app_domain.core
            .run(request_item(&s, &app_domain.client, &app_domain.endpoint))
            .unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }
    #[test]
    fn request_top_stories_test() {
        let mut app_domain = create_app_domain();
        let response = app_domain.core
            .run(request_top_story_ids(&app_domain.client, &app_domain.endpoint))
            .unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }
    #[test]
    fn request_best_stories_test() {
        let mut app_domain = create_app_domain();
        let response = app_domain.core
            .run(request_best_stories_ids(&app_domain.client, &app_domain.endpoint))
            .unwrap();
        assert_eq!(StatusCode::Ok, response.status());
    }

    #[test]
    fn get_item_by_id_test() {
        let mut app_domain = create_app_domain();
        let s = String::from("126809");
        let hnitem: HnItem = get_item_by_id(&s, &mut app_domain).unwrap();
        assert!(hnitem.score.unwrap() != 0);
        assert!(hnitem.id != 0);
        assert!(!hnitem.type_str.is_empty());
        assert!(!hnitem.title.unwrap().is_empty());
        assert_eq!(126809, hnitem.id);
    }
    #[test]
    fn get_top_stories_test() {
        let mut app_domain = create_app_domain();
        let top_stories: HnListOfItems = get_top_story_ids(&mut app_domain).unwrap();
        assert!(top_stories.values.len() != 0);
    }

    #[test]
    fn get_comments_test() {
        let mut app_domain = create_app_domain();
        let s = String::from("14114235");
        let hnitem: HnItem = get_item_by_id(&s, &mut app_domain).unwrap();
        let comments: Vec<HnItem> = get_comments_for_item(&mut app_domain, &hnitem).unwrap();
        assert!(comments.len() != 0);
    }
}
