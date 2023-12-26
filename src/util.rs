use serde::ser;

use crate::error::Error;

pub fn serialize_into_query_parts<T: ser::Serialize>(
    input: T,
) -> Result<Vec<(String, String)>, Error> {
    let serialized =
        comma_serde_urlencoded::to_string(input).map_err(Error::UrlencodedSerializeError)?;

    let parts =
        comma_serde_urlencoded::from_str(&serialized).map_err(Error::UrlencodedDeserializeError)?;

    Ok(parts)
}
