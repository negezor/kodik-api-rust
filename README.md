# Kodik Rust

<a href="https://crates.io/crates/kodik-api"><img src="https://img.shields.io/crates/v/kodik-api?style=flat-square&logo=rust" alt="Crate version"></a>
<a href="https://github.com/negezor/kodik-api-rust/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/negezor/kodik-api-rust/main.yml?style=flat-square&logo=github&label=Tests" alt="Tests"></a>
<a href="https://github.com/negezor/kodik-api-rust/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-informational?style=flat-square" alt="License"></a>

> **Kodik Rust** - An efficient Rust library serving as a wrapper for the Kodik API 🦾

| 📖 [Documentation](https://docs.rs/kodik-api)  |
| ------------------------------------------ |

## Installation

Install [kodik from crates.io](https://crates.io/crates/kodik-api). Add the following line to your `Cargo.toml` file's dependencies section:

```toml
kodik-api = "0.3"
```

Or you can add with cargo

```sh
cargo add kodik-api
```

## Usage

```rs
use kodik_api::Client;
use kodik_api::search::SearchQuery;

#[tokio::main]
async fn main() {
    // KODIK_API_KEY=q8p5vnf9crt7xfyzke4iwc6r5rvsurv7
    let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");

    let client = Client::new(api_key);

    let search_response = SearchQuery::new()
        .with_title("Cyberpunk: Edgerunners")
        .with_limit(1)
        .execute(&client)
        .await
        .unwrap();

    println!("search response = {search_response:#?}");
    // search response = SearchResponse {
    //     time: "3ms",
    //     total: 1,
    //     prev_page: None,
    //     next_page: None,
    //     results: [
    //         Release {
    //             id: "serial-45534",
    //             title: "Киберпанк: Бегущие по краю",
    //             title_orig: "Cyberpunk: Edgerunners",
    //             other_title: Some("サイバーパンク エッジランナーズ"),
    //             link: "//kodik.info/serial/45534/d8619e900d122ea8eff8b55891b09bac/720p",
    //             year: 2022,
    //             kinopoisk_id: Some(
    //                 "2000102",
    //             ),
    //             imdb_id: Some(
    //                 "tt12590266",
    //             ),
    //             mdl_id: None,
    //             worldart_link: Some(
    //                 "http://www.world-art.ru/animation/animation.php?id=10534",
    //             ),
    //             shikimori_id: Some(
    //                 "42310",
    //             ),
    //             release_type: AnimeSerial,
    //             quality: WebDlRip720p,
    //             camrip: false,
    //             lgbt: false,
    //             translation: Translation {
    //                 id: 610,
    //                 title: "AniLibria.TV",
    //                 translation_type: Voice,
    //             },
    //             created_at: "2022-09-14T10:54:34Z",
    //             updated_at: "2022-09-23T22:31:33Z",
    //             blocked_seasons: Some(
    //                 {},
    //             ),
    //             seasons: None,
    //             last_season: Some(
    //                 1,
    //             ),
    //             last_episode: Some(
    //                 10,
    //             ),
    //             episodes_count: Some(
    //                 10,
    //             ),
    //             blocked_countries: [],
    //             material_data: None,
    //             screenshots: [
    //                 "https://i.kodik.biz/screenshots/seria/104981222/1.jpg",
    //                 "https://i.kodik.biz/screenshots/seria/104981222/2.jpg",
    //                 "https://i.kodik.biz/screenshots/seria/104981222/3.jpg",
    //                 "https://i.kodik.biz/screenshots/seria/104981222/4.jpg",
    //                 "https://i.kodik.biz/screenshots/seria/104981222/5.jpg",
    //             ],
    //         },
    //     ],
    // }
}
```

## Usage streams

```rs
use futures_util::{pin_mut, StreamExt};

use kodik_api::Client;
use kodik_api::list::ListQuery;
use kodik_api::types::ReleaseType;

#[tokio::main]
async fn main() {
    // KODIK_API_KEY=q8p5vnf9crt7xfyzke4iwc6r5rvsurv7
    let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");

    let client = Client::new(api_key);

    let stream = ListQuery::new()
        .with_limit(100)
        .with_types(&[ReleaseType::Anime, ReleaseType::AnimeSerial])
        .stream(&client);

    pin_mut!(stream);

    while let Some(response) = stream.next().await {
        match response {
            Ok(response) => {
                dbg!(response.total);
                dbg!(response.results);
            }
            Err(err) => {
                match err {
                    // Kodik error
                    kodik_api::error::Error::KodikError(message) => {
                        panic!("kodik error = {}", message);
                    }
                    // Reqwest error
                    kodik_api::error::Error::HttpError(_err) => {
                        // Another try
                        continue;
                    }
                    _ => {
                        panic!("Unknown error")
                    }
                }
            }
        }
    }
}
```
