use axum::{Json, extract::State};
use serde::Deserialize;

use crate::SharedState;

#[derive(Deserialize)]
pub struct FavoriteRequest {
    pub gid: u64,
    pub fav: u8,
    pub note: Option<String>,
}

pub async fn favorite(State(state): State<SharedState>, Json(req): Json<FavoriteRequest>) {
    state.fav_db.lock().unwrap().add(req.gid, req.fav, req.note);
}
