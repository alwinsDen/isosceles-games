use crate::{
    project_constants::{GIT_REPO_SSH, PROJECT_VARIANTS},
    utils::simple_utils::simple_command_runner,
};

mod project_constants;
mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        println!("TEST");
        assert_eq!(result, 4);
    }
}

pub fn __core_version_details() -> &'static [&'static str] {
    return PROJECT_VARIANTS;
}

pub fn __core_version() -> &'static str {
    return env!("CARGO_PKG_VERSION");
}

pub fn __core_setup() -> Result<(), ()> {
    let _ = simple_command_runner("git", &["--version"]);
    _ = simple_command_runner("git", &["clone", "--progress", GIT_REPO_SSH]);
    Ok(())
}
