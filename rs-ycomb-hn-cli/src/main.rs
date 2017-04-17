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

use tokio_core::reactor::{Core, Handle};
use std::io::{self, BufRead};
use std::thread::spawn;
use std::thread;
use std::process;
use futures::{Stream, Sink, Future};
use futures::sync::mpsc;
use futures::sync::mpsc::{Receiver, Sender};
use futures::stream::BoxStream;

fn main() {

    let mut app_domain = AppDomain::new();
    let mut app_cache: AppCache = AppCache::new();
    let mut app_state_machine: AppStateMachine = AppStateMachine::new();
    let mut main_core = Core::new().expect("Failed to create core");
    info!(&app_domain.logger, "Application started");
    app_cache.retrieved_top_stories = get_top_story_ids(&mut app_domain, &mut app_state_machine)
        .ok();
    print_ten_stories(&mut app_domain, &mut app_cache, &mut app_state_machine);
    let (mut sender, receiver) = mpsc::channel(1);

    spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let sender = sender.clone();
            sender.send(line).wait().unwrap();
        }
    });
    let listener =
        receiver.for_each(|msg| {
            gui_listener(msg, &mut app_domain, &mut app_cache, &mut app_state_machine)
        });
    main_core.run(listener);
}

fn gui_listener(msg_result: Result<String, io::Error>,
                app_domain: &mut AppDomain,
                app_cache: &mut AppCache,
                app_state_machine: &mut AppStateMachine)
                -> Result<(), ()> {
    // TODO Handle different commands and arguments related to state
    // TODO Logging
    match msg_result {
        Ok(msg) => {
            if msg.len() >= 4 && &msg[0..4] == "next" {
                app_state_machine.listing_page_index += 1;
                print_ten_stories(app_domain, app_cache, app_state_machine);
            } else if msg.parse::<i32>().is_ok() {
                let numb = msg.parse::<i32>().unwrap();
                println!("{}", numb);
                // TODO open story by index
            } else if (msg.len() >= 4 && &msg[0..4] == "exit") {
                process::exit(0);
            
            } else if msg.len() >= 8 && &msg[0..8] == "comments" {
                // TODO get comments for story
            }
            else if msg.len() >= 4 && &msg[0..4] == "load" {
                // TODO load page linked in the url to a folder
            } 
            else if msg.len() >= 4 && &msg[0..4] == "back"  {
                if app_state_machine.listing_page_index >= 0  {
                    app_state_machine.listing_page_index -= 1;
                }
                print_ten_stories(app_domain, app_cache, app_state_machine);
            } 
        }
        Err(_) => println!("{}", "Error"),
    }
    Ok(())
}

fn print_ten_stories(app_domain: &mut AppDomain,
                     app_cache: &mut AppCache,
                     app_state_machine: &mut AppStateMachine) {

    // This probably should not need all the parameters
    let top_stories = app_cache.retrieved_top_stories.as_ref().unwrap();
    info!(&app_domain.logger,
          format!("Received {} top stories", top_stories.values.len()));
    let skipped: usize = (app_state_machine.listing_page_index * 10) as usize;
    let mut index = 0 + skipped as i32;

    for item_id in top_stories.values.iter().skip(skipped).take(10) {
        index += 1;
        let s = format!("{}", item_id);
        let item: HnItem = client::get_item_by_id(&s, app_domain, app_state_machine).unwrap();
        print_headline_with_author(&item, &index);
    }
}
