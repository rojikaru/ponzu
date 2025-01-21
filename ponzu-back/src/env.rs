use std::fmt::Debug;
use std::str::FromStr;

pub fn get_from_env<T: FromStr>(key: &str, default: Option<&str>) -> T
where
    <T as FromStr>::Err: Debug,
{
    // Get the unparsed value from the environment
    let unparsed = match default {
        Some(default) => std::env::var(key).unwrap_or(default.to_string()),
        None => std::env::var(key).unwrap(),
    };
    // Return the parsed value
    unparsed.parse::<T>().unwrap()
}
