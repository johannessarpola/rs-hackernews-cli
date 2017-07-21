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
extern crate tokio_tls;
extern crate tokio_service;
extern crate curl;
extern crate webbrowser;
extern crate termion;
extern crate native_tls;
extern crate regex;

mod ui;
mod decoding;
mod formatting;
mod core;
mod helpers;

use core::app::*;
use core::models::*;
use core::client;
use ui::cli;
use helpers::logging_utils;

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
    app_cache.retrieved_top_stories =
        client::get_top_story_ids(&mut app_domain, &mut app_state_machine).ok();
    output_stories(&mut app_domain, &mut app_cache, &mut app_state_machine);
    let (sender, receiver) = mpsc::channel(1);

    spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let sender = sender.clone();
            sender.send(line).wait().unwrap();
        }
    });
    let listener = receiver.for_each(|verb| {
        let option_cmd = UiCommand::parse(verb);
        match option_cmd {
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

    if cmd.command.is_some() {

        let verb: String = cmd.command.unwrap();
        let mut numb: usize = 0;
        let mut has_numb = false;
        if cmd.number.is_some() {
            numb = cmd.number.unwrap() - 1; // UI is designed as index starting from 1
            has_numb = true;
        }

        if verb == "next" {
            if app_state_machine.viewing_stories() {
                handle_next_stories(app_domain, app_cache, app_state_machine);
            } else if app_state_machine.viewing_comments() {
                handle_next_comments(app_domain, app_cache, app_state_machine);
            } else {
                // todo wrong state, something went wrong
            }
        } else if verb == "back" {
            if app_state_machine.viewing_stories() {
                handle_previous_stories(app_domain, app_cache, app_state_machine);
            } else if app_state_machine.viewing_comments() {
                handle_previous_comments(app_domain, app_cache, app_state_machine);
            } else {
                // todo wrong state, something went wrong
            }
        } else if verb == "top" {
            output_stories(app_domain, app_cache, app_state_machine);
            app_state_machine.register_viewing_stories();
        } else if verb == "exit" {
            logging_utils::log_exit(&app_domain.logger);
            process::exit(0);
        } else if verb == "comments" && has_numb {
            // needs cache and state as they're retrieved from remote
            safe_load_story(numb, app_domain, app_cache, app_state_machine).and_then(|item| { 
                handle_comments(item, app_domain, app_cache, app_state_machine);
                app_state_machine.register_viewing_comments(); // viewing comments 
                Some(0)
            });
        } else if verb == "expand" && has_numb {
            safe_load_comment(numb, app_cache).and_then(|item| {
                app_state_machine.comments_page_index = 0; // reset comments index
                handle_comments(item, app_domain, app_cache, app_state_machine);
                app_state_machine.register_expanded_comment(); // expanded comments
                Some(0)
            });
        } else if verb == "load" && has_numb {
            cli::print_warning_for_downloading_page();
            download_page(numb, app_domain, app_cache, app_state_machine);
        } else if verb == "open" && has_numb {
            open_page(numb, app_domain, app_cache, app_state_machine);
            app_state_machine.register_opened_story();
        }
        else if verb == "help" {
            cli::print_help();
        }
        // todo print help
        else {
            cli::print_invalid_command();
        }
    }
    Ok(())
}

fn handle_next_comments(app_domain: &mut AppDomain,
                        app_cache: &mut AppCache,
                        app_state_machine: &mut AppStateMachine) {
    let max_comments = app_cache.comments_len();
    if max_comments.is_some() &&
        max_comments.map(|val| under_index_and_over10(val, app_state_machine.listing_page_index)).unwrap() {
        app_state_machine.comments_page_index += 1;
        output_comments(app_domain, app_cache, app_state_machine);
    } else {
        cli::print_tried_to_navigate_over_index();
    }
}

fn handle_previous_comments(app_domain: &mut AppDomain,
                            app_cache: &mut AppCache,
                            app_state_machine: &mut AppStateMachine) {
    if app_state_machine.comments_page_index > 0 {
        app_state_machine.comments_page_index -= 1;
        output_comments(app_domain, app_cache, app_state_machine);
    } else {
        cli::print_tried_to_navigate_over_index();
    }
}


fn handle_next_stories(app_domain: &mut AppDomain,
                       app_cache: &mut AppCache,
                       app_state_machine: &mut AppStateMachine) {
    let max_stories = app_cache.stories_len();
    if max_stories.is_some() && 
        max_stories.map(|val| under_index_and_over10(val, app_state_machine.listing_page_index)).unwrap() {
        app_state_machine.listing_page_index += 1;
        print_and_log_stories(app_domain, app_cache, app_state_machine);
    } else {
        cli::print_tried_to_navigate_over_index();
    }
}

fn under_index_and_over10(val: usize, index:usize) -> bool {
    val > 10 && index * 10 <= val
}

fn handle_previous_stories(app_domain: &mut AppDomain,
                           app_cache: &mut AppCache,
                           app_state_machine: &mut AppStateMachine) {
    if app_state_machine.listing_page_index > 0 {
        app_state_machine.listing_page_index -= 1;
        print_and_log_stories(app_domain, app_cache, app_state_machine);
    } else {
        cli::print_tried_to_navigate_over_index();
    }
}

fn handle_comments(item: HnItem,
                   app_domain: &mut AppDomain,
                   app_cache: &mut AppCache,
                   app_state_machine: &mut AppStateMachine) {
    // todo This overrides the cached comments even if they were spam, probably fixed once the back traverse for comments is implemented
    retrieve_comments_for_item(item, app_domain, app_cache, app_state_machine);
    output_comments(app_domain, app_cache, app_state_machine);
}

fn print_and_log_stories(app_domain: &mut AppDomain,
                         app_cache: &mut AppCache,
                         app_state_machine: &mut AppStateMachine) {
    output_stories(app_domain, app_cache, app_state_machine);
    logging_utils::log_stories_page_with_index(&app_domain.logger,
                                               app_state_machine.listing_page_index);
    app_state_machine.register_viewing_stories();

}


fn check_numb_against_stories(numb: usize, app_cache: &mut AppCache) -> Option<usize> {
    match app_cache.stories_len() {
        Some(l) => Some(min((l - 1), numb)),
        None => None,
    }
}

fn check_numb_against_comments(numb: usize, app_cache: &mut AppCache) -> Option<usize> {
    match app_cache.comments_len() {
        Some(l) => Some(min((l - 1), numb)),
        None => None,
    }
}

fn safe_load_story(numb: usize,
                   app_domain: &mut AppDomain,
                   app_cache: &mut AppCache,
                   app_state_machine: &mut AppStateMachine)
                   -> Option<HnItem> {
    let opt_numb = check_numb_against_stories(numb, app_cache);
    if opt_numb.is_none() {
        cli::print_invalid_numb();
    } else {
        let act_numb = opt_numb.unwrap();
        if act_numb != numb {
            cli::print_over_limit_but_using_index(act_numb + 1);
        }
        let parent_opt = retrieve_story(act_numb, app_domain, app_cache, app_state_machine);
        match parent_opt {
            Some(parent) => {
                return Some(parent); // retrieve_comments_for_item(parent, app_domain, app_cache, app_state_machine)
            }
            None => cli::print_could_not_get_story(act_numb + 1),
        }
    }
    None
}

fn safe_load_comment(numb: usize,
                     app_cache: &mut AppCache)
                     -> Option<HnItem> {

    let opt_numb = check_numb_against_comments(numb, app_cache);
    if opt_numb.is_none() {
        cli::print_invalid_numb();
    } else {
        let act_numb = opt_numb.unwrap(); // todo did this handle index somewhere?
        if act_numb != numb {
            cli::print_over_limit_but_using_index(act_numb + 1);
        }
        return app_cache.get_comment_if_kids(act_numb);
    }
    None
}

fn retrieve_comments_for_item(parent: HnItem,
                              app_domain: &mut AppDomain,
                              app_cache: &mut AppCache,
                              app_state_machine: &mut AppStateMachine)
                              -> bool {
    let comments = client::get_comments_for_item(&parent, app_domain, app_state_machine);
    match comments {
        Some(comments_vector) => {
            app_cache.last_parent_items.push_back(parent); // move parent to this location
            app_cache.last_retrieved_comments = Some(comments_vector); // return comments and return
            true
        }
        None => false,
    }
}

fn retrieve_story(numb: usize,
                  app_domain: &mut AppDomain,
                  app_cache: &mut AppCache,
                  app_state_machine: &mut AppStateMachine)
                  -> Option<HnItem> {
    let s = format!("{}",
                    app_cache.retrieved_top_stories.as_ref().unwrap().values[numb]); // FIXME unsafe way to do this
    let item = client::get_item_by_id(&s, app_domain, app_state_machine).ok();
    item
}

fn open_page(numb: usize,
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

fn download_page(numb: usize,
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

fn output_comments(app_domain: &mut AppDomain,
                      app_cache: &mut AppCache,
                      app_state_machine: &mut AppStateMachine) {

    let skipped: usize = (app_state_machine.comments_page_index * 10) as usize;
    let mut partition: Option<Vec<&HnItem>> = None;
    match app_cache.last_retrieved_comments {
        Some(ref comments) => {
            partition = Some(comments.iter().skip(skipped).take(10).collect());
        }
        None => (),
    }
    cli::print_comments_and_parent(app_cache.last_parent_items.back(),
                                  &partition,
                                  &app_domain.formatters,
                                  skipped);
}

fn output_stories(app_domain: &mut AppDomain,
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
