use std::time::Duration;

use axum::{Json, extract::State};
use db_creator::{Category, TagPrefix};
use serde::{Deserialize, Serialize};

use crate::{SharedState, hitomi::run_hitomi};

#[derive(Deserialize)]
pub struct InfoRequest {
    pub gid: u64,
    pub page: u32,
    pub hitomi: bool,
}

#[derive(Serialize)]
pub struct ImagePage {
    pub id: u32,
    pub ratio: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub key: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct Info {
    pub id: u64,
    pub token: String,
    pub thumb: ImagePage,
    pub tags: Vec<String>,
    pub rating: Option<f64>,
    pub newer: Vec<(u64, String)>,
    pub category: String,
    pub title: String,
    pub alt_title: Option<String>,
    pub per_page: u32,
    pub pages: Vec<ImagePage>,
    pub posted: Duration,
    pub files: u32,
    pub size: u64,
    pub visible: bool,
    pub language: String,
    pub uploader: String,
    pub parent: Option<(u64, String)>,
    pub favorited: u64,
    pub favorite: Option<u8>,
    pub my_stars: Option<u8>,
}

pub async fn info(State(state): State<SharedState>, Json(req): Json<InfoRequest>) -> Json<Info> {
    let offset = *state.info_db.seek.get(&req.gid).unwrap();
    let item = &state.info_db.items[offset];
    let (pages, per_page, favorited) = match req.hitomi {
        true => {
            let per_page = 50;
            let pages = run_hitomi(req.gid, req.page as u64, 50, 8081).unwrap();
            (pages, per_page, 0)
        }
        false => {
            let info = state
                .client
                .info(req.gid, state.info_db.get_str(item.token), req.page)
                .await
                .unwrap();
            (
                info.pages
                    .into_iter()
                    .map(|p| ImagePage {
                        id: p.id,
                        ratio: p.ratio,
                        width: p.width,
                        height: p.height,
                        key: p.key,
                        name: p.name,
                        url: p.url,
                    })
                    .collect::<Vec<_>>(),
                info.per_page,
                info.favorited,
            )
        }
    };

    let tags = state.info_db.get_tags(item.tags.clone());
    let l = tags
        .iter()
        .filter(|v| v.category == TagPrefix::Language as u8)
        .map(|v| state.info_db.get_tag(v.id))
        .filter(|v| *v != "translated")
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();
    assert!(l.len() < 2);
    let tags = tags
        .iter()
        .map(|v| {
            format!(
                "{}:{}",
                TagPrefix::from(v.category).to_string(),
                state.info_db.get_tag(v.id)
            )
        })
        .collect::<Vec<String>>();
    let newer = if let Some(v) = item.first_gid {
        state
            .info_db
            .items
            .iter()
            .filter(|v| v.gid > req.gid && v.first_gid == item.first_gid)
            .map(|v| (v.gid, state.info_db.get_str(v.token).to_owned()))
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    Json(Info {
        id: item.gid,
        token: state.info_db.get_str(item.token).to_owned(),
        tags,
        language: l.get(0).map(|v| v.to_owned()).unwrap_or_default(),
        rating: Some(item.rating),
        newer,
        category: Category::from_bits(item.category).unwrap().to_string(),
        title: state.info_db.get_str(item.title).to_owned(),
        alt_title: item.title_jpn.map(|v| state.info_db.get_str(v).to_owned()),
        files: item.filecount,
        size: item.filesize,
        posted: Duration::from_secs(item.posted),
        uploader: item
            .uploader
            .map(|v| state.info_db.get_user(v).to_owned())
            .unwrap_or_default(),
        parent: {
            if let Some(p) = item.parent_gid {
                let offset = *state.info_db.seek.get(&p).unwrap();
                let item = &state.info_db.items[offset];
                Some((item.gid, state.info_db.get_str(item.token).to_owned()))
            } else {
                None
            }
        },
        favorite: state
            .fav_db
            .lock()
            .unwrap()
            .get(item.first_gid.unwrap_or(item.gid))
            .map(|v| v.0),
        my_stars: state
            .rating_db
            .lock()
            .unwrap()
            .get(item.first_gid.unwrap_or(item.gid)),
        visible: !item.expunged,
        per_page,
        thumb: ImagePage {
            id: 0,
            ratio: pages[0].ratio,
            url: format!(
                "https://ehgt.org/{}",
                state.info_db.get_str(item.thumb).to_owned()
            ),
            width: pages[0].width,
            height: pages[0].height,
            key: "".to_owned(),
            name: "".to_owned(),
        },
        pages,
        favorited,
    })
}
