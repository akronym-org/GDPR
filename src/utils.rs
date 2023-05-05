/// Split a string with at most one point `.`
///
/// Always return a tuple. If no point is found, return a string splice
/// in the first part of the tuple.
/// Panic if there's more than one point.
///
/// # Arguments
/// * `string` - A string splice that holds first_part.second_part
///
/// # Example
/// ```
/// let a_string = "some_funny.business".to_string();
/// let split = split_one_point_strictly(a_string);
/// assert_eq!(, b);
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
