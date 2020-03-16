use std::{collections::HashMap, env};

use lazy_static::lazy_static;
use serde::Serialize;
use stringsort::insertsort;
use warp::{self, filters::BoxedFilter, http::Method, Filter};

// Include the English words in the binary.
static _WORD_FILE_CONTENT: &str = include_str!("words_alpha.txt");

lazy_static! {
    // Parse the English word list string into a map of sorted word to original word.
    static ref WORD_LOOKUP: HashMap<String, Vec<String>> = {
        let word_list: Vec<&str> = _WORD_FILE_CONTENT.split_whitespace().collect();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        word_list.iter().for_each(|&word| {
            let owned = word.to_owned();
            let sorted = insertsort(&owned);
            if let Some(existing) = map.get_mut(&sorted) {
                existing.push(owned);
            } else {
                map.insert(sorted, vec![owned]);
            }
        });
        map
    };
}

/// JSON struct for the search handler.
#[derive(Debug, Serialize)]
struct SearchResponse {
    original: String,
    sorted: String,
    matches: Vec<String>,
}

impl SearchResponse {
    /// Create a new response struct, processing the string.
    fn new(original: &str) -> Self {
        let original = original.to_owned();
        let sorted = insertsort(&original);
        let matches = match WORD_LOOKUP.get(&sorted) {
            Some(s) => s.to_owned(),
            None => vec![],
        };
        Self {
            original,
            sorted,
            matches,
        }
    }
}

/// Return the route for /search/:String
fn route_search() -> BoxedFilter<(impl warp::Reply,)> {
    warp::get()
        .and(warp::path("word"))
        .and(warp::path::param())
        .and_then(handle_search)
        .with(warp::cors().allow_methods(&[Method::GET]))
        .boxed()
}

/// Return the handler for taking letters and matching
/// them against the list of English words.
async fn handle_search(path_val: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&SearchResponse::new(&path_val)))
}

/// Entry point.
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let routes = route_search();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
