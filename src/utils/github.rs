extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        "https://github.com".to_string()
    } else {
        // TODO: Assume the other match is "gh page"
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        format!("https://github.com/{}", encoded_query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_url_with_gh() {
        let query = "gh";
        assert_eq!(
            construct_github_url(query),
            "https://github.com"
        )
    }

    #[test]
    fn test_construct_github_url_with_repo_url() {
        let query = "gh rust-lang/rust";
        assert_eq!(
            construct_github_url(query),
            "https://github.com/rust-lang/rust"
        )
    }
}
