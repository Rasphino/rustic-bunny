pub mod google;
pub mod github;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    let index_of_whitespace = query_string.find(' ').unwrap_or(query_string.len());
    &query_string[..index_of_whitespace]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        assert_eq!(
            get_command_from_query_string("tw"),
            "tw"
        );
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        assert_eq!(
            get_command_from_query_string("tw @fbOpenSource"),
            "tw"
        );
    }
}
