use mongodb::bson::serde_helpers::{deserialize_hex_string_from_object_id, serialize_hex_string_as_object_id};
use serde::ser::Error;
use serde::Serializer;

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

pub fn deserialize_option_hex_string_from_object_id<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = deserialize_hex_string_from_object_id(deserializer)?;
    Ok(Some(value))
}

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
