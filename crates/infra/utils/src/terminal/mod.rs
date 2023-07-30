use owo_colors::{colors::BrightBlue, OwoColorize};
use terminal_size::terminal_size;

use crate::github::GitHub;

pub struct Terminal;

impl Terminal {
    /// Prints a colored visual separator to split logs from different tasks/operations.
    pub fn separator(title: impl Into<String>) {
        const DEFAULT_WIDTH: usize = 100;
        const BANNER_GLYPHS: usize = 6; // "╾┤  ├╼"

        let title = title.into();

        let terminal_width = terminal_size().map_or(DEFAULT_WIDTH, |(width, _)| width.0 as usize);
        let spacer_width = terminal_width - title.chars().count() - BANNER_GLYPHS;

        let left_spacer_width = spacer_width / 2;
        let right_spacer_width = spacer_width - left_spacer_width;

        let contents = format!(
            "{start} {middle} {end}",
            start = format!("╾{sep}┤", sep = "─".repeat(left_spacer_width)).dimmed(),
            middle = title.fg::<BrightBlue>().bold(),
            end = format!("├{sep}╼", sep = "─".repeat(right_spacer_width)).dimmed(),
        );

        println!();
        println!();
        println!("{contents}");
        println!();
        println!();
    }

    /// If running in GitHub CI, this prints special directives before/after the operation,
    /// to make sure they are collapsed in the log viewer. They can be expanded individually when needed.
    pub fn collapse_group<T>(title: impl AsRef<str>, operation: impl FnOnce() -> T) -> T {
        let title = title.as_ref();

        if GitHub::is_running_in_ci() {
            println!("::group::{title}")
        } else {
            println!();
            println!("{title}");
            println!();
        }

        let result = operation();

        if GitHub::is_running_in_ci() {
            println!("::endgroup::")
        } else {
            println!();
        }

        return result;
    }
}
