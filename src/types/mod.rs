//! Note, these types were hand crafted based on
//! <https://developers.squarespace.com/commerce-apis>.

pub mod orders;
pub mod pagination;
pub mod products;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rstest::fixture;

    #[fixture]
    pub(crate) fn test_responses_path() -> PathBuf {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("test_responses");
        file_path
    }
}
