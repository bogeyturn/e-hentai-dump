use axum::{Json, extract::State};
use serde::Deserialize;

use crate::SharedState;

#[derive(Deserialize)]
pub struct RatingRequest {
    pub gid: u64,
    pub rating: u8,
}

pub async fn rating(State(state): State<SharedState>, Json(req): Json<RatingRequest>) {
    state.rating_db.lock().unwrap().add(req.gid, req.rating);
}
