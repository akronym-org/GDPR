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
    let count = string.matches('.').count();
    if count == 0 {
        return (string, None);
    } else if count == 1 {
        let split = string.split_once('.').unwrap();
        return (split.0, Some(split.1));
    } else {
        panic!("{} has too many points.", string)
    }
}

pub fn remove_whitespace(s: &str) -> Result<String, String> {
    let trimmed = s.split(',').map(|word| word.trim().to_owned()).collect::<Vec<String>>().join(",");
    Ok(trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_one_point_strictly_splits_one_point() {
        let s = "h.0";
        let result = split_one_point_strictly(s);
        assert_eq!(result, ("h", Some("0")));
    }

    #[test]
    fn split_one_point_strictly_splits_even_with_no_point() {
        let s = "h";
        let result = split_one_point_strictly(s);
        assert_eq!(result, ("h", None));
    }

    #[test]
    fn remove_whitespace_works() {
        let s = "h, t";
        let result = remove_whitespace(s);
        assert_eq!(result, Ok("h,t".to_owned()));
    }

    #[test]
    fn remove_whitespace_works_with_multiple_spaces() {
        let s = "h  ,t";
        let result = remove_whitespace(s);
        assert_eq!(result, Ok("h,t".to_owned()));
    }

    #[test]
    fn remove_whitespace_works_with_more_commas() {
        let s = "h, t, s, t, s?";
        let result = remove_whitespace(s);
        assert_eq!(result, Ok("h,t,s,t,s?".to_owned()));
    }

    #[test]
    fn remove_whitespace_works_doesnt_remove_inner_whitespace() {
        let s = "h h, ?";
        let result = remove_whitespace(s);
        assert_eq!(result, Ok("h h,?".to_owned()));
    }
}
