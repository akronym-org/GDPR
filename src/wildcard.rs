//! Tools and functions to help deduplicate data
//! To display many data points in yamls to users more nicely.
//!
use regex::Regex;

/// Simplify a series of strings into a short list of glob-like patterns.
///
/// Shorten and compress a series of strings with similar prefixes and suffixes
/// by using wildcards (*).
/// For each group, there can only ever be one wildcard at the start or the end.
/// Prioritize putting the wildcard at the end and prefer longer matches over
/// shorter matches.
///
/// # Arguments
///
/// * `strings` - A vector of strings.
///
/// # Example
/// ```
/// let pattern = build_glob_patterns(vec!["glob:", "glob:2", "no-glob:xx"])
/// assert_eq!(pattern, vec!["glob:*", "no-glob:xx"])
/// ```
///
/// Works also with suffix matches:
///
/// ```
/// let pattern = build_glob_patterns(vec!["2:glob", "3:glob", "xx:no-glob"])
/// assert_eq!(pattern, vec!["*:glob", "xx:no-glob"])
/// ```
///
/// Prefers longer matches and prefers prefix matches:
///
/// ```
/// let pattern = build_glob_patterns(vec!["prefer_prefix_glob:xx", "prefer_prefix_glob:xxx", "prefer_prefix_glob_xxx"])
/// assert_eq!(pattern, vec!["prefer_prefix_glob*"])
/// ```
///
/// Sorts all strings, so the result is deterministic:
///
/// ```
/// let pattern = build_glob_patterns(vec!["prefer_prefix_glob:xx", "random_glob_3x", "prefer_prefix_?_glob_3x", "prefer_prefix_glob_3x"])
/// assert_eq!(pattern, vec!["random_glob_3x", "prefer_prefix_*"])
/// ```
pub fn build_glob_like_patterns(strings: Vec<&str>) -> Vec<String> {
    todo!();
}

/// Find all matching entries in a vector that satisfy an expression.
///
/// # Arguments
///
/// * `expression` - a string with a valid wildcard (at the start or end)
/// * `total_set` - a vec of strings from which to match the expression
///
/// # Returns
///
/// A vec of strings that matches the expression.
pub fn find_with(expression: &str, total_set: &Vec<String>) -> Vec<String> {
    if !expression.contains('*') {
        return vec![expression.to_owned()];
    }

    let expression = r"^".to_owned() + &expression.replace('*', ".*") + "$";
    let re = Regex::new(&expression).unwrap();
    let mut result: Vec<String> = vec![];
    for item in total_set {
        if re.is_match(&item) {
            result.push(item.to_owned());
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_strings_by_wildcard_returns_expected_prefixes() {
        let test_me = vec!["base:0", "base:xx", "base:6"];
        let result = build_glob_like_patterns(test_me);
        assert_eq!(result, vec!["base:*"])
    }

    #[test]
    fn find_with_matches_wildcard_at_end() {
        let test_me = vec!["word:take".to_owned(), "words:avoid".to_owned(), "sword:avoid".to_owned()];
        let result = find_with("word:*", &test_me);
        assert_eq!(result, vec!["word:take"])
    }

    #[test]
    fn find_with_matches_wildcard_at_start() {
        let test_me = vec!["t_word".to_owned(), "f_words".to_owned(), "f_sword".to_owned()];
        let result = find_with("*_word", &test_me);
        assert_eq!(result, vec!["t_word"])
    }

    #[test]
    fn find_with_matches_start_and_end() {
        let test_me = vec!["apy".to_owned(), "pt)".to_owned(), " p?".to_owned(), "no".to_owned()];
        let result = find_with("*p*", &test_me);
        assert_eq!(result, vec!["apy", "pt)", " p?"])
    }

    #[test]
    fn find_with_matches_multiple_wildcards() {
        let test_me = vec!["y_t_t".to_owned(), "y:t:t".to_owned(), "n:t:t".to_owned(), "y_sts_xt".to_owned()];
        let result = find_with("y*t*t", &test_me);
        assert_eq!(result, vec!["y_t_t", "y:t:t", "y_sts_xt"])
    }

    #[test]
    fn find_with_matches_with_point_literals() {
        let test_me = vec!["false".to_owned(), "true.".to_owned()];
        let result = find_with("*e.*", &test_me);
        assert_eq!(result, vec!["true."])
    }
}
