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
extern crate curl;
extern crate webbrowser;
extern crate termion;

mod utils;
mod ui;
mod decoding;
mod logging_utils;
mod formatting;
mod core;

use core::app::*;
use core::models::*;
use core::client;
use ui::cli;

use std::cmp::min;
use tokio_core::reactor::Core;
use std::io::{self, BufRead};
use std::thread::spawn;
use std::process;
use futures::{Stream, Sink, Future};
use futures::sync::mpsc;
use ui::backend::UiCommand;

fn main() {

    let mut app_domain = AppDomain::new();
    let mut app_cache: AppCache = AppCache::new();
    let mut app_state_machine: AppStateMachine = AppStateMachine::new();
    let mut main_core = Core::new().expect("Failed to create core");

    info!(&app_domain.logger, "Application started");
    app_cache.retrieved_top_stories = client::get_top_story_ids(&mut app_domain, &mut app_state_machine)
        .ok();
    print_ten_stories(&mut app_domain, &mut app_cache, &mut app_state_machine);
    let (sender, receiver) = mpsc::channel(1);

    spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let sender = sender.clone();
            sender.send(line).wait().unwrap();
        }
    });
    let listener = receiver.for_each(|verb| {
        let optionCmd = UiCommand::parse(verb);
        match optionCmd {
            Some(cmd) => {
                logging_utils::log_cmd(&app_domain.logger, &cmd);
                gui_listener(cmd, &mut app_domain, &mut app_cache, &mut app_state_machine)
            }
            None => {
                println!("Could not parse command");
                Err(())
            }
        }

    });

    let result = main_core.run(listener);
}

fn gui_listener(cmd: UiCommand,
                app_domain: &mut AppDomain,
                app_cache: &mut AppCache,
                app_state_machine: &mut AppStateMachine)
                -> Result<(), ()> {

    if (cmd.command.is_some()) {

        let verb: String = cmd.command.unwrap();
        let mut numb: usize = 0;
        let mut has_numb = false;
        if (cmd.number.is_some()) {
            numb = cmd.number.unwrap() - 1; // UI is designed as index starting from 1
            has_numb = true;
        }

        if verb == "next" {
            app_state_machine.listing_page_index += 1;
            print_ten_stories(app_domain, app_cache, app_state_machine);
            logging_utils::log_stories_page_with_index(&app_domain.logger,
                                                       app_state_machine.listing_page_index)
        } else if verb == "top" {
            print_ten_stories(app_domain, app_cache, app_state_machine);
        } else if verb == "exit" {
            logging_utils::log_exit(&app_domain.logger);
            process::exit(0);
        } else if verb == "comments" && has_numb {
            load_comments_for_story(numb, app_domain, app_cache, app_state_machine);
            cli::print_comment_and_parent(&app_cache.last_parent_items.back(),
                                          &app_cache.last_retrieved_comments);
        } else if verb == "expand" && has_numb {
            // todo handle non retreived comments
            load_comments_for_commment(numb, app_domain, app_cache, app_state_machine);
            cli::print_comment_and_parent(&app_cache.last_parent_items.back(),
                                          &app_cache.last_retrieved_comments);
        } else if verb == "load" && has_numb {
            load_page_to_local(numb, app_domain, app_cache, app_state_machine);
        } else if verb == "back" {
            if app_state_machine.listing_page_index >= 0 {
                app_state_machine.listing_page_index -= 1;
            }
            print_ten_stories(app_domain, app_cache, app_state_machine);
            logging_utils::log_stories_page_with_index(&app_domain.logger,
                                                       app_state_machine.listing_page_index)
        } else if verb == "open" && has_numb {
            open_page_with_default_browser(numb, app_domain, app_cache, app_state_machine);
        }
    }
    Ok(())
}
fn check_numb_against_stories(numb: usize, app_cache: &mut AppCache) -> Option<usize> {
    match app_cache.retrieved_top_stories {
        Some(ref top_stories) => {
            match app_cache.stories_len() {
                Some(l) => Some(min((l - 1), numb)),
                None => None,
            }
        }
        None => None,
    }
}

fn check_numb_against_comments(numb: usize, app_cache: &mut AppCache) -> Option<usize> {
    match app_cache.retrieved_top_stories {
        Some(ref top_stories) => {
            match app_cache.comments__len() {
                Some(l) => Some(min((l - 1), numb)),
                None => None,
            }
        }
        None => None,
    }
}

fn load_comments_for_story(numb: usize,
                           app_domain: &mut AppDomain,
                           app_cache: &mut AppCache,
                           app_state_machine: &mut AppStateMachine) {
    let opt_numb = check_numb_against_stories(numb, app_cache);
    let mut act_numb = 0;
    if opt_numb.is_none() {
        cli::print_invalid_numb();
    } else {
        act_numb = opt_numb.unwrap();
        if act_numb != numb {
            cli::print_over_limit_but_using_index(act_numb +1);
        }
        let parent_opt = get_story(act_numb, app_domain, app_cache, app_state_machine);
        match parent_opt {
            Some(parent) => {
                retrieve_comments_for_item(parent, app_domain, app_cache, app_state_machine)
            }
            None => cli::print_could_not_get_story(act_numb + 1),
        }
    }
}

fn load_comments_for_commment(numb: usize,
                              app_domain: &mut AppDomain,
                              mut app_cache: &mut AppCache,
                              app_state_machine: &mut AppStateMachine) {

    let opt_numb = check_numb_against_comments(numb, app_cache);
    let mut act_numb = 0;
    if opt_numb.is_none() {
        cli::print_invalid_numb();
    } else {
        act_numb = opt_numb.unwrap();
        if act_numb != numb {
            cli::print_over_limit_but_using_index(act_numb +1);
        }
        let item = app_cache.get_comment_if_kids(act_numb);
        if item.is_some() {
            retrieve_comments_for_item(item.unwrap(), app_domain, app_cache, app_state_machine);
        }
    }
}

fn retrieve_comments_for_item(parent: HnItem,
                              app_domain: &mut AppDomain,
                              app_cache: &mut AppCache,
                              app_state_machine: &mut AppStateMachine) {
    let comments = client::get_comments_for_item(&parent, app_domain, app_state_machine);
    match comments {
        Some(comments_vector) => {
            app_cache.last_parent_items.push_back(parent); // move parent to this location
            app_cache.last_retrieved_comments = Some(comments_vector); // return comments and return
        }
        None => (),
    }
}

fn get_story(numb: usize,
             app_domain: &mut AppDomain,
             app_cache: &mut AppCache,
             app_state_machine: &mut AppStateMachine)
             -> Option<HnItem> {
    let s = format!("{}",
                    app_cache.retrieved_top_stories.as_ref().unwrap().values[numb]); // FIXME unsafe way to do this
    let item = client::get_item_by_id(&s, app_domain, app_state_machine).ok();
    item
}

fn open_page_with_default_browser(numb: usize,
                                  app_domain: &mut AppDomain,
                                  app_cache: &mut AppCache,
                                  app_state_machine: &mut AppStateMachine) {
    let s = format!("{}",
                    app_cache.retrieved_top_stories.as_ref().unwrap().values[numb]); // FIXME unsafe way to do this
    let item = client::get_item_by_id(&s, app_domain, app_state_machine).unwrap();
    if item.url.is_some() && webbrowser::open(&item.url.as_ref().unwrap()).is_ok() {
        // todo cleanup
        logging_utils::log_open_page(&app_domain.logger, item.url.as_ref().unwrap());
        println!("{} {}", "Opened browser to url", item.url.as_ref().unwrap()) // todo move to cli
    }
}

fn load_page_to_local(numb: usize,
                      app_domain: &mut AppDomain,
                      app_cache: &mut AppCache,
                      app_state_machine: &mut AppStateMachine) {
    let s = format!("{}",
                    app_cache.retrieved_top_stories.as_ref().unwrap().values[numb]); // FIXME unsafe way to do this
    let item = client::get_item_by_id(&s, app_domain, app_state_machine).unwrap();
    let filen = client::download_page_from_item(&item, app_domain, app_state_machine);
    match filen {
        Ok(n) => {
            cli::print_filename_of_loaded_page(&n, item.title.as_ref().unwrap());
            logging_utils::log_loaded_page_locally(&app_domain.logger,
                                                   item.url.as_ref().unwrap(),
                                                   &n)
        }
        Err(e) => {
            cli::could_not_load_page(item.title.as_ref().unwrap());
            warn!(&app_domain.logger,
                  format!("Could not load page to file {}", e));
        }
    }

}

fn print_ten_stories(app_domain: &mut AppDomain,
                     app_cache: &mut AppCache,
                     app_state_machine: &mut AppStateMachine) {

    // This probably should not need all the parameters
    let top_stories = app_cache.retrieved_top_stories.as_ref().unwrap();
    logging_utils::log_loaded_top_stories(&app_domain.logger, top_stories.values.len());
    let skipped: usize = (app_state_machine.listing_page_index * 10) as usize;
    let mut index = 0 + skipped as i32;

    for item_id in top_stories.values.iter().skip(skipped).take(10) {
        index += 1;
        let s = format!("{}", item_id);
        let item: HnItem = client::get_item_by_id(&s, app_domain, app_state_machine).unwrap();
        cli::print_headline_with_author(&item, &index);
    }
}
