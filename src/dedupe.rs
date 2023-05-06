//! Tools and functions to help deduplicate data
//! To display many data points in yamls to users more nicely.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_strings_by_wildcard_returns_expected_prefixes() {
        let test_me = vec!["base:0", "base:xx", "base:6"];
        let result = build_glob_like_patterns(test_me);
        assert_eq!(result, vec!["base:*"])
    }

    // #[test]
    // fn group_strings_by_wildcard_returns_expected_with_prefix_and_suffix() {
    //     let test_me = vec!["base:0", "base:xx", "base:6", "other_2", "other_3", "other2", "something:end", "else:end", "bla:end", "different", "last"];
    //     let result = group_strings_by_wildcard(test_me);
    //     assert_eq!(result, vec!["base:*", "other_*", "other2", "*:end", "different", "last"])
    // }
}
