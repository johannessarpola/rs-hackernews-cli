use std::io;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::collections::VecDeque;

use hyper::client::HttpConnector;
use hyper::{Client};
use native_tls::TlsConnector;
use tokio_core::reactor::{Core, Handle};

use log;
use std;
use fern;
use chrono;
use super::endpoint::HnNewsEndpoint;
use super::models::{HnItem, HnListOfItems};
use super::connector::HttpsConnector;
use helpers::gen_utils::comment_has_kids;
use formatting::formatter::Formatters;

///
/// 'AppDomain' struct which have relevant parts which are use as core elements of the application
///
pub struct AppDomain {
    pub core: Core,
    pub endpoint: HnNewsEndpoint,
    pub client: Client<HttpsConnector>,
    pub formatters: Formatters,
}

impl AppDomain {
    pub fn new() -> AppDomain {
        initialize_loggers();
        let core = Core::new().expect("Failed to create core");
        let handle = core.handle();
        let client = configure_client(&handle);
        let endpoint = HnNewsEndpoint::build_default();
        let formatters = Formatters::new();
        AppDomain {
            core: core,
            endpoint: endpoint,
            client: client,
            formatters: formatters,
        }
    }
}

pub struct AppCache {
    pub retrieved_top_stories: Option<HnListOfItems>,
    pub retrieved_best_stories: Option<HnListOfItems>,
    pub retrieved_new_stories: Option<HnListOfItems>,
    pub last_retrieved_item: Option<HnItem>,
    pub last_parent_items: VecDeque<HnItem>, // this does not need to be optional
    pub last_retrieved_comments: Option<Vec<HnItem>>,
}

impl AppCache {
    pub fn new() -> AppCache {
        AppCache {
            retrieved_top_stories: None,
            retrieved_best_stories: None,
            retrieved_new_stories: None,
            last_retrieved_item: None,
            last_parent_items: VecDeque::new(),
            last_retrieved_comments: None,
        }
    }
    pub fn get_comment(&mut self, numb: usize) -> Option<HnItem> {
        match self.last_retrieved_comments {
            Some(ref mut comments) => Some(comments.remove(numb)),
            None => None,
        }
    }

    pub fn get_comment_if_kids(&mut self, numb: usize) -> Option<HnItem> {
        match self.last_retrieved_comments {
            Some(ref mut comments) => {
                if comment_has_kids(&comments[numb]) {
                    Some(comments.remove(numb))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn stories_len(&self) -> Option<usize> {
        match self.retrieved_top_stories {
            Some(ref top_stories) => Some(top_stories.values.len()),
            None => None,
        }
    }

    pub fn comments_len(&self) -> Option<usize> {
        match self.last_retrieved_comments {
            Some(ref comments) => Some(comments.len()),
            None => None,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum AppPreviousCommand {
    NoPrevious,
    ExpandedComment,
    ViewingComments,
    ViewingStories,
    OpenedStory,
    DownloadedPage,
}

pub enum AppStates {
    WaitingUserInput,
    RetrievingResults,
    DoingLocalWork,
    Idle,
    Starting,
}

pub struct AppStateMachine {
    pub connection_working: bool,
    pub listing_page_index: usize,
    pub comments_page_index: usize,
    pub last_opened_item_id: String,
    pub current_state: AppStates,
    pub previous_command: AppPreviousCommand,
}

impl AppStateMachine {
    pub fn new() -> AppStateMachine {
        AppStateMachine {
            connection_working: false,
            listing_page_index: 0,
            comments_page_index: 0,
            last_opened_item_id: String::from(""),
            current_state: AppStates::Starting,
            previous_command: AppPreviousCommand::NoPrevious,
        }
    }

    pub fn register_viewing_comments(&mut self) {
        self.previous_command = AppPreviousCommand::ViewingComments;
    }
    pub fn register_expanded_comment(&mut self) {
        self.previous_command = AppPreviousCommand::ExpandedComment;
    }
    pub fn register_viewing_stories(&mut self) {
        self.previous_command = AppPreviousCommand::ViewingStories;
    }
    pub fn register_downloaded_story(&mut self) {
        self.previous_command = AppPreviousCommand::DownloadedPage;
    }
    pub fn register_opened_story(&mut self) {
        self.previous_command = AppPreviousCommand::OpenedStory;
    }
    pub fn viewing_comments(&self) -> bool {
        self.previous_command == AppPreviousCommand::ExpandedComment || self.previous_command == AppPreviousCommand::ViewingComments
    }
    pub fn viewing_stories(&self) -> bool {
        // we can just use negation of viewing comments currently
        !self.viewing_comments()
    }
}


struct AppLogFormat;

fn configure_client(handle: &Handle) -> Client<HttpsConnector> {
    let tls_cx = TlsConnector::builder().unwrap().build().unwrap();
    let mut connector = HttpsConnector {
        tls: Arc::new(tls_cx),
        http: HttpConnector::new(4, handle),
    };
    connector.disable_enforce_http();
    Client::configure()
            .connector(connector)
            .build(handle)

}

fn initialize_loggers() -> Result<(), fern::InitError> {

    // Configure logger at runtime
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("app.log")?)
        .apply()?;
    Ok(())

}