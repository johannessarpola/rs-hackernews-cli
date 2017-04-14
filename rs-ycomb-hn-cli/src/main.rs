#![allow(dead_code, unused_imports, unused_mut)] // Unused mut warning is false in this context
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

mod utils;
mod models;
mod client;
mod app;
mod endpoint;

use app::*;
use models::*;
use client::*;


fn main() {
    let mut main = create_main();
    info!(&main.logger, "Application started");
    let top_stories: HnTopStories = get_top_story_ids(&mut main).unwrap();
    for s in &top_stories.values {
        println!("{}", s);
    }
}
