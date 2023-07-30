use std::env::var;

pub struct GitHub;

impl GitHub {
    pub fn is_running_in_ci() -> bool {
        return var("CI").is_ok();
    }
}
