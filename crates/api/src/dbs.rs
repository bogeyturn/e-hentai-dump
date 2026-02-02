use std::io::Write;
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions, read_to_string},
    path::Path,
};

use base64::{Engine as _, engine::general_purpose::STANDARD};

pub struct FavDb {
    data: BTreeMap<u64, (u8, Option<String>)>,
    file: File,
}

fn split_once(s: &str) -> (u64, &str) {
    s.split_once(":")
        .map(|v| (v.0.parse().unwrap(), v.1))
        .unwrap()
}

fn parse_info2(s: &str) -> (u64, u8) {
    let (id, rest) = split_once(s);
    let rating = rest.parse().unwrap();
    (id, rating)
}

fn parse_info(s: &str) -> (u64, (u8, Option<String>)) {
    let (id, rest) = split_once(s);
    let rating = rest.split_once(":");

    let (rating, title) = match rating {
        Some((a, b)) => (
            a.parse().unwrap(),
            Some(String::from_utf8(STANDARD.decode(b).unwrap()).unwrap()),
        ),
        None => (rest.parse().unwrap(), None),
    };
    (id, (rating, title))
}

impl FavDb {
    pub fn load(path: &Path) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        let data = read_to_string(path)
            .unwrap()
            .lines()
            .map(parse_info)
            .collect();
        Self { data, file }
    }
    pub fn get(&self, id: u64) -> Option<(u8, Option<String>)> {
        self.data.get(&id).cloned()
    }

    pub fn add(&mut self, id: u64, rating: u8, title: Option<String>) {
        self.data.insert(id, (rating, title.clone()));
        let prefix = format!("{id}:{rating}");
        let playload = title
            .map(|v| format!("{prefix}:{}", STANDARD.encode(v)))
            .unwrap_or(prefix);
        writeln!(&mut self.file, "{}", playload).unwrap();
    }
}

pub struct RatingDb {
    data: BTreeMap<u64, u8>,
    file: File,
}

impl RatingDb {
    pub fn load(path: &Path) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        let data = read_to_string(path)
            .unwrap()
            .lines()
            .map(parse_info2)
            .collect();
        Self { data, file }
    }
    pub fn get(&self, id: u64) -> Option<u8> {
        self.data.get(&id).copied()
    }

    pub fn add(&mut self, id: u64, rating: u8) {
        self.data.insert(id, rating);

        writeln!(&mut self.file, "{id}:{rating}").unwrap();
    }
}
