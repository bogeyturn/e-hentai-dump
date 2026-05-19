mod api;
mod dbs;
mod gg;
mod hitomi;
mod search;

use std::{
    env,
    path::Path,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    http::HeaderValue,
    routing::{get, post},
};
use clap::Parser;
use db_creator::{Db, build};
use nord_proxy::ProxyTrait as _;
use reqwest::{Method, header};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::dbs::{FavDb, RatingDb};
pub type SharedState = Arc<AppState>;
pub struct AppState {
    pub info_db: Db,
    pub fav_db: Mutex<FavDb>,
    pub rating_db: Mutex<RatingDb>,
    pub client: api_scraped::Session,
}
#[derive(Parser, Debug)]
#[command(name = "server", about = "Tiny tokio server")]
struct Args {
    /// Host/IP to bind to (e.g. 127.0.0.1, 0.0.0.0)
    #[arg(short = 'h', long = "host", default_value = "0.0.0.0")]
    host: String,

    /// Port to bind to
    #[arg(short = 'p', long = "port", default_value_t = 8081)]
    port: u16,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let info_db = build();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let rating_db = RatingDb::load(Path::new("ratings"));
    let fav_db = FavDb::load(Path::new("favorites"));
    let mut c = api_scraped::Session::cookie("", "", "", None, None, None, None, None, None);
    let proxy = nord_proxy::Proxy::new().await;
    let mut proxies = proxy
        .proxies(&env::var("USER").unwrap(), &env::var("PASSWORD").unwrap())
        .into_iter()
        .filter(|v| v.load > 20)
        .collect::<Vec<_>>();
    proxies.sort_by_key(|v| v.load as i32 * 1);
    let p = &proxies[0..4];
    let p = p.iter().map(|v| v.proxy.clone()).collect::<Vec<_>>();
    c.set_proxies(p);
    let state = Arc::new(AppState {
        info_db,
        fav_db: Mutex::new(fav_db),
        rating_db: Mutex::new(rating_db),
        client: c.no_cookies(),
    });

    let app = Router::new()
        .route("/search", post(api::search::search))
        .route("/remove-favorite", post(api::favorite::favorite_delete))
        .route("/set-favorite", post(api::favorite::favorite))
        .route("/favorites", post(api::favorite::favorites))
        .route("/set-rating", post(api::rating::rating))
        .route("/grouped", post(api::compute_group::compute_group))
        .route("/ids", get(api::ids::ids))
        .route("/info", post(api::info::info))
        .nest_service("/imgs", ServeDir::new("./hit"))
        .layer(cors)
        .with_state(state);

    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("serving at {addr}");

    axum::serve(listener, app).await.unwrap();
}
