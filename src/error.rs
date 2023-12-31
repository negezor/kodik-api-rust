use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("HTTP request failed: {}", .0)]
    HttpError(reqwest::Error),
    #[error("Error urlencoded serialize: {}", .0)]
    UrlencodedSerializeError(comma_serde_urlencoded::ser::Error),
    #[error("Error urlencoded deserialize: {}", .0)]
    UrlencodedDeserializeError(comma_serde_urlencoded::de::Error),

    #[error("Kodik error: {}", .0)]
    KodikError(String),
}
