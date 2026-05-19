use axum::{Json, extract::State};

use crate::SharedState;

pub async fn ids(State(state): State<SharedState>) -> Json<Vec<u64>> {
    Json(state.info_db.items.iter().map(|v| v.gid).collect())
}
