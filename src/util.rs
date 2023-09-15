//! Utiliy fns that do not fit anywhere else

use http::{
    header::{HOST, REFERER},
    HeaderMap,
};

/// Returns http(s)://domain, depending on what is used inside the headers
pub fn base_url(headers: &HeaderMap) -> String {
    let referer = headers
        .get(REFERER)
        .map_or("https://", |header_value| header_value.to_str().unwrap());
    let splits: Vec<&str> = referer.split(':').collect();
    let proto = match splits[..] {
        [proto, ..] => proto,
        _ => "http",
    };
    let host = headers[HOST].to_str().expect("Unable te get HOST header");
    format!("{proto}://{host}")
}
