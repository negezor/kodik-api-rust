use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{error::Error, util::serialize_into_query_parts, Client};

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
        let payload = serialize_into_query_parts(self)?;

        let response = client
            .init_post_request("/qualities")
            .query(&payload)
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
