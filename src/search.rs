use serde::{Deserialize, Serialize};

use crate::{
    constants::BASE_URL,
    error::Error,
    types::{
        AllStatus, AnimeKind, AnimeStatus, DramaStatus, MppaRating, Release, ReleaseType,
        TranslationType,
    },
    Client,
};

/// A struct containing search results and other information about the search
#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub time: String,
    pub total: i32,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
    pub results: Vec<Release>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum SearchResponseUnion {
    Result(SearchResponse),
    Error { error: String },
}

#[derive(Debug, Serialize, Clone)]
pub struct SearchQuery<'a> {
    /// The name of the movie. It is not necessary to specify it explicitly, you can use a variant written by the user or a variant containing extra words. If you specify one of these parameters, the search will be performed on several fields at once: `title`, `title_orig`, `other_title`
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    /// Original title. When this option is used, only the title_orig will be searched. It is not necessary to specify it explicitly, you can use a variant written by the user or a variant containing extra words
    #[serde(skip_serializing_if = "Option::is_none")]
    title_orig: Option<&'a str>,
    /// If title or title_orig parameter was specified, this parameter defines the severity of checking if the title of the material corresponds to the search query. If true, the search results will show only those materials in which the word order is exactly the same as in the search query (but extra words in the search query are still allowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
    /// If title or title_orig parameter was specified, this parameter defines the severity of checking if the title of the material corresponds to the search query. If true, the search results will only show content where the title completely matches the search query (no extra words, word order and presence of characters are also completely identical). The only thing the title may differ from the query is the case. When used in conjunction with the title, full consistency with at least one of the titles of the material is checked
    #[serde(skip_serializing_if = "Option::is_none")]
    full_match: Option<bool>,

    /// Search by Kodik ID
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    /// Search for any link to the player
    #[serde(skip_serializing_if = "Option::is_none")]
    player_link: Option<&'a str>,

    /// Search by kinopoisk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    kinopoisk_id: Option<&'a str>,
    /// Search by IMDb ID
    #[serde(skip_serializing_if = "Option::is_none")]
    imdb_id: Option<&'a str>,
    /// Search by MyDramaList ID
    #[serde(skip_serializing_if = "Option::is_none")]
    mdl_id: Option<&'a str>,

    /// Search for World Art IDs in the anime section (World Art has different content sections, each with their own independent IDs)
    #[serde(skip_serializing_if = "Option::is_none")]
    worldart_animation_id: Option<&'a str>,
    /// Search for World Art IDs in the Movies section
    #[serde(skip_serializing_if = "Option::is_none")]
    worldart_cinema_id: Option<&'a str>,
    /// Search the full World Art link
    #[serde(skip_serializing_if = "Option::is_none")]
    worldart_link: Option<&'a str>,
    /// Search by Shikimori ID
    #[serde(skip_serializing_if = "Option::is_none")]
    shikimori_id: Option<&'a str>,

    /// Maximum number of outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,

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
    /// Increases the priority of certain voices. The IDs are listed in commas. The "leftmost" ID, the higher its priority. IDs of all voices can be received through API resource /translations or on the page of list of voices. Standard priority of dubbed and prof. Multivoiced". To deactivate standard priority you need to pass value 0. You can also specify the translation type (subtitles/voice) instead of the ID
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: Add wrapper
    prioritize_translations: Option<&'a [&'a str]>,
    /// Decreases the priority of certain voices. The IDs are listed in commas. The "leftmost" ID, the lower its priority. IDs of all voices can be received through API resource /translations or on page of voices list. Standard priority of soundtracks "Ukrainian", "English" and all subtitles are lowered. To deactivate standard priority you need to pass value 0. You can also specify the translation type (subtitles/voice) instead of the ID
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: Add wrapper
    unprioritize_translations: Option<&'a [&'a str]>,
    /// Increases the priority of a certain type of translation. If you specify voice, voiceovers will be output first. If subtitles, subtitles will be output
    #[serde(skip_serializing_if = "Option::is_none")]
    prioritize_translation_type: Option<&'a [TranslationType]>,

    /// Deletes certain voices from the search results. IDs are listed separated by commas
    #[serde(skip_serializing_if = "Option::is_none")]
    block_translations: Option<&'a [u32]>,

    /// Filtering materials by camrip parameter. If you specify false, only materials with a quality picture will be output. If you don't specify this parameter, all materials will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    camrip: Option<bool>,
    /// Filters materials by the lgbt parameter. If you specify false, only materials that do not contain LGBT scenes will be output. If you don't specify this parameter, all materials will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    lgbt: Option<bool>,

    /// If you specify true, the seasons of the series will also be listed in the seasons field. This and the following parameter are made to avoid overloading the output with a huge amount of information about seasons and episodes, if this information is not needed for parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    with_seasons: Option<bool>,

    /// With this option you can specify which season you are interested in. This way, only shows that have that season will appear in the search results. Passing this parameter also automatically enables the with_seasons parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    season: Option<&'a [u32]>,

    /// If you specify true, the seasons field will be added to each series (even if with_seasons is not specified or specified as false) and the episodes field with the episodes of that season will be added to each season. If the with_episodes parameter is used, the series numbers will correspond to the normal series references. If you use the with_episodes_data parameter, episode objects will be assigned to the episode numbers, where the link will be available via the link parameter, the episode name (if any) via the title parameter, and the frames via screenshots
    #[serde(skip_serializing_if = "Option::is_none")]
    with_episodes: Option<bool>,
    /// If you specify true, the seasons field will be added to each series (even if with_seasons is not specified or specified as false) and the episodes field with the episodes of that season will be added to each season. If the with_episodes parameter is used, the series numbers will correspond to the normal series references. If you use the with_episodes_data parameter, episode objects will be assigned to the episode numbers, where the link will be available via the link parameter, the episode name (if any) via the title parameter, and the frames via screenshots
    #[serde(skip_serializing_if = "Option::is_none")]
    with_episodes_data: Option<bool>,

    /// With this option, you can specify which episode of a particular season you are interested in. Thus, only shows with that episode will appear in the search results. If you use this parameter, you must also pass the season parameter. Passing this parameter also automatically includes the with_episodes parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    episode: Option<&'a [u32]>,

    /// If you specify true, all links to players will be replaced by special links to pages with players (suitable for cases when you don't have your own site). You can customize appearance of these pages in settings in the base. If parameter with_seasons or with_episodes / with_episodes_data is specified together with this parameter, links in seasons and episodes will also be replaced
    #[serde(skip_serializing_if = "Option::is_none")]
    with_page_links: Option<bool>,

    /// Filters materials by country in which they should not be blocked. The country codes are specified separated by commas
    #[serde(skip_serializing_if = "Option::is_none")]
    not_blocked_in: Option<&'a [&'a str]>,
    /// A simpler analog of the previous parameter. Our server itself checks which country the current request comes from and doesn't display those materials that are blocked for that country. This parameter can be useful if the API is called on your site
    #[serde(skip_serializing_if = "Option::is_none")]
    not_blocked_for_me: Option<&'a [&'a str]>,

    /// If you specify true, the material_data field will be added to each movie/series with information from Kinopoisk and Shikimori
    #[serde(skip_serializing_if = "Option::is_none")]
    with_material_data: Option<bool>,

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
    mpaa_rating: Option<&'a [MppaRating]>,

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

impl<'a> SearchQuery<'a> {
    pub fn new() -> SearchQuery<'a> {
        SearchQuery {
            title: None,
            title_orig: None,
            strict: None,
            full_match: None,
            id: None,
            player_link: None,
            kinopoisk_id: None,
            imdb_id: None,
            mdl_id: None,
            worldart_animation_id: None,
            worldart_cinema_id: None,
            worldart_link: None,
            shikimori_id: None,
            limit: None,
            types: None,
            year: None,
            translation_id: None,
            translation_type: None,
            prioritize_translations: None,
            unprioritize_translations: None,
            prioritize_translation_type: None,
            block_translations: None,
            camrip: None,
            lgbt: None,
            with_seasons: None,
            season: None,
            with_episodes: None,
            with_episodes_data: None,
            episode: None,
            with_page_links: None,
            not_blocked_in: None,
            not_blocked_for_me: None,
            with_material_data: None,
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
            mpaa_rating: None,
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

    /// The name of the movie. It is not necessary to specify it explicitly, you can use a variant written by the user or a variant containing extra words. If you specify one of these parameters, the search will be performed on several fields at once: `title`, `title_orig`, `other_title`
    pub fn with_title<'b>(&'b mut self, title: &'a str) -> &'b mut SearchQuery<'a> {
        self.title = Some(title);
        self
    }
    /// Original title. When this option is used, only the title_orig will be searched. It is not necessary to specify it explicitly, you can use a variant written by the user or a variant containing extra words
    pub fn with_title_orig<'b>(&'b mut self, title_orig: &'a str) -> &'b mut SearchQuery<'a> {
        self.title_orig = Some(title_orig);
        self
    }
    /// If title or title_orig parameter was specified, this parameter defines the severity of checking if the title of the material corresponds to the search query. If true, the search results will show only those materials in which the word order is exactly the same as in the search query (but extra words in the search query are still allowed)
    pub fn with_strict<'b>(&'b mut self, strict: bool) -> &'b mut SearchQuery<'a> {
        self.strict = Some(strict);
        self
    }
    /// If title or title_orig parameter was specified, this parameter defines the severity of checking if the title of the material corresponds to the search query. If true, the search results will only show content where the title completely matches the search query (no extra words, word order and presence of characters are also completely identical). The only thing the title may differ from the query is the case. When used in conjunction with the title, full consistency with at least one of the titles of the material is checked
    pub fn with_full_match<'b>(&'b mut self, full_match: bool) -> &'b mut SearchQuery<'a> {
        self.full_match = Some(full_match);
        self
    }

    /// Search by Kodik ID
    pub fn with_id<'b>(&'b mut self, id: &'a str) -> &'b mut SearchQuery<'a> {
        self.id = Some(id);
        self
    }
    /// Search for any link to the player
    pub fn with_player_link<'b>(&'b mut self, player_link: &'a str) -> &'b mut SearchQuery<'a> {
        self.player_link = Some(player_link);
        self
    }

    /// Search by kinopoisk ID
    pub fn with_kinopoisk_id<'b>(&'b mut self, kinopoisk_id: &'a str) -> &'b mut SearchQuery<'a> {
        self.kinopoisk_id = Some(kinopoisk_id);
        self
    }
    /// Search by IMDb ID
    pub fn with_imdb_id<'b>(&'b mut self, imdb_id: &'a str) -> &'b mut SearchQuery<'a> {
        self.imdb_id = Some(imdb_id);
        self
    }
    /// Search by MyDramaList ID
    pub fn with_mdl_id<'b>(&'b mut self, mdl_id: &'a str) -> &'b mut SearchQuery<'a> {
        self.mdl_id = Some(mdl_id);
        self
    }

    /// Search for World Art IDs in the anime section (World Art has different content sections, each with their own independent IDs)
    pub fn with_worldart_animation_id<'b>(
        &'b mut self,
        worldart_animation_id: &'a str,
    ) -> &'b mut SearchQuery<'a> {
        self.worldart_animation_id = Some(worldart_animation_id);
        self
    }
    /// Search for World Art IDs in the Movies section
    pub fn with_worldart_cinema_id<'b>(
        &'b mut self,
        worldart_cinema_id: &'a str,
    ) -> &'b mut SearchQuery<'a> {
        self.worldart_cinema_id = Some(worldart_cinema_id);
        self
    }
    /// Search the full World Art link
    pub fn with_worldart_link<'b>(&'b mut self, worldart_link: &'a str) -> &'b mut SearchQuery<'a> {
        self.worldart_link = Some(worldart_link);
        self
    }
    /// Search by Shikimori ID
    pub fn with_shikimori_id<'b>(&'b mut self, shikimori_id: &'a str) -> &'b mut SearchQuery<'a> {
        self.shikimori_id = Some(shikimori_id);
        self
    }

    /// Maximum number of outputs
    pub fn with_limit<'b>(&'b mut self, limit: u32) -> &'b mut SearchQuery<'a> {
        self.limit = Some(limit);
        self
    }

    /// Maximum number of outputs
    pub fn with_types<'b>(&'b mut self, types: &'a [ReleaseType]) -> &'b mut SearchQuery<'a> {
        self.types = Some(types);
        self
    }

    ///Filter materials by year If you set this parameter, only materials of the corresponding year will be displayed

    pub fn with_year<'b>(&'b mut self, year: &'a [u32]) -> &'b mut SearchQuery<'a> {
        self.year = Some(year);
        self
    }

    /// Filtering materials by translation ID
    pub fn with_translation_id<'b>(
        &'b mut self,
        translation_id: &'a [u32],
    ) -> &'b mut SearchQuery<'a> {
        self.translation_id = Some(translation_id);
        self
    }
    /// Filter content by translation type. Allows you to output only voice translation or only subtitles
    pub fn with_translation_type<'b>(
        &'b mut self,
        translation_type: &'a [TranslationType],
    ) -> &'b mut SearchQuery<'a> {
        self.translation_type = Some(translation_type);
        self
    }
    /// Increases the priority of certain voices. The IDs are listed in commas. The "leftmost" ID, the higher its priority. IDs of all voices can be received through API resource /translations or on the page of list of voices. Standard priority of dubbed and prof. Multivoiced". To deactivate standard priority you need to pass value 0. You can also specify the translation type (subtitles/voice) instead of the ID
    // TODO: Add wrapper
    pub fn with_prioritize_translations<'b>(
        &'b mut self,
        prioritize_translations: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.prioritize_translations = Some(prioritize_translations);
        self
    }
    /// Decreases the priority of certain voices. The IDs are listed in commas. The "leftmost" ID, the lower its priority. IDs of all voices can be received through API resource /translations or on page of voices list. Standard priority of soundtracks "Ukrainian", "English" and all subtitles are lowered. To deactivate standard priority you need to pass value 0. You can also specify the translation type (subtitles/voice) instead of the ID
    // TODO: Add wrapper
    pub fn with_unprioritize_translations<'b>(
        &'b mut self,
        unprioritize_translations: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.unprioritize_translations = Some(unprioritize_translations);
        self
    }
    /// Increases the priority of a certain type of translation. If you specify voice, voiceovers will be output first. If subtitles, subtitles will be output
    pub fn with_prioritize_translation_type<'b>(
        &'b mut self,
        prioritize_translation_type: &'a [TranslationType],
    ) -> &'b mut SearchQuery<'a> {
        self.prioritize_translation_type = Some(prioritize_translation_type);
        self
    }

    /// Deletes certain voices from the search results. IDs are listed separated by commas
    pub fn with_block_translations<'b>(
        &'b mut self,
        block_translations: &'a [u32],
    ) -> &'b mut SearchQuery<'a> {
        self.block_translations = Some(block_translations);
        self
    }

    /// Filtering materials by camrip parameter. If you specify false, only materials with a quality picture will be output. If you don't specify this parameter, all materials will be displayed
    pub fn with_camrip<'b>(&'b mut self, camrip: bool) -> &'b mut SearchQuery<'a> {
        self.camrip = Some(camrip);
        self
    }
    /// Filters materials by the lgbt parameter. If you specify false, only materials that do not contain LGBT scenes will be output. If you don't specify this parameter, all materials will be displayed
    pub fn with_lgbt<'b>(&'b mut self, lgbt: bool) -> &'b mut SearchQuery<'a> {
        self.lgbt = Some(lgbt);
        self
    }

    /// If you specify true, the seasons of the series will also be listed in the seasons field. This and the following parameter are made to avoid overloading the output with a huge amount of information about seasons and episodes, if this information is not needed for parsing
    pub fn with_with_seasons<'b>(&'b mut self, with_seasons: bool) -> &'b mut SearchQuery<'a> {
        self.with_seasons = Some(with_seasons);
        self
    }

    /// With this option you can specify which season you are interested in. This way, only shows that have that season will appear in the search results. Passing this parameter also automatically enables the with_seasons parameter
    pub fn with_season<'b>(&'b mut self, season: &'a [u32]) -> &'b mut SearchQuery<'a> {
        self.season = Some(season);
        self
    }

    /// If you specify true, the seasons field will be added to each series (even if with_seasons is not specified or specified as false) and the episodes field with the episodes of that season will be added to each season. If the with_episodes parameter is used, the series numbers will correspond to the normal series references. If you use the with_episodes_data parameter, episode objects will be assigned to the episode numbers, where the link will be available via the link parameter, the episode name (if any) via the title parameter, and the frames via screenshots
    pub fn with_with_episodes<'b>(&'b mut self, with_episodes: bool) -> &'b mut SearchQuery<'a> {
        self.with_episodes = Some(with_episodes);
        self
    }
    /// If you specify true, the seasons field will be added to each series (even if with_seasons is not specified or specified as false) and the episodes field with the episodes of that season will be added to each season. If the with_episodes parameter is used, the series numbers will correspond to the normal series references. If you use the with_episodes_data parameter, episode objects will be assigned to the episode numbers, where the link will be available via the link parameter, the episode name (if any) via the title parameter, and the frames via screenshots
    pub fn with_with_episodes_data<'b>(
        &'b mut self,
        with_episodes_data: bool,
    ) -> &'b mut SearchQuery<'a> {
        self.with_episodes_data = Some(with_episodes_data);
        self
    }

    /// With this option, you can specify which episode of a particular season you are interested in. Thus, only shows with that episode will appear in the search results. If you use this parameter, you must also pass the season parameter. Passing this parameter also automatically includes the with_episodes parameter
    pub fn with_episode<'b>(&'b mut self, episode: &'a [u32]) -> &'b mut SearchQuery<'a> {
        self.episode = Some(episode);
        self
    }

    /// If you specify true, all links to players will be replaced by special links to pages with players (suitable for cases when you don't have your own site). You can customize appearance of these pages in settings in the base. If parameter with_seasons or with_episodes / with_episodes_data is specified together with this parameter, links in seasons and episodes will also be replaced
    pub fn with_with_page_links<'b>(
        &'b mut self,
        with_page_links: bool,
    ) -> &'b mut SearchQuery<'a> {
        self.with_page_links = Some(with_page_links);
        self
    }

    /// Filters materials by country in which they should not be blocked. The country codes are specified separated by commas
    pub fn with_not_blocked_in<'b>(
        &'b mut self,
        not_blocked_in: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.not_blocked_in = Some(not_blocked_in);
        self
    }
    /// A simpler analog of the previous parameter. Our server itself checks which country the current request comes from and doesn't display those materials that are blocked for that country. This parameter can be useful if the API is called on your site
    pub fn with_not_blocked_for_me<'b>(
        &'b mut self,
        not_blocked_for_me: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.not_blocked_for_me = Some(not_blocked_for_me);
        self
    }
    /// If you specify true, the material_data field will be added to each movie/series with information from Kinopoisk and Shikimori
    pub fn with_with_material_data<'b>(
        &'b mut self,
        with_material_data: bool,
    ) -> &'b mut SearchQuery<'a> {
        self.with_material_data = Some(with_material_data);
        self
    }

    /// Filtering materials by country. You can specify a single value or multiple values, separated by commas (then materials with at least one of the listed countries will be displayed). The parameter is case sensitive
    pub fn with_countries<'b>(&'b mut self, countries: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.countries = Some(countries);
        self
    }

    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_genres<'b>(&'b mut self, genres: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.genres = Some(genres);
        self
    }
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_anime_genres<'b>(
        &'b mut self,
        anime_genres: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.anime_genres = Some(anime_genres);
        self
    }
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_drama_genres<'b>(
        &'b mut self,
        drama_genres: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.drama_genres = Some(drama_genres);
        self
    }
    /// Filtering by genre. You can specify either one value or several values separated by commas (then materials that have at least one of the specified genres will be displayed). You can search by Kinopoisk, Shikimori, MyDramaList or by all genres at once. The parameter is not case sensitive
    pub fn with_all_genres<'b>(&'b mut self, all_genres: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.all_genres = Some(all_genres);
        self
    }

    /// Filtering by duration (in minutes). You can specify either a single value to search for the exact duration, or an interval.
    pub fn with_duration<'b>(&'b mut self, duration: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.duration = Some(duration);
        self
    }

    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_kinopoisk_rating<'b>(
        &'b mut self,
        kinopoisk_rating: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.kinopoisk_rating = Some(kinopoisk_rating);
        self
    }
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_imdb_rating<'b>(
        &'b mut self,
        imdb_rating: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.imdb_rating = Some(imdb_rating);
        self
    }
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_shikimori_rating<'b>(
        &'b mut self,
        shikimori_rating: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.shikimori_rating = Some(shikimori_rating);
        self
    }
    /// Filtering by Kinopoisk, IMDb, Shikimori, or MyDramaList ratings. You can specify either a single value to search for the exact rating, or an interval
    pub fn with_mydramalist_rating<'b>(
        &'b mut self,
        mydramalist_rating: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.mydramalist_rating = Some(mydramalist_rating);
        self
    }

    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_actors<'b>(&'b mut self, actors: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.actors = Some(actors);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_directors<'b>(&'b mut self, directors: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.directors = Some(directors);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_producers<'b>(&'b mut self, producers: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.producers = Some(producers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_writers<'b>(&'b mut self, writers: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.writers = Some(writers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_composers<'b>(&'b mut self, composers: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.composers = Some(composers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_editors<'b>(&'b mut self, editors: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.editors = Some(editors);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_designers<'b>(&'b mut self, designers: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.designers = Some(designers);
        self
    }
    /// Filtering materials by personas. You can specify a single value or multiple values, separated by commas (then materials that have at least one of the specified personas will be displayed). This parameter is case-independent. You can specify filters for several professions at once
    pub fn with_operators<'b>(&'b mut self, operators: &'a [&'a str]) -> &'b mut SearchQuery<'a> {
        self.operators = Some(operators);
        self
    }

    /// Filtering materials by age rating. You can specify a single value or multiple values, separated by commas. The parameter is case-insensitive
    pub fn with_mpaa_rating<'b>(
        &'b mut self,
        mpaa_rating: &'a [MppaRating],
    ) -> &'b mut SearchQuery<'a> {
        self.mpaa_rating = Some(mpaa_rating);
        self
    }

    /// Filter content by the minimum age from which it can be viewed. You can specify either a single value or a range of values
    pub fn with_minimal_age<'b>(
        &'b mut self,
        minimal_age: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.minimal_age = Some(minimal_age);
        self
    }

    /// Filtering materials by anime type. You can specify one value or several values separated by commas (then materials with at least one of these types will be displayed)
    pub fn with_anime_kind<'b>(
        &'b mut self,
        anime_kind: &'a [AnimeKind],
    ) -> &'b mut SearchQuery<'a> {
        self.anime_kind = Some(anime_kind);
        self
    }

    /// Filters materials by MyDramaList tags. You can specify one value or several values separated by commas (then materials with at least one of these types will be displayed)
    pub fn with_mydramalist_tags<'b>(
        &'b mut self,
        mydramalist_tags: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.mydramalist_tags = Some(mydramalist_tags);
        self
    }

    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    pub fn with_anime_status<'b>(
        &'b mut self,
        anime_status: &'a [AnimeStatus],
    ) -> &'b mut SearchQuery<'a> {
        self.anime_status = Some(anime_status);
        self
    }
    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    pub fn with_drama_status<'b>(
        &'b mut self,
        drama_status: &'a [DramaStatus],
    ) -> &'b mut SearchQuery<'a> {
        self.drama_status = Some(drama_status);
        self
    }
    /// Filter materials by Shikimori status, MyDramaList, or by all statuses. You can specify a single value or several values separated by commas (then materials that have at least one of the listed statuses will be displayed)
    pub fn with_all_status<'b>(
        &'b mut self,
        all_status: &'a [AllStatus],
    ) -> &'b mut SearchQuery<'a> {
        self.all_status = Some(all_status);
        self
    }

    /// Filtering materials by anime studio. You can specify either one value or several values separated by commas (then materials with at least one of the listed studios will be displayed)
    pub fn with_anime_studios<'b>(
        &'b mut self,
        anime_studios: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.anime_studios = Some(anime_studios);
        self
    }
    /// Filtering materials by license owner. You can specify a single value or several values separated by commas (then materials that have at least one of the listed owners will be displayed)
    pub fn with_anime_licensed_by<'b>(
        &'b mut self,
        anime_licensed_by: &'a [&'a str],
    ) -> &'b mut SearchQuery<'a> {
        self.anime_licensed_by = Some(anime_licensed_by);
        self
    }

    /// Execute the query and fetch the results.
    pub async fn execute<'b>(&'a self, client: &'b Client) -> Result<SearchResponse, Error> {
        let body =
            comma_serde_urlencoded::to_string(self).map_err(Error::UrlencodedSerializeError)?;

        let response = client
            .init_post_request(&format!("{BASE_URL}/search"))
            .body(body)
            .send()
            .await
            .map_err(Error::HttpError)?;

        let result = response
            .json::<SearchResponseUnion>()
            .await
            .map_err(Error::HttpError)?;

        match result {
            SearchResponseUnion::Result(result) => Ok(result),
            SearchResponseUnion::Error { error } => Err(Error::KodikError(error)),
        }
    }
}

impl<'a> Default for SearchQuery<'a> {
    fn default() -> Self {
        Self::new()
    }
}
