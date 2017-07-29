use slog::Logger;
use ui::backend::UiCommand;

pub fn log_no_connection(logger: &Logger) {
    warn!(logger, "No internet connection");
}
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

pub fn log_written_file(logger: &Logger, success:bool, filename:&str) {
        if success {             
            info!(logger, format!("Written file with name {}", filename));
        }
        else {
            warn!(logger, format!("Failed to write file with name {}", filename));
        }
}

pub fn log_stories_page_with_index(logger: &Logger, index: usize) {
    info!(logger,
          format!("Retrieved previous stories page with index {}", index));
}

pub fn log_invalid_state(logger: &Logger) {
    warn!(logger, "Application went into invalid state");
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