mod api;
mod dbs;
mod search;

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    routing::{get, post},
};
use db_creator::{Db, build};

use crate::dbs::{FavDb, RatingDb};
pub type SharedState = Arc<AppState>;
pub struct AppState {
    pub info_db: Db,
    pub fav_db: Mutex<FavDb>,
    pub rating_db: Mutex<RatingDb>,
}
#[tokio::main]
async fn main() {
    let info_db = build();

    let rating_db = RatingDb::load(Path::new("ratings"));
    let fav_db = FavDb::load(Path::new("ratings"));
    let state = Arc::new(AppState {
        info_db,
        fav_db: Mutex::new(fav_db),
        rating_db: Mutex::new(rating_db),
    });

    let app = Router::new()
        .route("/search", post(api::search::search))
        .route("/set-favorite", post(api::favorite::favorite))
        .route("/set-rating", post(api::rating::rating))
        .route("/grouped", post(api::compute_group::compute_group))
        .route("/ids", get(api::ids::ids))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("serving at 0.0.0.0:8081");

    axum::serve(listener, app).await.unwrap();
}
