use std::u32;

use axum::{Json, extract::State};
use db_creator::{Category, ParseTag, TagPrefix, Torrents1};
use search_parser::parse_search;
use serde::{Deserialize, Serialize};

use crate::{
    SharedState,
    search::{Pagination, Search as _, SearchData, Unit},
};

#[derive(Deserialize)]
#[serde(default)]
pub struct SearchRequest {
    pub filter: String,
    pub page: Option<u64>,
    pub forward: bool,
    pub range: Option<u8>,
    pub date: Option<String>,
    pub jump: Option<String>,
    pub explunged: Option<bool>,
    pub torrent: bool,
    pub min_rating: f64,
    pub min_pages: u32,
    pub max_pages: u32,
    pub category_bits: u16,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            filter: String::new(),
            page: None,
            forward: true,
            explunged: Some(false),
            torrent: false,
            min_rating: 0.0,
            min_pages: 0,
            max_pages: u32::MAX,
            category_bits: Category::all().bits(),
            range: None,
            date: None,
            jump: None,
        }
    }
}

#[derive(Serialize)]
pub struct SearchResponse {
    progress: Option<f64>,
    first: bool,
    last: bool,
    count: Option<u64>,
    pub items: Vec<NewItem>,
}

#[derive(Serialize)]
pub struct NewItem {
    pub gid: u64,
    pub token: String,
    pub first_gid: Option<u64>,
    pub parent_gid: Option<u64>,

    pub title: String,
    pub title_jpn: Option<String>,
    pub thumb: String,
    pub category: Category,
    pub rating: f64,
    pub tags: Vec<ParseTag>,

    pub filecount: u32,
    pub filesize: u64,
    pub torrentcount: u32,
    pub torrents: Vec<Torrents1>,

    pub uploader: Option<String>,
    pub posted: u64,
    pub dumped: u64,
    pub expunged: bool,

    pub your_rating: Option<u8>,
    pub fav_group: Option<(u8, Option<String>)>,
}

pub async fn search(
    State(state): State<SharedState>,
    Json(req): Json<SearchRequest>,
) -> Json<SearchResponse> {
    let category = Category::from_bits(req.category_bits).unwrap_or_else(Category::all);
    let filters = parse_search(&req.filter);
    let pagination = if let Some(v) = req.range {
        Pagination::Range(v)
    } else if let Some(v) = req.date {
        Pagination::Seek(v.parse().unwrap())
    } else if let Some(v) = req.jump {
        if v.len() < 2 {
            panic!()
        }
        let suffix = &v[v.len() - 1..];
        let value = v[..v.len() - 1].parse().unwrap();
        Pagination::Jump {
            unit: match suffix {
                "d" => Unit::Day,
                "w" => Unit::Week,
                "m" => Unit::Month,
                "y" => Unit::Year,
                _ => panic!(),
            },
            value,
        }
    } else {
        Pagination::Id {
            id: req.page,
            forward: req.forward,
        }
    };
    let (out, p, count, f, l) = state.info_db.search(
        SearchData {
            filter: filters,
            pagination,
            explunged: req.explunged,
            torrent: req.torrent,
            min_rating: req.min_rating,
            min_pages: req.min_pages,
            max_pages: req.max_pages,
            category,
        },
        50,
    );
    let db = &state.info_db;

    Json(SearchResponse {
        progress: Some(p),
        count: Some(count),
        first: f,
        last: l,
        items: out
            .into_iter()
            .map(|v| NewItem {
                gid: v.gid,
                token: db.get_str(v.token).to_owned(),
                first_gid: v.first_gid,
                parent_gid: v.parent_gid,
                title: db.get_str(v.title).to_owned(),
                title_jpn: v.title_jpn.map(|v| db.get_str(v).to_owned()),
                thumb: format!("https://ehgt.org/{}", db.get_str(v.thumb).to_owned()),
                category: Category::from_bits(v.category).unwrap(),
                rating: v.rating,
                tags: db
                    .get_tags(v.tags.clone())
                    .into_iter()
                    .map(|v| ParseTag {
                        tag: db.get_tag(v.id).to_owned(),
                        prefix: TagPrefix::from(v.category),
                    })
                    .collect(),
                filecount: v.filecount,
                filesize: v.filesize,
                torrentcount: v.torrentcount,
                torrents: db
                    .get_torrents(v.torrents.clone())
                    .into_iter()
                    .map(|v| Torrents1 {
                        added: v.added,
                        fsize: v.fsize,
                        hash: db.get_str(v.hash).to_owned(),
                        name: v.name.map(|v| db.get_str(v).to_owned()),
                        tsize: v.tsize,
                    })
                    .collect(),
                uploader: v.uploader.map(|v| db.get_user(v).to_owned()),
                posted: v.posted,
                dumped: v.dumped,
                expunged: v.expunged,
                your_rating: state
                    .rating_db
                    .lock()
                    .unwrap()
                    .get(v.first_gid.unwrap_or(v.current_gid)),
                fav_group: state
                    .fav_db
                    .lock()
                    .unwrap()
                    .get(v.first_gid.unwrap_or(v.current_gid)),
            })
            .collect(),
    })
}
