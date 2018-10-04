use ui::backend::UiCommand;

pub fn log_no_connection() {
    warn!( "No internet connection");
}

pub fn log_cmd(cmd: &UiCommand) {
    if cmd.number.is_some() {
        info!("Trying with command: {} {}",
                      cmd.command.as_ref().unwrap(),
                      cmd.number.as_ref().unwrap());
    } else {
        info!("Trying with command: {}", cmd.command.as_ref().unwrap());
    }
}

pub fn log_written_file(success:bool, filename:&str) {
        if success {
            info!("Written file with name {}", filename);
        }
        else {
            warn!("Failed to write file with name {}", filename);
        }
}

pub fn log_stories_page_with_index(index: usize) {
    info!("Retrieved previous stories page with index {}", index);
}

pub fn log_invalid_state() {
    warn!("Application went into invalid state");
}

pub fn log_exit() {
    info!("Exited application normally");
}

pub fn log_open_page(url: &String) {
    info!("Opened page with default browser with url {}", url);
}

pub fn log_loaded_page_locally(url: &String, filename: &String) {
    info!("Loaded page {} to file {}", url, filename);
}

pub fn log_loaded_top_stories(length: usize) {
    info!("Received {} top stories", length);
}

pub fn log_response_status(url: &String, status: &String) {
    info!("Request to {} finished with status {}", url, status);
}