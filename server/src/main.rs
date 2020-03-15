use std::env;

use lazy_static::lazy_static;
use warp::{self, filters::BoxedFilter, Filter};

static _WORD_FILE_CONTENT: &str = include_str!("words_alpha.txt");

lazy_static! {
    static ref WORD_LIST: Vec<String> = _WORD_FILE_CONTENT
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();
    let routes = route_search();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
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
    Ok(warp::reply::html(format!(
        "Param: {}, is valid word: {}",
        path_val,
        WORD_LIST.contains(&path_val)
    )))
}
