extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    format!("https://google.com/search?q={}", encoded_query)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let query = "rust";
        assert_eq!(
            construct_google_search_url(query),
            "https://google.com/search?q=rust"
        );
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let query = "actix rust";
        assert_eq!(
            construct_google_search_url(query),
            "https://google.com/search?q=actix%20rust"
        );
    }

}
