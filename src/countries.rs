use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    types::{
        AllStatus, AnimeKind, AnimeStatus, DramaStatus, MaterialDataField, MppaRating, ReleaseType,
        TranslationType,
    },
    util::serialize_into_query_parts,
    Client,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountryResult {
    // Name of the country
    pub title: String,

    /// The number of materials with this voice acting
    pub count: i32,
}

/// A struct containing countries results and other information about the countries
#[derive(Deserialize, Debug, Clone)]
pub struct CountryResponse {
    pub time: String,
    pub total: i32,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
    pub results: Vec<CountryResult>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum CountryResponseUnion {
    Result(CountryResponse),
    Error { error: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CountrySort {
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "count")]
    Count,
}

#[derive(Debug, Serialize, Clone)]
pub struct CountryQuery<'a> {
    /// What field to sort materials by
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<CountrySort>,

    /// Maximum number of outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<&'a [ReleaseType]>,

    ///Filter materials by year If you set this parameter, only materials of the corresponding year will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<&'a [u32]>,

    /// Filtering materials by translation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    translation_id: Option<&'a [u32]>,
    /// Filter content by translation type. Allows you to output only voice translation or only subtitles
    #[serde(skip_serializing_if = "Option::is_none")]
    translation_type: Option<&'a [TranslationType]>,

    /// Filtering materials based on the presence of a specific field. Materials that have at least one of the listed fields are shown. In order to show only materials that have all the listed fields
    #[serde(skip_serializing_if = "Option::is_none")]
    has_field: Option<&'a [MaterialDataField]>,
    /// Filtering materials based on the presence of a specific field. Materials that have all the listed fields are shown
    #[serde(skip_serializing_if = "Option::is_none")]
    has_field_and: Option<&'a [MaterialDataField]>,

    /// Filtering materials by country. You can specify a single value or multiple values, separated by commas (then materials with at least one of the listed countries will be displayed). The parameter is case sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    countries: Option<&'a [&'a str]>,

    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    genres: Option<&'a [&'a str]>,
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    anime_genres: Option<&'a [&'a str]>,
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    drama_genres: Option<&'a [&'a str]>,
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    all_genres: Option<&'a [&'a str]>,

    /// Filtering by duration (in minutes). You can specify either a single value to search for the exact duration, or an interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<&'a [&'a str]>,

    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    #[serde(skip_serializing_if = "Option::is_none")]
    kinopoisk_rating: Option<&'a [&'a str]>,
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    #[serde(skip_serializing_if = "Option::is_none")]
    imdb_rating: Option<&'a [&'a str]>,
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    #[serde(skip_serializing_if = "Option::is_none")]
    shikimori_rating: Option<&'a [&'a str]>,
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    #[serde(skip_serializing_if = "Option::is_none")]
    mydramalist_rating: Option<&'a [&'a str]>,

    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    actors: Option<&'a [&'a str]>,
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    directors: Option<&'a [&'a str]>,
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    producers: Option<&'a [&'a str]>,
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    writers: Option<&'a [&'a str]>,
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    composers: Option<&'a [&'a str]>,
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    editors: Option<&'a [&'a str]>,
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    designers: Option<&'a [&'a str]>,
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    #[serde(skip_serializing_if = "Option::is_none")]
    operators: Option<&'a [&'a str]>,

    /// Filtering materials by age rating. You can specify a single value or multiple values, separated by commas. The parameter is case-insensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    rating_mpaa: Option<&'a [MppaRating]>,

    /// Filter content by the minimum age from which it can be viewed. You can specify either a single value or a range of values
    #[serde(skip_serializing_if = "Option::is_none")]
    minimal_age: Option<&'a [&'a str]>,

    /// Filtering materials by anime type. You can specify one value or several values separated by commas (then materials with at least one of these types will be displayed)
    #[serde(skip_serializing_if = "Option::is_none")]
    anime_kind: Option<&'a [AnimeKind]>,

    /// Filters materials by MyDramaList tags. You can specify one value or several values separated by commas (then materials with at least one of these types will be displayed)
    #[serde(skip_serializing_if = "Option::is_none")]
    mydramalist_tags: Option<&'a [&'a str]>,

    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    #[serde(skip_serializing_if = "Option::is_none")]
    anime_status: Option<&'a [AnimeStatus]>,
    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    #[serde(skip_serializing_if = "Option::is_none")]
    drama_status: Option<&'a [DramaStatus]>,
    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    #[serde(skip_serializing_if = "Option::is_none")]
    all_status: Option<&'a [AllStatus]>,

    /// Filtering materials by anime studio. You can specify either one value or several values separated by commas (then materials with at least one of the listed studios will be displayed)
    #[serde(skip_serializing_if = "Option::is_none")]
    anime_studios: Option<&'a [&'a str]>,
    /// Filtering materials by license owner. You can specify a single value or several values separated by commas (then materials that have at least one of the listed owners will be displayed)
    #[serde(skip_serializing_if = "Option::is_none")]
    anime_licensed_by: Option<&'a [&'a str]>,
}

impl<'a> CountryQuery<'a> {
    pub fn new() -> CountryQuery<'a> {
        CountryQuery {
            sort: None,
            types: None,
            year: None,
            translation_id: None,
            translation_type: None,
            has_field: None,
            has_field_and: None,
            countries: None,
            genres: None,
            anime_genres: None,
            drama_genres: None,
            all_genres: None,
            duration: None,
            kinopoisk_rating: None,
            imdb_rating: None,
            shikimori_rating: None,
            mydramalist_rating: None,
            actors: None,
            directors: None,
            producers: None,
            writers: None,
            composers: None,
            editors: None,
            designers: None,
            operators: None,
            rating_mpaa: None,
            minimal_age: None,
            anime_kind: None,
            mydramalist_tags: None,
            anime_status: None,
            drama_status: None,
            all_status: None,
            anime_studios: None,
            anime_licensed_by: None,
        }
    }

    /// Maximum number of outputs
    pub fn with_types<'b>(&'b mut self, types: &'a [ReleaseType]) -> &'b mut CountryQuery<'a> {
        self.types = Some(types);
        self
    }

    ///Filter materials by year If you set this parameter, only materials of the corresponding year will be displayed
    pub fn with_year<'b>(&'b mut self, year: &'a [u32]) -> &'b mut CountryQuery<'a> {
        self.year = Some(year);
        self
    }

    /// Filtering materials by translation ID
    pub fn with_translation_id<'b>(
        &'b mut self,
        translation_id: &'a [u32],
    ) -> &'b mut CountryQuery<'a> {
        self.translation_id = Some(translation_id);
        self
    }
    /// Filter content by translation type. Allows you to output only voice translation or only subtitles
    pub fn with_translation_type<'b>(
        &'b mut self,
        translation_type: &'a [TranslationType],
    ) -> &'b mut CountryQuery<'a> {
        self.translation_type = Some(translation_type);
        self
    }

    /// Filtering materials based on the presence of a specific field. Materials that have at least one of the listed fields are shown. In order to show only materials that have all the listed fields
    pub fn with_has_field<'b>(
        &'b mut self,
        has_field: &'a [MaterialDataField],
    ) -> &'b mut CountryQuery<'a> {
        self.has_field = Some(has_field);
        self
    }
    /// Filtering materials based on the presence of a specific field. Materials that have all the listed fields are shown
    pub fn with_has_field_and<'b>(
        &'b mut self,
        has_field: &'a [MaterialDataField],
    ) -> &'b mut CountryQuery<'a> {
        self.has_field_and = Some(has_field);
        self
    }

    /// Filtering materials by country. You can specify a single value or multiple values, separated by commas (then materials with at least one of the listed countries will be displayed). The parameter is case sensitive
    pub fn with_countries<'b>(&'b mut self, countries: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.countries = Some(countries);
        self
    }

    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_genres<'b>(&'b mut self, genres: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.genres = Some(genres);
        self
    }
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_anime_genres<'b>(
        &'b mut self,
        anime_genres: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.anime_genres = Some(anime_genres);
        self
    }
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_drama_genres<'b>(
        &'b mut self,
        drama_genres: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.drama_genres = Some(drama_genres);
        self
    }
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_all_genres<'b>(
        &'b mut self,
        all_genres: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.all_genres = Some(all_genres);
        self
    }

    /// Filtering by duration (in minutes). You can specify either a single value to search for the exact duration, or an interval.
    pub fn with_duration<'b>(&'b mut self, duration: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.duration = Some(duration);
        self
    }

    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_kinopoisk_rating<'b>(
        &'b mut self,
        kinopoisk_rating: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.kinopoisk_rating = Some(kinopoisk_rating);
        self
    }
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_imdb_rating<'b>(
        &'b mut self,
        imdb_rating: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.imdb_rating = Some(imdb_rating);
        self
    }
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_shikimori_rating<'b>(
        &'b mut self,
        shikimori_rating: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.shikimori_rating = Some(shikimori_rating);
        self
    }
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_mydramalist_rating<'b>(
        &'b mut self,
        mydramalist_rating: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.mydramalist_rating = Some(mydramalist_rating);
        self
    }

    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_actors<'b>(&'b mut self, actors: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.actors = Some(actors);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_directors<'b>(&'b mut self, directors: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.directors = Some(directors);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_producers<'b>(&'b mut self, producers: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.producers = Some(producers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_writers<'b>(&'b mut self, writers: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.writers = Some(writers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_composers<'b>(&'b mut self, composers: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.composers = Some(composers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_editors<'b>(&'b mut self, editors: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.editors = Some(editors);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_designers<'b>(&'b mut self, designers: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.designers = Some(designers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_operators<'b>(&'b mut self, operators: &'a [&'a str]) -> &'b mut CountryQuery<'a> {
        self.operators = Some(operators);
        self
    }

    /// Filtering materials by age rating. You can specify a single value or multiple values, separated by commas. The parameter is case-insensitive
    pub fn with_rating_mpaa<'b>(
        &'b mut self,
        rating_mpaa: &'a [MppaRating],
    ) -> &'b mut CountryQuery<'a> {
        self.rating_mpaa = Some(rating_mpaa);
        self
    }

    /// Filter content by the minimum age from which it can be viewed. You can specify either a single value or a range of values
    pub fn with_minimal_age<'b>(
        &'b mut self,
        minimal_age: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.minimal_age = Some(minimal_age);
        self
    }

    /// Filtering materials by anime type. You can specify one value or several values separated by commas (then materials with at least one of these types will be displayed)
    pub fn with_anime_kind<'b>(
        &'b mut self,
        anime_kind: &'a [AnimeKind],
    ) -> &'b mut CountryQuery<'a> {
        self.anime_kind = Some(anime_kind);
        self
    }

    /// Filters materials by MyDramaList tags. You can specify one value or several values separated by commas (then materials with at least one of these types will be displayed)
    pub fn with_mydramalist_tags<'b>(
        &'b mut self,
        mydramalist_tags: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.mydramalist_tags = Some(mydramalist_tags);
        self
    }

    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    pub fn with_anime_status<'b>(
        &'b mut self,
        anime_status: &'a [AnimeStatus],
    ) -> &'b mut CountryQuery<'a> {
        self.anime_status = Some(anime_status);
        self
    }
    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    pub fn with_drama_status<'b>(
        &'b mut self,
        drama_status: &'a [DramaStatus],
    ) -> &'b mut CountryQuery<'a> {
        self.drama_status = Some(drama_status);
        self
    }
    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    pub fn with_all_status<'b>(
        &'b mut self,
        all_status: &'a [AllStatus],
    ) -> &'b mut CountryQuery<'a> {
        self.all_status = Some(all_status);
        self
    }

    /// Filtering materials by anime studio. You can specify either one value or several values separated by commas (then materials with at least one of the listed studios will be displayed)
    pub fn with_anime_studios<'b>(
        &'b mut self,
        anime_studios: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.anime_studios = Some(anime_studios);
        self
    }
    /// Filtering materials by license owner. You can specify a single value or several values separated by commas (then materials that have at least one of the listed owners will be displayed)
    pub fn with_anime_licensed_by<'b>(
        &'b mut self,
        anime_licensed_by: &'a [&'a str],
    ) -> &'b mut CountryQuery<'a> {
        self.anime_licensed_by = Some(anime_licensed_by);
        self
    }

    /// Execute the query and fetch the results.
    pub async fn execute<'b>(&'a self, client: &'b Client) -> Result<CountryResponse, Error> {
        let payload = serialize_into_query_parts(self)?;

        let response = client
            .init_post_request("/countries")
            .query(&payload)
            .send()
            .await
            .map_err(Error::HttpError)?;

        let result = response
            .json::<CountryResponseUnion>()
            .await
            .map_err(Error::HttpError)?;

        match result {
            CountryResponseUnion::Result(result) => Ok(result),
            CountryResponseUnion::Error { error } => Err(Error::KodikError(error)),
        }
    }
}

impl<'a> Default for CountryQuery<'a> {
    fn default() -> Self {
        Self::new()
    }
}
