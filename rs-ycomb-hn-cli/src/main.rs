// Unused mut warning is false in this context
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stream;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate time;
extern crate clap;

mod utils;
mod models;
mod client;
mod app;
mod endpoint;
mod cli;

use app::*;
use models::*;
use client::*;
use cli::*;

use std::io::{self, BufRead};
use std::thread::spawn;
use std::thread;
use futures::{Stream, Sink, Future};
use futures::sync::mpsc;
use futures::sync::mpsc::{Receiver, Sender};
use futures::stream::BoxStream;

fn main() {

    let mut app_domain = AppDomain::new();
    let mut app_cache: AppCache = AppCache::new();
    let mut app_state_machine: AppStateMachine = AppStateMachine::new();

    info!(&app_domain.logger, "Application started");
    app_cache.retrieved_top_stories = get_top_story_ids(&mut app_domain, &mut app_state_machine)
        .ok();
    front_page(&mut app_domain, &mut app_cache, &mut app_state_machine);
    let (mut tx, rx) = mpsc::channel(1);

    let listener = rx.for_each(|res| stdin_listener_sink(res));
    spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            tx = tx.send(line).wait().unwrap();
        }
    });

    app_domain.core.run(listener);

    // run_gui_client(&mut app_domain, &mut app_cache, &mut app_state_machine);
}
fn stdin_listener_sink(res:Result<String, io::Error>) -> Result<(),()> {
        match res {
            Ok(_res) => println!("{}", _res),
            Err(_) => println!("{}", "Error"),
        }
        Ok(())
}

fn front_page(app_domain: &mut AppDomain,
              app_cache: &mut AppCache,
              app_state_machine: &mut AppStateMachine) {
    let top_stories = app_cache.retrieved_top_stories.as_ref().unwrap();
    info!(&app_domain.logger,
          format!("Received {} top stories", top_stories.values.len()));
    let mut index = 0;
    for item_id in top_stories.values.iter().take(10) {
        index += 1;
        let s = format!("{}", item_id);
        let item: HnItem = client::get_item_by_id(&s, app_domain, app_state_machine).unwrap();
        print_headline_with_author(&item, &index);
    }
}

