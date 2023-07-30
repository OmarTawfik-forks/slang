use anyhow::{Ok, Result};
use clap::ValueEnum;

pub trait Task: Clone + Ord + PartialEq + ValueEnum {
    fn execute(&self) -> Result<()>;

    fn execute_user_selection(tasks: &Vec<Self>) -> Result<()> {
        let mut tasks = tasks.clone();

        if tasks.is_empty() {
            // Execute all tasks if none are provided:
            tasks.extend(Self::value_variants().iter().cloned());
        } else {
            // Sort and deduplicate user provided tasks by order of definition:
            tasks.sort();
            tasks.dedup();
        }

        for task in tasks {
            task.execute()?;
        }

        return Ok(());
    }
}

pub trait ValueEnumExtensions {
    fn clap_name(&self) -> String;
}

impl<T: ValueEnum> ValueEnumExtensions for T {
    fn clap_name(&self) -> String {
        return self
            .to_possible_value()
            .expect("Expected Clap ValueEnum to have a name (not skipped).")
            .get_name()
            .to_owned();
    }
}
