use slog::Logger;
use cli_backend::UiCommand;

pub fn log_cmd(logger: &Logger, cmd: &UiCommand) {
    if cmd.number.is_some() {
        info!(logger,
              format!("Trying with command: {} {}",
                      cmd.command.as_ref().unwrap(),
                      cmd.number.as_ref().unwrap()));
    } else {
        info!(logger,
              format!("Trying with command: {}", cmd.command.as_ref().unwrap()));
    }
}

pub fn log_stories_page_with_index(logger: &Logger, index: i32) {
    info!(logger,
          format!("Retrieved previous stories page with index {}", index));
}

pub fn log_exit(logger: &Logger) {
    info!(logger, "Exited application normally");
}

pub fn log_open_page(logger: &Logger, url: &String) {
    info!(logger, format!("Opened page with default browser with url {}", url));
}

pub fn log_loaded_page_locally(logger: &Logger, url: &String, filename: &String) {
    info!(logger, format!("Loaded page {} to file {}", url, filename));
}

pub fn log_loaded_top_stories(logger:&Logger, length: usize) {
    info!(logger, format!("Received {} top stories", length));
}

pub fn log_response_status(logger: &Logger, url: &String, status: &String) {
    info!(logger,
          format!("Request to {} finished with status {}", url, status));
}