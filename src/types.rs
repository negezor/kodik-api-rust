use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Represents a release type on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReleaseType {
    #[serde(rename = "foreign-movie")]
    ForeignMovie,
    #[serde(rename = "soviet-cartoon")]
    SovietCartoon,
    #[serde(rename = "foreign-cartoon")]
    ForeignCartoon,
    #[serde(rename = "russian-cartoon")]
    RussianCartoon,
    #[serde(rename = "anime")]
    Anime,
    #[serde(rename = "russian-movie")]
    RussianMovie,
    #[serde(rename = "cartoon-serial")]
    CartoonSerial,
    #[serde(rename = "documentary-serial")]
    DocumentarySerial,
    #[serde(rename = "russian-serial")]
    RussianSerial,
    #[serde(rename = "foreign-serial")]
    ForeignSerial,
    #[serde(rename = "anime-serial")]
    AnimeSerial,
    #[serde(rename = "multi-part-film")]
    MultiPartFilm,
}

/// Represents a release quality on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReleaseQuality {
    #[serde(rename = "BDRip")]
    BdRip,
    #[serde(rename = "BDRip 1080p")]
    BdRip1080p,
    #[serde(rename = "BDRip 720p")]
    BdRip720p,
    #[serde(rename = "CAMRip")]
    CamRip,
    #[serde(rename = "D-VHS")]
    DVhs,
    #[serde(rename = "DVBRip")]
    DvbRip,
    #[serde(rename = "DVBRip 720p")]
    DvbRip720p,
    #[serde(rename = "DVDRip")]
    DvdRip,
    #[serde(rename = "DVDSrc")]
    DvdSrc,
    #[serde(rename = "HDDVDRip")]
    HddvdRip,
    #[serde(rename = "HDDVDRip 1080p")]
    HddvdRip1080p,
    #[serde(rename = "HDDVDRip 720p")]
    HddvdRip720p,
    #[serde(rename = "HDRip")]
    HdRip,
    #[serde(rename = "HDRip 1080p")]
    HdRip1080p,
    #[serde(rename = "HDRip 720p")]
    HdRip720p,
    #[serde(rename = "HDTVRip")]
    HdtvRip,
    #[serde(rename = "HDTVRip 1080p")]
    HdtvRip1080p,
    #[serde(rename = "HDTVRip 720p")]
    HdtvRip720p,
    #[serde(rename = "IPTVRip")]
    IptvRip,
    #[serde(rename = "Laserdisc-RIP")]
    LaserdiscRip,
    #[serde(rename = "SATRip")]
    SatRip,
    #[serde(rename = "SuperTS")]
    SuperTs,
    #[serde(rename = "TS")]
    Ts,
    #[serde(rename = "TS 720p")]
    Ts720p,
    #[serde(rename = "TVRip")]
    TvRip,
    #[serde(rename = "TVRip 720p")]
    TvRip720p,
    #[serde(rename = "VHSRip")]
    VhsRip,
    #[serde(rename = "WEB-DLRip")]
    WebDlRip,
    #[serde(rename = "WEB-DLRip 1080p")]
    WebDlRip1080p,
    #[serde(rename = "WEB-DLRip 720p")]
    WebDlRip720p,
    #[serde(rename = "Workprint-AVC")]
    WorkprintAvc,
    #[serde(other)]
    Unknown,
}

/// Represents a release on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    /// `"movie-452654"`
    pub id: String,

    /// `"Аватар"`
    pub title: String,

    /// `"Avatar"`
    pub title_orig: String,

    /// Other titles that are often used in anime
    pub other_title: Option<String>,

    /// `"http://kodik.cc/video/19850/6476310cc6d90aa9304d5d8af3a91279/720p"`
    pub link: String,

    /// Year of release of the title
    pub year: i32,

    /// `43949`
    pub kinopoisk_id: Option<String>,

    /// `tt0084716`
    pub imdb_id: Option<String>,

    /// `1245`
    pub mdl_id: Option<String>,

    /// Link to the material on World Art (not using ID because there are different
    pub worldart_link: Option<String>,

    /// `1234`
    pub shikimori_id: Option<String>,

    #[serde(rename = "type")]
    pub release_type: ReleaseType,

    pub quality: ReleaseQuality,

    /// Is the material a camrip
    pub camrip: bool,

    /// Does the material contain LGBT scenes
    pub lgbt: bool,

    /// The team that did the translation
    pub translation: Translation,

    /// ISO 8601
    pub created_at: String,

    /// ISO 8601
    pub updated_at: String,

    /// If the series is blocked entirely, this field contains the string `"all"`. If individual seasons are blocked, the field is an object containing season numbers, and for each season: either `"all"` (if all episodes are blocked) or an array of episode numbers `["1", "2", "3"]` (if individual episodes are blocked). If nothing is blocked, the field is an empty object. This field is present only in materials with the series type.
    pub blocked_seasons: Option<HashMap<String, BlockedSeason>>,

    /// Object with seasons and episodes in them. This field is present only if the parameters `with_seasons` or `with_episodes`, `with_episodes_data` were specified in the request.
    pub seasons: Option<HashMap<String, Season>>,

    /// Number of the last season of the series. This field is present only in materials with the series type.
    pub last_season: Option<i32>,

    /// Number of the last episode of the series. This field is present only in materials with the series type.
    pub last_episode: Option<i32>,

    /// Total number of episodes in the series. This field is present only in materials with the series type.
    pub episodes_count: Option<i32>,

    /// Array containing countries where the material is blocked. Empty array if the material is not blocked anywhere.
    pub blocked_countries: Vec<String>,

    pub material_data: Option<MaterialData>,

    /// Links to frames from the video. For series, frames from the first episode are displayed in the main information. To get frames from each episode, use the `with_episodes_data`.
    pub screenshots: Vec<String>,
}

/// Represents a release blocked season on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockedSeason {
    All,

    Episodes(Vec<String>),
}

/// Represents a release season object on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Season {
    /// For example, it can be marked as a recap, special, etc.
    pub title: Option<String>,

    pub link: String,

    pub episodes: HashMap<String, EpisodeUnion>,
}

/// Represents a release episode on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EpisodeUnion {
    /// `"http://kodik.cc/seria/119611/09249413a7eb3c03b15df57cd56a051b/720p"`
    Link(String),

    Episode(Episode),
}

/// Represents a release episode object on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Episode {
    /// For example, it сan be marked as special
    pub title: Option<String>,

    /// `"http://kodik.cc/seria/119611/09249413a7eb3c03b15df57cd56a051b/720p"`
    pub link: String,

    pub screenshots: Vec<String>,
}

/// Represents a release translation type on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TranslationType {
    #[serde(rename = "subtitles")]
    Subtitles,

    #[serde(rename = "voice")]
    Voice,
}

/// Represents a release translation on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Translation {
    pub id: i32,

    /// Name of the translation team
    pub title: String,

    /// Specifies what the translation team does
    #[serde(rename = "type")]
    pub translation_type: TranslationType,
}

/// Represents a release anime kind on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnimeKind {
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "ova")]
    Ova,
    #[serde(rename = "ona")]
    Ona,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "tv_13")]
    Tv13,
    #[serde(rename = "tv_24")]
    Tv24,
    #[serde(rename = "tv_48")]
    Tv48,
}

/// Represents a release all kind on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AllStatus {
    #[serde(rename = "anons")]
    Anons,
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "released")]
    Released,
}

/// Represents a release anime status on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnimeStatus {
    #[serde(rename = "anons")]
    Anons,
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "released")]
    Released,
}

/// Represents a release drama status on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DramaStatus {
    #[serde(rename = "anons")]
    Anons,
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "released")]
    Released,
}

/// Represents a release MPPA rating on Kodik
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MppaRating {
    /// `0+ `
    #[serde(rename = "G")]
    G,
    /// `6+`
    #[serde(rename = "PG")]
    Pg,
    /// `12+`
    #[serde(rename = "PG-13")]
    Pg13,
    /// `16+`
    #[serde(rename = "R")]
    R,
    /// `18+ `
    #[serde(rename = "R+")]
    RPlus,
    /// `21+`
    #[serde(rename = "Rx")]
    Rx,
}

/// Represents various data related to a material, such as title, description, ratings, etc.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialData {
    /// `"Аватар"`
    ///
    /// Source: `KinoPoisk`, `Shikimori`
    pub title: Option<String>,

    /// `"Аватар"`
    ///
    /// Source: `Shikimori`
    pub anime_title: Option<String>,

    /// Original title
    ///
    /// `"Avatar"`
    ///
    /// Source: `KinoPoisk`, `Shikimori`, `MyDramaList`
    pub title_en: Option<String>,

    /// `["Аватар", "Аватар 2", "Аватар 3"]`
    ///
    /// Source: `Shikimori`, `MyDramaList`
    pub other_titles: Option<Vec<String>>,

    /// `["Avatar", "Avatar 2", "Avatar 3"]`
    ///
    /// Source: `Shikimori`
    pub other_titles_en: Option<Vec<String>>,

    /// `["アバター", "アバター 2", "アバター 3"]`
    ///
    /// Source: `Shikimori`
    pub other_titles_jp: Option<Vec<String>>,

    /// `"Avatar"`
    ///
    /// Source: `Shikimori`
    pub anime_license_name: Option<String>,

    /// License holders
    ///
    /// `["Wakanim", "Русский Репортаж"]`
    ///
    /// Source: `Shikimori`
    pub anime_licensed_by: Option<Vec<String>>,

    /// Source: `Shikimori`
    pub anime_kind: Option<AnimeKind>,

    /// Material status from all sources
    ///
    /// Source: `Shikimori`, `MyDramaList`
    pub all_status: Option<AllStatus>,

    /// Source: `Shikimori`
    pub anime_status: Option<AnimeStatus>,

    /// Source: `MyDramaList`
    pub drama_status: Option<DramaStatus>,

    /// Year of release of the title
    ///
    /// `2016`
    ///
    /// Source: `KinoPoisk`
    pub year: Option<i32>,

    /// `"«An entire universe. Once and for all»"`
    ///
    /// Source: `KinoPoisk`
    pub tagline: Option<String>,

    /// `"Пока Мстители и их союзники продо..."`
    ///
    /// Source: `KinoPoisk`, `Shikimori`
    pub description: Option<String>,

    /// `"Пока Мстители и их союзники продо..."`
    ///
    /// Source: `Shikimori`
    pub anime_description: Option<String>,

    /// `"https://st.kp.yandex.net/images/film_iphone/iphone360_840471.jpg"`
    ///
    /// Source: `KinoPoisk`, `Shikimori`, `MyDramaList`
    pub poster_url: Option<String>,

    /// `["https://site.com/image1.png", "https://site.com/image2.png"]`
    ///
    /// Source: `Shikimori`
    pub screenshots: Option<Vec<String>>,

    /// Duration in minutes
    ///
    /// `160`
    ///
    /// Source: `KinoPoisk`, `Shikimori`, `MyDramaList`
    pub duration: Option<i32>,

    /// `["США", "Великобритания"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub countries: Option<Vec<String>>,

    /// Genres from all available sources
    ///
    /// `["комедия", "боевик"]`
    ///
    /// Source: `KinoPoisk`, `Shikimori`, `MyDramaList`
    pub all_genres: Option<Vec<String>>,

    /// `["комедия", "боевик"]`
    ///
    /// Source: `KinoPoisk`
    pub genres: Option<Vec<String>>,

    /// `["приключения","комедия"]`
    ///
    /// Source: `Shikimori`
    pub anime_genres: Option<Vec<String>>,

    /// `["приключения","комедия"]`
    ///
    /// Source: `MyDramaList`
    pub drama_genres: Option<Vec<String>>,

    /// `["Studio Deen"]`
    ///
    /// Source: `Shikimori`
    pub anime_studios: Option<Vec<String>>,

    /// `7.2`
    ///
    /// Source: `KinoPoisk`
    pub kinopoisk_rating: Option<f64>,

    /// Number of votes on Kinopoisk
    ///
    /// `723856`
    ///
    /// Source: `KinoPoisk`
    pub kinopoisk_votes: Option<i32>,

    /// `7.2`
    ///
    /// Source: `KinoPoisk`
    pub imdb_rating: Option<f64>,

    /// Number of votes on IMDb
    ///
    /// `723856`
    ///
    /// Source: `KinoPoisk`
    pub imdb_votes: Option<i32>,

    /// `7.2`
    ///
    /// Source: `Shikimori`
    pub shikimori_rating: Option<f32>,

    /// Number of votes on Shikimori
    ///
    /// Source: `Shikimori`
    pub shikimori_votes: Option<i32>,

    /// `7.2`
    ///
    /// Source: `MyDramaList`
    pub mydramalist_rating: Option<f32>,

    /// Number of votes on MyDramaList
    ///
    /// Source: `MyDramaList`
    pub mydramalist_votes: Option<i32>,

    /// Premiere date in Russia
    ///
    /// `"2018-04-16"`
    ///
    /// Source: `KinoPoisk`
    pub premiere_ru: Option<String>,

    /// Worldwide premiere date
    ///
    /// `"2018-04-16"`
    ///
    /// Source: `KinoPoisk`
    pub premiere_world: Option<String>,

    /// Airing start date
    ///
    /// `"2018-04-16"`
    ///
    /// Source: `Shikimori`, `MyDramaList`
    pub aired_at: Option<String>,

    /// Airing end date
    ///
    /// `"2018-04-16"`
    ///
    /// Source: `Shikimori`, `MyDramaList`
    pub released_at: Option<String>,

    /// Next episode release time
    ///
    /// `"2021-04-06T14:19:27Z"`
    ///
    /// Source: `Shikimori`, `MyDramaList`
    pub next_episode_at: Option<String>,

    /// MPAA rating
    /// Source: `KinoPoisk`, `Shikimori`
    pub rating_mpaa: Option<MppaRating>,

    /// Minimum age to watch
    ///
    /// `16`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub minimal_age: Option<i32>,

    /// Total number of episodes
    ///
    /// `14`
    ///
    /// Source: `Shikimori`, `MyDramaList`
    pub episodes_total: Option<i32>,

    /// Number of aired episodes
    ///
    /// `14`
    ///
    /// Source: `Shikimori`, `MyDramaList`
    pub episodes_aired: Option<i32>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub actors: Option<Vec<String>>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub directors: Option<Vec<String>>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub producers: Option<Vec<String>>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub writers: Option<Vec<String>>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub composers: Option<Vec<String>>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub editors: Option<Vec<String>>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub designers: Option<Vec<String>>,

    /// `["Роберт Дауни мл.", "Крис Хемсворт", "Марк Руффало"]`
    ///
    /// Source: `KinoPoisk`, `MyDramaList`
    pub operators: Option<Vec<String>>,
}
