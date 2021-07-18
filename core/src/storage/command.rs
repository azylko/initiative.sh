use crate::app::Context;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Load { query: String },
}

impl Command {
    pub fn run(&self, context: &mut Context) -> String {
        match self {
            Self::Load { query } => {
                let lowercase_query = query.to_lowercase();
                if let Some(result) = context.recent().iter().find(|t| {
                    t.name()
                        .value()
                        .map_or(false, |s| s.to_lowercase() == lowercase_query)
                }) {
                    format!("{}", result.display_details())
                } else {
                    format!("No matches for \"{}\"", query)
                }
            }
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.starts_with(char::is_uppercase) {
            Ok(Self::Load {
                query: raw.to_string(),
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_test() {
        let parsed_command = "Gandalf the Grey".parse();
        if let Ok(Command::Load { query }) = parsed_command {
            assert_eq!("Gandalf the Grey", query.as_str());
        } else {
            panic!("{:?}", parsed_command);
        }

        let parsed_command = "potato".parse::<Command>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }
}
