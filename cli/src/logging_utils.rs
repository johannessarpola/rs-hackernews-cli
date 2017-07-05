use slog::Logger;
use cli_backend::UiCommand;

pub fn log_cmd(logger:&Logger, cmd:&UiCommand) {
    if cmd.number.is_some() {
        info!(logger,
          format!("Trying with command: {} {}",
                  cmd.command.as_ref().unwrap(),
                  cmd.number.as_ref().unwrap()));
    }
    else {
        info!(logger, format!("Trying with command: {}", cmd.command.as_ref().unwrap()));
    }
}