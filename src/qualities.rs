use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{error::Error, Client};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualityResult {
    // Name of quality
    pub title: String,
}

/// A struct containing qualities results
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum QualityResponseUnion {
    Result(Vec<QualityResult>),
    Error { error: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QualitySort {
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "count")]
    Count,
}

#[derive(Debug, Serialize, Clone)]
pub struct QualityQuery<'a>(PhantomData<&'a i32>);

impl<'a> QualityQuery<'a> {
    pub fn new() -> QualityQuery<'a> {
        QualityQuery(PhantomData)
    }

    /// Execute the query and fetch the results.
    pub async fn execute<'b>(&'a self, client: &'b Client) -> Result<Vec<QualityResult>, Error> {
        let body =
            comma_serde_urlencoded::to_string(self).map_err(Error::UrlencodedSerializeError)?;

        let response = client
            .init_post_request("/qualities")
            .body(body)
            .send()
            .await
            .map_err(Error::HttpError)?;

        let result = response
            .json::<QualityResponseUnion>()
            .await
            .map_err(Error::HttpError)?;

        match result {
            QualityResponseUnion::Result(result) => Ok(result),
            QualityResponseUnion::Error { error } => Err(Error::KodikError(error)),
        }
    }
}

impl<'a> Default for QualityQuery<'a> {
    fn default() -> Self {
        Self::new()
    }
}
