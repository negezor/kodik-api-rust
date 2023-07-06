//! # 🚀 Getting started
//!
//! ### Search Releases
//!
//! ```
//! use kodik::Client;
//! use kodik::search::SearchQuery;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let search_response = SearchQuery::new()
//!         .with_title("Cyberpunk: Edgerunners")
//!         .with_limit(1)
//!         .execute(&client)
//!         .await
//!         .unwrap();
//!
//!     println!("search response = {search_response:#?}");
//! }
//! ```
//!
//! ### List Releases
//!
//! ```
//! use futures::{pin_mut, StreamExt};
//!
//! use kodik::Client;
//! use kodik::list::ListQuery;
//! use kodik::types::ReleaseType;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let stream = ListQuery::new()
//!         .with_limit(100)
//!         .with_types(&[ReleaseType::Anime, ReleaseType::AnimeSerial])
//!         .stream(&client);
//!
//!     pin_mut!(stream);
//!
//!     while let Some(response) = stream.next().await {
//!         match response {
//!             Ok(response) => {
//!                 dbg!(response.total);
//!                 dbg!(response.results);
//!             }
//!             Err(err) => {
//!                 match err {
//!                     // Kodik error
//!                     kodik::error::Error::KodikError(message) => {
//!                         panic!("kodik error = {}", message);
//!                     }
//!                     // Reqwest error
//!                     kodik::error::Error::HttpError(_err) => {
//!                         // Another try
//!                         continue;
//!                     }
//!                     _ => {
//!                         panic!("Unknown error")
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ### List Translations
//!
//! ```
//! use kodik::Client;
//! use kodik::translations::TranslationQuery;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let translations_response = TranslationQuery::new()
//!         .execute(&client)
//!         .await
//!         .unwrap();
//!
//!     println!("translations response = {translations_response:#?}");
//! }
//! ```
//!
//! ### List Genres
//!
//! ```
//! use kodik::Client;
//! use kodik::genres::GenreQuery;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let genres_response = GenreQuery::new()
//!         .execute(&client)
//!         .await
//!         .unwrap();
//!
//!     println!("genres response = {genres_response:#?}");
//! }
//! ```
//!
//! ### List Countries
//!
//! ```
//! use kodik::Client;
//! use kodik::countries::CountryQuery;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let countries_response = CountryQuery::new()
//!         .execute(&client)
//!         .await
//!         .unwrap();
//!
//!     println!("countries response = {countries_response:#?}");
//! }
//! ```
//!
//! ### List Years
//!
//! ```
//! use kodik::Client;
//! use kodik::years::YearQuery;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let years_response = YearQuery::new()
//!         .execute(&client)
//!         .await
//!         .unwrap();
//!
//!     println!("years response = {years_response:#?}");
//! }
//! ```
//!
//! ### List Qualities
//!
//! ```
//! use kodik::Client;
//! use kodik::qualities::QualityQuery;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let qualities_response = QualityQuery::new()
//!         .execute(&client)
//!         .await
//!         .unwrap();
//!
//!     println!("qualities response = {qualities_response:#?}");
//! }
//! ```

/// Module containing the [`client::Client`] struct.
pub mod client;

/// Module containing the [`errors::Error`] struct.
pub mod error;

/// Module representing the [search releases] structures.
pub mod search;

/// Module representing the [list releases] structures.
pub mod list;

/// Module representing the [list translations] structures.
pub mod translations;

/// Module representing the [list years] structures.
pub mod years;

/// Module representing the [list countries] structures.
pub mod countries;

/// Module representing the [list genres] structures.
pub mod genres;

/// Module representing the [list qualities] structures.
pub mod qualities;

/// Module representing the [types] structures.
pub mod types;

/// The module contains structures for unifying the API seasons response.
pub mod unify_seasons;

mod constants;

pub use client::*;
