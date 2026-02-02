use std::collections::HashMap;

use axum::{Json, extract::State};
use db_creator::{Tag, TagPrefix};
use search_parser::parse_search;
use serde::{Deserialize, Serialize};

use crate::{
    SharedState,
    search::{SearchData, filter_func},
};

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.rank[ra] < self.rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        if self.rank[ra] == self.rank[rb] {
            self.rank[ra] += 1;
        }
    }
}

pub fn merge_on_overlap(mut groups: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let n = groups.len();
    if n == 0 {
        return vec![];
    }

    for g in &mut groups {
        g.sort_unstable();
        g.dedup();
    }

    let mut dsu = Dsu::new(n);

    let mut owner: HashMap<u64, usize> = HashMap::new();

    for (i, g) in groups.iter().enumerate() {
        for &x in g {
            if let Some(&j) = owner.get(&x) {
                // overlap: group i shares element x with group j
                dsu.union(i, j);
            } else {
                owner.insert(x, i);
            }
        }
    }

    let mut merged: HashMap<usize, Vec<u64>> = HashMap::new();
    for (i, g) in groups.into_iter().enumerate() {
        let r = dsu.find(i);
        merged.entry(r).or_default().extend(g);
    }

    let mut out: Vec<Vec<u64>> = merged.into_values().collect();
    for v in &mut out {
        v.sort_unstable();
        v.dedup();
    }

    out
}

pub fn prepare_string(input: &str) -> Option<String> {
    let mut raw = String::with_capacity(input.len());

    let mut paren = 0usize; // ()
    let mut square = 0usize; // []
    let mut curly = 0usize; // {}

    for ch in input.chars() {
        match ch {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '[' => square += 1,
            ']' => square = square.saturating_sub(1),
            '{' => curly += 1,
            '}' => curly = curly.saturating_sub(1),
            _ => {
                if paren == 0 && square == 0 && curly == 0 {
                    raw.push(ch);
                }
            }
        }
    }

    let s = raw
        .split_whitespace()
        .filter(|v| v.len() > 0)
        .collect::<Vec<_>>()
        .join(" ");

    if s.len() < 3 { None } else { Some(s) }
}

#[derive(Serialize, Deserialize)]
pub struct Groups {
    count: usize,
    items: Vec<Vec<u64>>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupRequest {
    filter: Option<String>,
}

pub async fn compute_group(
    State(state): State<SharedState>,
    Json(req): Json<GroupRequest>,
) -> Json<Groups> {
    let filter = req
        .filter
        .as_ref()
        .map(|v| parse_search(v))
        .unwrap_or_default();
    let sd = SearchData {
        filter,
        explunged: None,
        ..Default::default()
    };
    let mut hm: HashMap<(&Tag, String), Vec<u64>> = HashMap::new();
    for item in state.info_db.items.iter() {
        if item.gid != item.current_gid {
            continue;
        }
        if !filter_func(&state.info_db, item, &sd) {
            continue;
        }
        let tags = state.info_db.get_tags(item.tags.clone());
        for user_id in tags.iter().filter(|v| {
            v.category == TagPrefix::Artist as u8 || v.category == TagPrefix::Group as u8
        }) {
            let str = prepare_string(state.info_db.get_str(item.title));
            let str = match str {
                Some(s) => s,
                None => continue,
            };
            hm.entry((user_id, str)).or_default().push(item.gid);
            if let Some(v) = item.title_jpn {
                let str = prepare_string(state.info_db.get_str(v));
                let str = match str {
                    Some(s) => s,
                    None => continue,
                };
                hm.entry((user_id, str)).or_default().push(item.gid);
            }
        }
    }

    let merge = merge_on_overlap(hm.into_values().collect())
        .into_iter()
        .filter(|v| v.len() > 1)
        .collect::<Vec<_>>();

    Json(Groups {
        count: merge.len(),
        items: merge,
    })
}
