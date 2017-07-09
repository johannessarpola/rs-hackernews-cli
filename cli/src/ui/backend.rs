use utils;
use io;

pub struct UiCommand {
    pub command: Option<String>,
    pub number: Option<usize>,
    pub extra_args: Option<Vec<String>>,
    pub valid: bool,
}

impl UiCommand {
    pub fn parse(msg: Result<String, io::Error>) -> Option<UiCommand> {
        match msg {
            Ok(text) => {
                let mut parts = text.split_whitespace().collect::<Vec<_>>().into_iter();
                let command = Some(parts.next().unwrap_or("invalid").to_owned()); // todo will cause error if it's empty probably
                let number = utils::try_to_parse_number(parts.next());
                let mut extra_args = parts.map(|s| String::from(s)).collect::<Vec<_>>();

                // todo check that command is in dict

                Some(UiCommand {
                    command: command,
                    number: number,
                    extra_args: Some(extra_args),
                    valid: true,
                })
            }
            Err(ref e) => None, // todo wrong command
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_command_parsing(){
        let cmd:UiCommand = UiCommand::parse(Ok("comments 10".to_owned())).unwrap();
        assert_eq!("comments", cmd.command.unwrap());
        assert_eq!(10, cmd.number.unwrap());

        let cmd2:UiCommand = UiCommand::parse(Ok("exit".to_owned())).unwrap();
        assert_eq!("exit", cmd2.command.unwrap());
        assert!(cmd2.number.is_none());

        let cmd3:UiCommand = UiCommand::parse(Ok("open 10 abc".to_owned())).unwrap();
        assert_eq!("abc", cmd3.extra_args.unwrap()[0]);
    }
}