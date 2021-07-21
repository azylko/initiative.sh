pub use app::AppCommand;
pub use runnable::{autocomplete_phrase, Runnable};

mod app;
mod runnable;

use super::Context;
use crate::storage::StorageCommand;
use crate::world::WorldCommand;
use rand::Rng;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    App(AppCommand),
    // Context(ContextCommand),
    World(WorldCommand),
    Storage(StorageCommand),
}

impl Command {
    pub fn run(&self, context: &mut Context, rng: &mut impl Rng) -> String {
        match self {
            Self::App(c) => c.run(context),
            Self::Storage(c) => c.run(context),
            Self::World(c) => c.run(context, rng),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(command) = raw.parse() {
            Ok(Command::App(command))
        } else if let Ok(command) = raw.parse() {
            Ok(Command::Storage(command))
        } else if let Ok(command) = raw.parse() {
            Ok(Command::World(command))
        } else {
            Err(())
        }
    }
}

impl Runnable for Command {
    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Command)> {
        let mut suggestions = Vec::new();
        let mut inputs = 0;
        let mut append = |mut cmd_suggestions: Vec<(String, Command)>| {
            if !cmd_suggestions.is_empty() {
                inputs += 1;
                suggestions.append(&mut cmd_suggestions);
            }
        };

        append(AppCommand::autocomplete(input, context));
        append(StorageCommand::autocomplete(input, context));
        append(WorldCommand::autocomplete(input, context));

        // No need to re-sort and truncate if we've only received suggestions from one command.
        if inputs > 1 {
            suggestions.sort_by(|(a, _), (b, _)| a.cmp(b));
            suggestions.truncate(10);
        }

        suggestions
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::Species;

    #[test]
    fn from_str_test() {
        {
            let result = "debug".parse();
            assert!(
                matches!(result, Ok(Command::App(AppCommand::Debug))),
                "{:?}",
                result,
            );
        }

        {
            let result = "npc".parse();
            assert!(
                matches!(
                    result,
                    Ok(Command::World(WorldCommand::Npc { species: None })),
                ),
                "{:?}",
                result,
            );
        }
    }

    #[test]
    fn autocomplete_test() {
        let results = Command::autocomplete("d", &Context::default());
        let mut result_iter = results.iter();

        if let Some((command_string, Command::App(AppCommand::Debug))) = result_iter.next() {
            assert_eq!("debug", command_string);
        } else {
            panic!("{:?}", results);
        }

        if let Some((
            command_string,
            Command::World(WorldCommand::Npc {
                species: Some(Species::Dragonborn),
            }),
        )) = result_iter.next()
        {
            assert_eq!("dragonborn", command_string);
        } else {
            panic!("{:?}", results);
        }

        if let Some((
            command_string,
            Command::World(WorldCommand::Npc {
                species: Some(Species::Dwarf),
            }),
        )) = result_iter.next()
        {
            assert_eq!("dwarf", command_string);
        } else {
            panic!("{:?}", results);
        }

        assert!(result_iter.next().is_none());
    }
}
