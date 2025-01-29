use std::fmt::Debug;
use std::str::FromStr;

/// Get a value from the environment
///
/// # Arguments
/// `key` - The key of the environment variable
/// `default` - The default value if the key is not found
///
/// # Returns
/// The value of the environment variable as the specified type `T`
pub fn get_from_env<T: FromStr>(key: &str, default: Option<&str>) -> T
where
    <T as FromStr>::Err: Debug,
{
    // Get the unparsed value from the environment
    let unparsed = match default {
        Some(default) => std::env::var(key).unwrap_or(default.to_string()),
        None => std::env::var(key).expect(&format!("{} environment variable not found", key)),
    };
    // Return the parsed value
    unparsed
        .parse::<T>()
        .expect(&format!("Failed to parse {} as {:?}", key, T::from_str("")))
}
