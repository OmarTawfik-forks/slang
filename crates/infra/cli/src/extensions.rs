use anyhow::{Ok, Result};
use clap::ValueEnum;

pub trait OrderedCommand: Clone + Ord + PartialEq + ValueEnum {
    fn execute(&self) -> Result<()>;

    fn execute_all(commands: &Vec<Self>) -> Result<()> {
        let mut commands = commands.clone();

        if commands.is_empty() {
            // Execute all tasks if none are provided:
            commands.extend(Self::value_variants().iter().cloned());
        } else {
            // Sort and deduplicate user provided tasks by order of definition:
            commands.sort();
            commands.dedup();
        }

        for command in commands {
            command.execute()?;
        }

        return Ok(());
    }
}

pub trait ClapExtensions {
    fn clap_name(&self) -> String;
}

impl<T: ValueEnum> ClapExtensions for T {
    fn clap_name(&self) -> String {
        return self
            .to_possible_value()
            .expect("Expected Clap ValueEnum to have a name (not skipped).")
            .get_name()
            .to_owned();
    }
}
