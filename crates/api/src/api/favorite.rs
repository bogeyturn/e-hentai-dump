use axum::{Json, extract::State};
use serde::Deserialize;

use crate::SharedState;

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
