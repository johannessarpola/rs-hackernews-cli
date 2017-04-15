#![allow(dead_code, unused_mut)] // Unused mut warning is false in this context
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
extern crate rayon;

mod utils;
mod models;
mod client;
mod app;
mod endpoint;

use app::*;
use models::*;
use client::*;


fn main() {
    let mut app_domain = create_app_domain();
    info!(&app_domain.logger, "Application started");
    let top_stories: HnListOfItems = get_top_story_ids(&mut app_domain).unwrap();
    info!(&app_domain.logger, format!("Received {} top stories", top_stories.values.len() ));
    for s in &top_stories.values {
        println!("{}", s);
    }
}
