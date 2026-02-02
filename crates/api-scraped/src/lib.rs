use std::sync::{Arc, atomic::AtomicUsize};

use futures::lock::Mutex;
use scraper::Selector;
use serde_json::Value;

use crate::{features::search::Search, init::Cookie};

pub fn format_data(data: Search) -> (String, String, String) {
    let d = data
        .items
        .iter()
        .map(|v| {
            (
                v.id,
                if v.voted { Some(v.rating) } else { None },
                v.favorite,
            )
        })
        .collect::<Vec<_>>();
    let ids = d.iter().map(|v| v.0.to_string()).collect::<Vec<_>>();
    let ratings = d
        .iter()
        .filter_map(|v| v.1.map(|a| format!("{}:{}", v.0, a)))
        .collect::<Vec<_>>();
    let favs = d
        .iter()
        .filter_map(|v| v.2.map(|a| format!("{}:{}", v.0, a)))
        .collect::<Vec<_>>();
    (ids.join("\n"), ratings.join("\n"), favs.join("\n"))
}

pub mod features;
mod fetch;
pub mod init;
#[cfg(test)]
mod tests;
pub mod unit;

pub trait CallbackTrait {
    fn call(&self, kind: &str, message: Value);
}

pub fn id(id: &str) -> Selector {
    Selector::parse(&format!("#{}", id)).expect("invalid selector")
}

pub fn selector(id: &str) -> Selector {
    Selector::parse(id).expect("invalid selector")
}

pub struct Session {
    clients: Vec<reqwest::Client>,
    rr: AtomicUsize,
    url_rewrite: Option<String>,
    pub cookie: Arc<Mutex<Cookie>>,
    pub callback: Arc<Mutex<Option<Box<dyn CallbackTrait>>>>,
}
