use crate::project_constants::PROJECT_VARIANTS;

mod project_constants;

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

pub fn _core_version_details() -> &'static [&'static str] {
    return PROJECT_VARIANTS;
}
