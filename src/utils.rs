use std::collections::HashMap;

/// Split a string with at most one point `.`
///
/// Always return a tuple. If no point is found, return a string splice
/// in the first part of the tuple.
/// Panic if there's more than one point.
///
/// # Arguments
/// 
/// * `string` - A string splice that holds first_part.second_part
///
/// # Example
/// ```
/// let a_string = "some_funny.business".to_string();
/// let split = split_one_point_strictly(a_string);
/// assert_eq!(split, ("some_funny", Some("business")));
/// ```
pub fn split_one_point_strictly(string: &str) -> (&str, Option<&str>) {
    let count = string.matches(".").count();
    if count == 0 {
        return (string, None);
    } else if count == 1 {
        let split = string.split_once(".").unwrap();
        return (split.0, Some(split.1));
    } else {
        panic!("{} has too many points.", string)
    }
}

enum Bound {
    Prefix,
    Suffix,
}

pub struct StringBounds;

impl StringBounds {
    fn longest_common_prefix(s1: &str, s2: &str) -> String {
        StringBounds::longest_common_bound(s1, s2, Bound::Prefix)
    }

    fn longest_common_suffix(s1: &str, s2: &str) -> String {
        StringBounds::longest_common_bound(s1, s2, Bound::Suffix)
    }

    fn longest_common_bound(s1: &str, s2: &str, bound: Bound) -> String {
        let (s1_str, s2_str) = match bound {
                Bound::Prefix => (s1.to_owned(), s2.to_owned()),
                Bound::Suffix => (s1.chars().rev().collect::<String>(), s2.chars().rev().collect::<String>()),
            };

        let result = s1_str
            .chars()
            .zip(s2_str.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .map(|(c1, _)| c1)
            .collect::<String>();

        match bound {
            Bound::Prefix => result,
            Bound::Suffix => result.chars().rev().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_prefix_returns_head() {
        let s1 = "head:0";
        let s2 = "head:xx";
        let result = StringBounds::longest_common_prefix(s1, s2);
        assert_eq!(result, "head:");
    }

    #[test]
    fn longest_common_prefix_returns_he() {
        let s1 = "head:0";
        let s2 = "he:ad:0";
        let result = StringBounds::longest_common_prefix(s1, s2);
        assert_eq!(result, "he");
    }

    #[test]
    fn longest_common_prefix_returns_empty() {
        let s1 = "shear:0";
        let s2 = "fear:0";
        let result = StringBounds::longest_common_prefix(s1, s2);
        assert_eq!(result, "");
    }

    #[test]
    fn longest_common_suffix_returns_tail() {
        let s1 = "0:tail";
        let s2 = "xx:tail";
        let result = StringBounds::longest_common_suffix(s1, s2);
        assert_eq!(result, ":tail");
    }
}
