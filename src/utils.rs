/// Split a string with maximum one point `.`. Always returns a tuple.
///
/// # Arguments
/// * `string` - A string splice that holds first_part.second_part
///
/// If no point found, returns string splice in first part of tuple.
///
/// Panics if more than one point is found.
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
