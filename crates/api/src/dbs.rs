use std::io::{Seek as _, SeekFrom, Write};
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
        Some((a, b)) => (a.parse().unwrap(), Some(b.to_owned())),
        None => (rest.parse().unwrap(), None),
    };
    (id, (rating, title))
}

impl FavDb {
    pub fn load(path: &Path) -> Self {
        let _ = OpenOptions::new().write(true).create(true).open(path);
        let data = read_to_string(path)
            .unwrap()
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(parse_info)
            .collect::<BTreeMap<_, _>>();
        for (_, (id, _)) in &data {
            assert!(*id < 10)
        }
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
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
        self.file.seek(SeekFrom::End(0)).unwrap();
        writeln!(&mut self.file, "{}", playload).unwrap();
        self.file.flush().unwrap();
    }

    pub fn remove(&mut self, id: u64) -> Option<(u8, Option<String>)> {
        let removed = self.data.remove(&id)?;
        self.rewrite_file();
        Some(removed)
    }

    fn rewrite_file(&mut self) {
        self.file.set_len(0).unwrap();
        self.file.seek(SeekFrom::Start(0)).unwrap();

        let items: Vec<_> = self.data.iter().collect();

        for (&id, (rating, text)) in items {
            let prefix = format!("{id}:{rating}");
            let payload = text
                .as_ref()
                .map(|v| format!("{prefix}:{}", v))
                .unwrap_or(prefix);

            writeln!(&mut self.file, "{}", payload).unwrap();
        }

        self.file.flush().unwrap();
    }
}

pub struct RatingDb {
    data: BTreeMap<u64, u8>,
    file: File,
}

impl RatingDb {
    pub fn load(path: &Path) -> Self {
        let data = read_to_string(path)
            .unwrap()
            .lines()
            .map(parse_info2)
            .collect::<BTreeMap<_, _>>();
        for (_, id) in &data {
            assert!(*id < 11)
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
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
