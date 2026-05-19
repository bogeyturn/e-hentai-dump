use axum::{Json, extract::State};
use db_creator::{Category, ParseTag, TagPrefix, Torrents1};
use serde::{Deserialize, Serialize};

use crate::{SharedState, api::search::NewItem};

#[derive(Deserialize)]
pub struct FavoriteRequest {
    pub gid: u64,
    pub fav: u8,
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct FavoriteDeleteRequest {
    pub gid: u64,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FavoriteSort {
    Gid,
    #[default]
    Added,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct FavoritesRequest {
    pub category: Option<u8>,
    pub sort: FavoriteSort,
    pub page: u64,
    pub page_size: u16,
}

impl Default for FavoritesRequest {
    fn default() -> Self {
        Self {
            category: None,
            sort: FavoriteSort::Added,
            page: 0,
            page_size: 50,
        }
    }
}

#[derive(Serialize)]
pub struct FavoritesResponse {
    pub first: bool,
    pub last: bool,
    pub count: u64,
    pub page: u64,
    pub page_size: u16,
    pub items: Vec<NewItem>,
}

struct FavoriteItem {
    item_idx: usize,
    gid: u64,
    added: u64,
    fav_group: (u8, Option<String>),
}
pub async fn favorite_delete(
    State(state): State<SharedState>,
    Json(req): Json<FavoriteDeleteRequest>,
) {
    let offset = state.info_db.seek.get(&req.gid);
    if let Some(&offset) = offset {
        let item = &state.info_db.items[offset];
        assert_eq!(item.gid, req.gid);
        state
            .fav_db
            .lock()
            .unwrap()
            .remove(item.first_gid.unwrap_or(item.gid));
    }
}

pub async fn favorite(State(state): State<SharedState>, Json(req): Json<FavoriteRequest>) {
    let offset = state.info_db.seek.get(&req.gid);
    if let Some(&offset) = offset {
        let item = &state.info_db.items[offset];
        assert_eq!(item.gid, req.gid);
        state
            .fav_db
            .lock()
            .unwrap()
            .add(item.first_gid.unwrap_or(item.gid), req.fav, req.note);
    }
}

pub async fn favorites(
    State(state): State<SharedState>,
    Json(req): Json<FavoritesRequest>,
) -> Json<FavoritesResponse> {
    let mut favorites = {
        state
            .fav_db
            .lock()
            .unwrap()
            .entries()
            .into_iter()
            .filter_map(|(gid, value)| {
                if req
                    .category
                    .is_some_and(|category| value.category != category)
                {
                    return None;
                }

                let first_offset = *state.info_db.seek.get(&gid)?;
                let first_item = &state.info_db.items[first_offset];
                let current_gid = first_item.current_gid;
                let item_idx = state
                    .info_db
                    .seek
                    .get(&current_gid)
                    .copied()
                    .unwrap_or(first_offset);

                Some(FavoriteItem {
                    item_idx,
                    gid: current_gid,
                    added: value.added,
                    fav_group: (value.category, value.note),
                })
            })
            .collect::<Vec<_>>()
    };

    match req.sort {
        FavoriteSort::Gid => {
            favorites.sort_by(|a, b| b.gid.cmp(&a.gid).then_with(|| b.added.cmp(&a.added)))
        }
        FavoriteSort::Added => {
            favorites.sort_by(|a, b| b.added.cmp(&a.added).then_with(|| b.gid.cmp(&a.gid)))
        }
    }

    let page_size = req.page_size.clamp(1, 200) as usize;
    let count = favorites.len();
    let start = req.page.saturating_mul(page_size as u64).min(count as u64) as usize;
    let end = (start + page_size).min(count);
    let first = start == 0;
    let last = end >= count;
    let db = &state.info_db;
    let ratings = state.rating_db.lock().unwrap();
    let items = favorites[start..end]
        .iter()
        .map(|entry| {
            let v = &db.items[entry.item_idx];
            NewItem {
                gid: v.gid,
                token: db.get_str(v.token).to_owned(),
                first_gid: v.first_gid,
                parent_gid: v.parent_gid,
                title: db.get_str(v.title).to_owned(),
                title_jpn: v.title_jpn.map(|v| db.get_str(v).to_owned()),
                thumb: format!("https://ehgt.org/{}", db.get_str(v.thumb)),
                category: Category::from_bits(v.category).unwrap(),
                rating: v.rating,
                tags: db
                    .get_tags(v.tags.clone())
                    .iter()
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
                    .iter()
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
                your_rating: ratings.get(v.first_gid.unwrap_or(v.current_gid)),
                fav_group: Some(entry.fav_group.clone()),
            }
        })
        .collect();

    Json(FavoritesResponse {
        first,
        last,
        count: count as u64,
        page: req.page,
        page_size: page_size as u16,
        items,
    })
}
