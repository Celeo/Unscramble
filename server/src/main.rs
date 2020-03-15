use std::{collections::HashMap, env};

use lazy_static::lazy_static;
use serde::Serialize;
use stringsort::insertsort;
use warp::{self, filters::BoxedFilter, Filter};

// Include the English words in the binary.
static _WORD_FILE_CONTENT: &str = include_str!("words_alpha.txt");

lazy_static! {
    // Parse the English word list string into a map of sorted word to original word.
    static ref WORD_LOOKUP: HashMap<String, String> = {
        let word_list: Vec<&str> = _WORD_FILE_CONTENT.split_whitespace().collect();
        let mut map = HashMap::new();
        word_list.iter().for_each(|&word| {
            map.insert(insertsort(word), word.to_owned());
        });
        map
    };
}

/// JSON struct for the search handler.
#[derive(Debug, Serialize)]
struct SearchResponse {
    original: String,
    sorted: String,
    matches: Option<String>,
}

impl SearchResponse {
    /// Create a new response struct, processing the string.
    fn new(word: &str) -> Self {
        let sorted = insertsort(word);
        SearchResponse {
            original: word.to_owned(),
            sorted: sorted.clone(),
            matches: WORD_LOOKUP.get(&sorted).map(|s| s.to_owned()),
        }
    }
}

/// Return the route for /search/:String
fn route_search() -> BoxedFilter<(impl warp::Reply,)> {
    warp::get()
        .and(warp::path("word"))
        .and(warp::path::param())
        .and_then(handle_search)
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
