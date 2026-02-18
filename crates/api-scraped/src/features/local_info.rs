use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    Session,
    features::{
        info::{self, Parent},
        mpv,
    },
};

#[derive(Serialize)]
pub struct InfoRequest {
    pub gid: u64,
    pub page: u32,
    pub hitomi: bool,
}

#[derive(Deserialize)]
pub struct ImagePage {
    pub id: u32,
    pub ratio: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub key: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
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

impl From<ImagePage> for mpv::ImagePage {
    fn from(value: ImagePage) -> Self {
        mpv::ImagePage {
            url: value.url,
            width: value.width,
            height: value.height,
            id: value.id,
            ratio: value.ratio,
            key: value.key,
            name: value.name,
        }
    }
}

impl Session {
    pub async fn info_local(&self, id: u64, _: &str, page: u32) -> anyhow::Result<info::Info> {
        let info: Info = self
            .local_api(
                "info",
                &InfoRequest {
                    gid: id,
                    page,
                    hitomi: false,
                },
            )
            .await?
            .json()
            .await?;
        Ok(info::Info {
            id,
            token: info.token,
            thumb: info.thumb.into(),
            tags: info.tags,
            rating: info.rating,
            newer: info.newer,
            category: info.category,
            title: info.title,
            alt_title: info.alt_title,
            per_page: info.per_page,
            pages: info.pages.into_iter().map(|v| v.into()).collect(),
            posted: info.posted,
            files: info.files,
            size: info.size,
            visible: info.visible,
            language: info.language,
            uploader: info.uploader,
            parent: info.parent.map(|v| Parent { id: v.0, key: v.1 }),
            favorited: info.favorited,
            favorite: info.favorite,
            my_stars: info.my_stars,
            apiuid: None,
            uploader_id: None,
            apikey: None,
            comments: Vec::new(),
        })
    }
}
