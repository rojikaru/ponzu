use std::str::FromStr;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::{
    deserialize_hex_string_from_object_id, serialize_hex_string_as_object_id,
};
use serde::ser::Error;
use serde::Serializer;

/// Serializes an `Option<String>` as an `ObjectId` in hexadecimal format.
///
/// # Parameters
/// - `value`: The `Option<String>` to serialize.
/// - `serializer`: The `Serializer` to use.
///
/// # Returns
/// A `Result` containing the serialized `ObjectId` if successful, or an `Error` if the operation fails.
pub fn serialize_option_hex_string_as_object_id<S>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serialize_hex_string_as_object_id(v, serializer),
        None => Err(Error::custom("Expected a hex string, found None")),
    }
}

/// Deserializes an `Option<String>` from an `ObjectId` in hexadecimal format.
///
/// # Parameters
/// - `deserializer`: The `Deserializer` to use.
///
/// # Returns
/// A `Result` containing the deserialized `Option<String>` if successful, or an `Error` if the operation fails.
pub fn deserialize_option_hex_string_from_object_id<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = deserialize_hex_string_from_object_id(deserializer)?;
    Ok(Some(value))
}

/// Serializes an `Option<mongodb::bson::DateTime>` as an RFC 3339 string.
///
/// # Parameters
/// - `value`: The `Option<mongodb::bson::DateTime>` to serialize.
///
/// # Returns
/// A `Result` containing the serialized RFC 3339 string if successful, or an `Error` if the operation fails.
pub fn serialize_option_bson_datetime_as_rfc3339_string<S>(
    value: &Option<mongodb::bson::DateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.try_to_rfc3339_string().unwrap()),
        None => serializer.serialize_none(),
    }
}

/// Converts a string to a MongoDB `ObjectId`.
///
/// # Parameters
/// - `id`: The string representation of the `_id` field.
///
/// # Returns
/// A `Result` containing the `ObjectId` if successful, or an `Error` if the operation fails.
pub(crate) fn get_object_id(id: &str) -> Result<ObjectId, mongodb::error::Error> {
    match ObjectId::from_str(id) {
        Ok(oid) => Ok(oid),
        Err(e) => Err(mongodb::error::Error::custom(format!("Invalid ObjectId: {}", e))),
    }
}
