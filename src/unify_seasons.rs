use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::{EpisodeUnion, Release};

/// Represents a release unified episode object on Kodik
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnifiedEpisode {
    /// For example, it сan be marked as special
    pub title: Option<String>,

    /// `"http://kodik.cc/seria/119611/09249413a7eb3c03b15df57cd56a051b/720p"`
    pub link: String,

    pub screenshots: Vec<String>,
}

/// Represents a release unified season object on Kodik
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnifiedSeason {
    /// For example, it can be marked as a recap, special, etc.
    pub title: Option<String>,

    pub link: String,

    pub episodes: BTreeMap<String, UnifiedEpisode>,
}

/// Returns seasons and episodes in a unified format for the Kodik release.
///
/// Kodik returns different response formats for movies, shows, depending on the parameters and the state of the sun.
pub fn unify_seasons(release: &Release) -> BTreeMap<String, UnifiedSeason> {
    let Some(kodik_seasons) = &release.seasons else {
        return BTreeMap::from([(
            "1".to_owned(),
            UnifiedSeason {
                title: None,
                link: release.link.clone(),
                episodes: BTreeMap::from([(
                    "1".to_owned(),
                    UnifiedEpisode {
                        title: None,
                        link: release.link.clone(),
                        screenshots: release.screenshots.clone(),
                    },
                )]),
            },
        )]);
    };

    let mut seasons = BTreeMap::new();

    for (season_num, kodik_season) in kodik_seasons {
        let mut episodes = BTreeMap::new();

        for (episode_num, kodik_episode_union) in &kodik_season.episodes {
            let episode = match kodik_episode_union {
                EpisodeUnion::Episode(kodik_episode) => UnifiedEpisode {
                    title: kodik_episode.title.clone(),
                    link: kodik_episode.link.clone(),
                    screenshots: kodik_episode.screenshots.clone(),
                },
                EpisodeUnion::Link(link) => UnifiedEpisode {
                    title: None,
                    link: link.clone(),
                    screenshots: release.screenshots.clone(),
                },
            };

            episodes.insert(episode_num.clone(), episode);
        }

        seasons.insert(
            season_num.clone(),
            UnifiedSeason {
                title: kodik_season.title.clone(),
                link: kodik_season.link.clone(),
                episodes,
            },
        );
    }

    seasons
}

#[cfg(test)]
mod tests {
    use crate::types::{
        Episode, ReleaseQuality, ReleaseType, Season, Translation, TranslationType,
    };

    use super::*;

    fn get_default_kodik_release() -> Release {
        Release {
            id: "serial-45534".to_owned(),
            title: "Киберпанк: Бегущие по краю".to_owned(),
            title_orig: "Cyberpunk: Edgerunners".to_owned(),
            other_title: Some("サイバーパンク エッジランナーズ".to_owned()),
            link: "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p".to_owned(),
            year: 2022,
            kinopoisk_id: Some("2000102".to_owned()),
            imdb_id: Some("tt12590266".to_owned()),
            mdl_id: None,
            worldart_link: Some(
                "http://www.world-art.ru/animation/animation.php?id=10534".to_owned(),
            ),
            shikimori_id: Some("42310".to_owned()),
            release_type: ReleaseType::AnimeSerial,
            quality: ReleaseQuality::WebDlRip720p,
            camrip: false,
            lgbt: false,
            translation: Translation {
                id: 610,
                title: "AniLibria.TV".to_owned(),
                translation_type: TranslationType::Voice,
            },
            created_at: "2022-09-14T10:54:34Z".to_owned(),
            updated_at: "2022-09-23T22:31:33Z".to_owned(),
            blocked_seasons: Some(BTreeMap::new()),
            seasons: None,
            last_season: Some(1),
            last_episode: Some(10),
            episodes_count: Some(10),
            blocked_countries: vec![],
            material_data: None,
            screenshots: vec!["https://i.kodik.biz/screenshots/seria/104981222/1.jpg".to_owned()],
        }
    }

    #[test]
    fn test_unify_kodik_without_seasons() {
        let kodik_release = get_default_kodik_release();

        let unified_season = unify_seasons(&kodik_release);

        assert_eq!(
            unified_season,
            BTreeMap::from([(
                "1".to_owned(),
                UnifiedSeason {
                    title: None,
                    link: kodik_release.link.clone(),
                    episodes: BTreeMap::from([(
                        "1".to_owned(),
                        UnifiedEpisode {
                            title: None,
                            link: kodik_release.link.clone(),
                            screenshots: kodik_release.screenshots,
                        }
                    )]),
                }
            )])
        )
    }

    #[test]
    fn test_unify_kodik_with_seasons() {
        let mut kodik_release = get_default_kodik_release();

        let seasons = BTreeMap::from([(
            "1".to_owned(),
            Season {
                link: kodik_release.link.clone(),
                title: None,
                episodes: BTreeMap::from([
                    (
                        "1".to_owned(),
                        EpisodeUnion::Link(
                            "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p/1"
                                .to_owned(),
                        ),
                    ),
                    (
                        "2".to_owned(),
                        EpisodeUnion::Episode(Episode {
                            title: None,
                            link:
                                "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p/2"
                                    .to_owned(),
                            screenshots: kodik_release.screenshots.clone(),
                        }),
                    ),
                    (
                        "3".to_owned(),
                        EpisodeUnion::Link(
                            "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p/3"
                                .to_owned(),
                        ),
                    ),
                ]),
            },
        )]);

        kodik_release.seasons = Some(seasons);

        let unified_season = unify_seasons(&kodik_release);

        assert_eq!(unified_season, BTreeMap::from([
            ("1".to_owned(), UnifiedSeason {
                title: None,
                link: kodik_release.link.clone(),
                episodes: BTreeMap::from([
                    ("1".to_owned(), UnifiedEpisode {
                        title: None,
                        link: "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p/1".to_owned(),
                        screenshots: kodik_release.screenshots.clone(),
                    }),
                    ("2".to_owned(), UnifiedEpisode {
                        title: None,
                        link: "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p/2".to_owned(),
                        screenshots: kodik_release.screenshots.clone(),
                    }),
                    ("3".to_owned(), UnifiedEpisode {
                        title: None,
                        link: "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p/3".to_owned(),
                        screenshots: kodik_release.screenshots,
                    }),
                ]),
            })
        ]))
    }
}
