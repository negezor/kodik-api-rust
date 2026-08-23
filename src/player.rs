use serde::{Deserialize, Serialize};

use crate::{
    Client,
    error::Error,
    util::serialize_into_query_parts,
};

/// Response returned by `/get-player`.
#[derive(Deserialize, Debug, Clone)]
pub struct PlayerResponse {
    pub found: bool,
    pub allowed: i32,
    pub quality: Option<String>,
    pub translation: Option<String>,
    pub link: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum PlayerApiResponse {
    Result(PlayerResponse),
    Error { error: String },
}

#[derive(Debug, Serialize, Clone)]
pub struct PlayerQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    #[serde(
        rename = "hasPlayer",
        skip_serializing_if = "Option::is_none"
    )]
    has_player: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,

    #[serde(
        rename = "ID",
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<&'a str>,
}

impl<'a> PlayerQuery<'a> {
    pub fn new() -> PlayerQuery<'a> {
        PlayerQuery {
            title: None,
            has_player: None,
            url: None,
            id: None,
        }
    }

    pub fn with_title<'b>(
        &'b mut self,
        title: &'a str,
    ) -> &'b mut PlayerQuery<'a> {
        self.title = Some(title);
        self
    }

    pub fn with_has_player<'b>(
        &'b mut self,
        has_player: bool,
    ) -> &'b mut PlayerQuery<'a> {
        self.has_player = Some(has_player);
        self
    }

    pub fn with_url<'b>(
        &'b mut self,
        url: &'a str,
    ) -> &'b mut PlayerQuery<'a> {
        self.url = Some(url);
        self
    }

    pub fn with_id<'b>(
        &'b mut self,
        id: &'a str,
    ) -> &'b mut PlayerQuery<'a> {
        self.id = Some(id);
        self
    }

    /// Execute the query and fetch the player.
    pub async fn execute<'b>(
        &'a self,
        client: &'b Client,
    ) -> Result<PlayerResponse, Error> {
        let payload = serialize_into_query_parts(self)?;

        let response = client
            .init_get_request("/get-player")
            .query(&payload)
            .send()
            .await
            .map_err(Error::HttpError)?;

        let result = response
            .json::<PlayerApiResponse>()
            .await
            .map_err(Error::HttpError)?;

        match result {
            PlayerApiResponse::Result(result) => Ok(result),
            PlayerApiResponse::Error { error } => {
                Err(Error::KodikError(error))
            }
        }
    }
}

impl<'a> Default for PlayerQuery<'a> {
    fn default() -> Self {
        Self::new()
    }
}
