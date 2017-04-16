use tokio_core::reactor::{Core, Handle};
use hyper::Client;
use hyper_tls::HttpsConnector;
use endpoint::HnNewsEndpoint;
use slog;
use slog_term;
use slog_stream;
use slog::{Level, LevelFilter, DrainExt};
use std::fs::OpenOptions;
use std::io;

///
/// 'AppDomain' struct which have relevant parts which are use as core elements of the application
///
pub struct AppDomain {
    pub core: Core,
    pub endpoint: HnNewsEndpoint,
    pub client: Client<HttpsConnector>,
    pub logger: slog::Logger,
}

enum AppStates {
    WaitingUserInput,
    RetrievingResults,
    DoingLocalWork,
    Idle,
}

struct AppStateMachine {
    pub viewing_top_stories:bool, 
    pub viewing_comments_for_a_story:bool,
    pub connection_working:bool,
    pub top_story_page_index:i32,
    pub comments_page_index:i32,
    pub current_state: AppStates,
}

struct AppLogFormat;

impl slog_stream::Format for AppLogFormat {
    fn format(&self,
              io: &mut io::Write,
              rinfo: &slog::Record,
              _logger_values: &slog::OwnedKeyValueList)
              -> io::Result<()> {
        let msg = format!("{} {} from line {} in {}\n", rinfo.level(), rinfo.msg(), rinfo.line(), rinfo.file());
        let _ = try!(io.write_all(msg.as_bytes()));
        Ok(())
    }
}

fn configure_client(handle: &Handle) -> Client<HttpsConnector> {
    Client::configure()
            // Does not check the validity of certificate
            .connector(HttpsConnector::new(4, &handle))
            .build(&handle)
}

pub fn create_app_domain() -> AppDomain {
    let logger = create_loggers();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = configure_client(&handle);
    let endpoint = HnNewsEndpoint::build_default();
    let mut app_domain = AppDomain {
        core: core,
        endpoint: endpoint,
        client: client,
        logger: logger,
    };
    app_domain
}
fn create_loggers() -> slog::Logger {
        let file = OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(true)
        .open("app.log").unwrap();
    let file_drain = slog_stream::stream(file, AppLogFormat);
    let std_out_drain = slog_term::streamer().build();
    // let logger = slog::Logger::root(slog::duplicate(console_drain, file_drain).fuse(), o!());
    let logger = slog::Logger::root(
        slog::Duplicate::new(
            LevelFilter::new(file_drain, Level::Info), 
            LevelFilter::new(std_out_drain, Level::Warning)
        ).fuse(), o!());
    logger
}