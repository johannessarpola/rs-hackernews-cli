extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;

use std::env;
use std::io::{self, Write};

use futures::Future;
use futures::stream::Stream;

use hyper::Client;

fn main() {
    let urlStr = String::from("https://hacker-news.firebaseio.com/v0/item/2921983.json");
    let url = urlStr.parse::<hyper::Uri>().unwrap();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle))
        .build(&handle);

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
    core.run(work).unwrap();
}