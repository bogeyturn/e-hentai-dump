use axum::{Json, extract::State};
use serde::Deserialize;

use crate::SharedState;

#[derive(Deserialize)]
pub struct RatingRequest {
    pub gid: u64,
    pub rating: u8,
}

pub async fn rating(State(state): State<SharedState>, Json(req): Json<RatingRequest>) {
    let offset = state.info_db.seek.get(&req.gid);
    if let Some(&offset) = offset {
        let item = &state.info_db.items[offset];
        assert_eq!(item.gid, req.gid);
        state
            .rating_db
            .lock()
            .unwrap()
            .add(item.first_gid.unwrap_or(item.gid), req.rating);
    } else {
        state.rating_db.lock().unwrap().add(req.gid, req.rating);
    }
}
