use api_scraped::{
    Session,
    features::{
        bounty::{BountyInfo, BountyPage, BountyStatus, BountyType},
        comment_vote::CommentVote,
        exchange::ExchangePage,
        fav::{self, Favorites},
        hentai_at_home::HentaiAtHome,
        home::HomeInfo,
        info::Info,
        logs::{CreditLog, KarmaPage},
        manage_folders::Folder,
        mpv::ImagePage,
        news::News,
        perks::PerkPage,
        reader,
        search::{AdvancedConfig, Search, SearchInfo, SearchQuery},
        stats::StatsPage,
        toplists::{ToplistItem, Toplists},
        torrents::{Sort, TorrentPage, TorrentStatus},
        upload::{GalleryPage, UploadGalleryInfo},
    },
};
use log::Level;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen(start)]
pub fn _start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Info).expect("error initializing logger");
}

#[wasm_bindgen]
pub struct WasmSession {
    inner: Session,
}

#[wasm_bindgen]
impl WasmSession {
    /// Create a new session. `cookie_str` is your whole cookie header.
    #[wasm_bindgen(constructor)]
    pub fn new(
        cookie_str: String,
        proxy: Option<String>,
        local_api: Option<String>,
    ) -> WasmSession {
        WasmSession {
            inner: Session::new(cookie_str, proxy, None, local_api),
        }
    }

    #[wasm_bindgen(js_name = cookie)]
    pub async fn cookie_js(&self) -> String {
        self.inner.cookie.lock().await.to_string()
    }

    /// search({ query?: string, pid?: number, forward?: boolean }) -> Promise<object>
    #[wasm_bindgen(js_name = search)]
    pub async fn search_js(
        &self,
        query: Option<String>,
        pid: Option<u64>,
        range: Option<u8>,
        seek: Option<String>,
        jump: Option<String>,
        forward: Option<bool>,
        advanced: Option<AdvancedConfig>,
        categories: Option<u16>,
        local: bool,
    ) -> Result<Search, JsValue> {
        let out = if local {
            self.inner
                .local_search(SearchQuery {
                    query,
                    pid,
                    forward: forward.unwrap_or(true),
                    advanced,
                    cat: categories,
                    range,
                    seek,
                    jump,
                })
                .await
        } else {
            self.inner
                .search(SearchQuery {
                    query,
                    pid,
                    forward: forward.unwrap_or(true),
                    advanced,
                    cat: categories,
                    range,
                    seek,
                    jump,
                })
                .await
        }
        .map_err(js_err)?;

        Ok(out)
    }

    /// info(id: number, token: string) -> Promise<object>
    #[wasm_bindgen(js_name = info)]
    pub async fn info_js(
        &self,
        id: u64,
        token: String,
        page: u32,
        local: bool,
    ) -> Result<Info, JsValue> {
        let out = if local {
            match self.inner.info_local(id, &token, page).await {
                Ok(info) => Ok(info),
                Err(_) => self.inner.info(id, &token, page).await,
            }
        } else {
            self.inner.info(id, &token, page).await
        };
        Ok(out.map_err(js_err)?)
    }

    /// nextImg(id: number, token: string, idx: number, aa?: string) -> Promise<object>
    #[wasm_bindgen(js_name = nextImg)]
    pub async fn next_img_js(
        &self,
        id: u64,
        token: String,
        idx: u32,
        showkey: Option<String>,
    ) -> Result<reader::Resp, JsValue> {
        let out = self
            .inner
            .next_img(id, &token, idx, showkey)
            .await
            .map_err(js_err)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = rateGallery)]
    pub async fn rate_gallery_js(
        &self,
        gid: u64,
        token: String,
        apiuid: i64,
        apikey: String,
        rating: u8,
        local: bool,
    ) -> Result<(), JsValue> {
        if local {
            self.inner
                .rate_local(gid, &token, apiuid, &apikey, rating)
                .await
                .map_err(js_err)?;
        } else {
            self.inner
                .rate(gid, &token, apiuid, &apikey, rating)
                .await
                .map_err(js_err)?;
        }
        Ok(())
    }

    #[wasm_bindgen(js_name = addFavorite)]
    pub async fn add_favorite_js(
        &self,
        gid: u64,
        token: String,
        favcat: u8,
        favnote: String,
        local: bool,
    ) -> Result<(), JsValue> {
        if local {
            self.inner
                .add_favorite_local(gid, &token, favcat, &favnote)
                .await
                .map_err(js_err)?;
        } else {
            self.inner
                .add_favorite(gid, &token, favcat, &favnote)
                .await
                .map_err(js_err)?;
        }
        Ok(())
    }

    #[wasm_bindgen(js_name = removeFavorite)]
    pub async fn remove_favorite_js(
        &self,
        gid: u64,
        token: String,
        local: bool,
    ) -> Result<(), JsValue> {
        if local {
            self.inner
                .remove_favorite_local(gid, &token)
                .await
                .map_err(js_err)?;
        } else {
            self.inner
                .remove_favorite(gid, &token)
                .await
                .map_err(js_err)?;
        }
        Ok(())
    }

    #[wasm_bindgen(js_name = voteComment)]
    pub async fn vote_comment_js(
        &self,
        gid: u64,
        token: String,
        comment_id: u64,
        upvote: bool,
        apiuid: i64,
        apikey: String,
    ) -> Result<CommentVote, JsValue> {
        let data = self
            .inner
            .vote_comment(gid, &token, comment_id, upvote, apiuid, &apikey)
            .await
            .map_err(js_err)?;
        Ok(data)
    }

    #[wasm_bindgen(js_name = voteTag)]
    pub async fn vote_tag_js(
        &self,
        gid: u64,
        token: String,
        tag: &str,
        upvote: bool,
        apiuid: i64,
        apikey: String,
    ) -> Result<Option<String>, JsValue> {
        let data = self
            .inner
            .tag_vote(gid, &token, tag, upvote, apiuid, &apikey)
            .await
            .map_err(js_err)?;
        Ok(data)
    }

    #[wasm_bindgen(js_name = addComment)]
    pub async fn add_comment_js(
        &self,
        gid: u64,
        token: String,
        comment: &str,
    ) -> Result<(), JsValue> {
        self.inner
            .comment_new(gid, &token, comment)
            .await
            .map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = updateComment)]
    pub async fn update_comment_js(
        &self,
        gid: u64,
        token: String,
        comment_id: u64,
        comment: &str,
    ) -> Result<(), JsValue> {
        self.inner
            .comment_update(gid, &token, comment_id, comment)
            .await
            .map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = mpv)]
    pub async fn mpv_js(&self, gid: u64, token: String) -> Result<Vec<ImagePage>, JsValue> {
        let data = self.inner.mpv_info(gid, &token).await.map_err(js_err)?;
        Ok(data)
    }

    #[wasm_bindgen(js_name = auto_reorder_folder)]
    pub async fn auto_reorder_folder_js(&self) -> Result<(), JsValue> {
        self.inner.auto_reorder_folder().await.map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = create_folder)]
    pub async fn create_folder_js(&self, folder: &str) -> Result<(), JsValue> {
        self.inner.create_folder(folder).await.map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = delete_folder)]
    pub async fn delete_folder_js(&self, folder_id: u32) -> Result<(), JsValue> {
        self.inner.delete_folder(folder_id).await.map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = credit_logs)]
    pub async fn credit_logs_js(&self) -> Result<Vec<CreditLog>, JsValue> {
        let info = self.inner.credit_logs().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = donate_info)]
    pub async fn donate_info_js(&self) -> Result<(), JsValue> {
        let info = self.inner.donate_info().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = exchange_info)]
    pub async fn exchange_info_js(&self, hath: bool) -> Result<ExchangePage, JsValue> {
        let info = self.inner.exchange_info(hath).await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = exchange_ask)]
    pub async fn exchange_ask_js(&self, hath: bool, count: u32, price: u32) -> Result<(), JsValue> {
        self.inner
            .exhange_ask(hath, count, price)
            .await
            .map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = exchange_bid)]
    pub async fn exchange_bid_js(&self, hath: bool, count: u32, price: u32) -> Result<(), JsValue> {
        self.inner
            .exhange_bid(hath, count, price)
            .await
            .map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = tos)]
    pub async fn tos_js(&self) -> Result<String, JsValue> {
        let info = self.inner.tos().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = perks)]
    pub async fn perks_js(&self) -> Result<PerkPage, JsValue> {
        let info = self.inner.perks().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = news)]
    pub async fn news_js(&self) -> Result<Vec<News>, JsValue> {
        let info = self.inner.news().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = popular)]
    pub async fn popular_js(&self) -> Result<Vec<SearchInfo>, JsValue> {
        let info = self.inner.popular().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = list_upload)]
    pub async fn list_upload_js(&self) -> Result<GalleryPage, JsValue> {
        let info = self.inner.list_upload().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = folder_info)]
    pub async fn folder_info_js(&self) -> Result<Vec<Folder>, JsValue> {
        let info = self.inner.folder_info().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = karma_logs)]
    pub async fn karma_logs_js(&self) -> Result<KarmaPage, JsValue> {
        let info = self.inner.karma_logs().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = reorder_folder)]
    pub async fn reorder_folder_js(&self, folders: Vec<Folder>) -> Result<(), JsValue> {
        self.inner.reorder_folder(folders).await.map_err(js_err)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = toplists)]
    pub async fn toplists_js(&self) -> Result<Vec<Toplists>, JsValue> {
        let info = self.inner.toplists().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = hentai_at_home)]
    pub async fn hentai_at_home_js(&self) -> Result<HentaiAtHome, JsValue> {
        let info = self.inner.hentai_at_home().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = toplist)]
    pub async fn toplist_js(&self, id: u32, page: u32) -> Result<Vec<ToplistItem>, JsValue> {
        let info = self.inner.toplist(id, page).await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = torrents)]
    pub async fn torrents_js(
        &self,
        query: &str,
        page: u32,
        status: TorrentStatus,
        my_torrents: Option<u32>,
        order: Sort,
        asc: bool,
    ) -> Result<TorrentPage, JsValue> {
        let info = self
            .inner
            .torrents(query, page, status, my_torrents, order, asc)
            .await
            .map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = list_favorite)]
    pub async fn list_favorite_js(
        &self,
        search: fav::FeatureSearchQuery,
    ) -> Result<Favorites, JsValue> {
        let info = self.inner.list_favorite(search).await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = bounty)]
    pub async fn bounty_js(
        &self,
        query: &str,
        page: u32,
        btype: BountyType,
        status: BountyStatus,
        user: Option<u32>,
    ) -> Result<BountyPage, JsValue> {
        let info = self
            .inner
            .bounty(query, page, btype, status, user)
            .await
            .map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = home)]
    pub async fn home_js(&self) -> Result<HomeInfo, JsValue> {
        let info = self.inner.home().await.map_err(js_err)?;
        Ok(info)
    }

    #[wasm_bindgen(js_name = mpv_bypass)]
    pub async fn mpv_bypass_js(&self, gid: u64, token: String) -> Result<Vec<ImagePage>, JsValue> {
        let data = self
            .inner
            .mpv_info_bypass(gid, &token)
            .await
            .map_err(js_err)?;
        Ok(data)
    }

    #[wasm_bindgen(js_name = stats)]
    pub async fn stats_js(
        &self,
        gid: Option<u64>,
        token: Option<String>,
    ) -> Result<StatsPage, JsValue> {
        let data = match (gid, token) {
            (Some(gid), Some(token)) => Some((gid, token)),
            _ => None,
        };
        let data = self.inner.stats(data).await.map_err(js_err)?;
        Ok(data)
    }

    #[wasm_bindgen(js_name = upload_info)]
    pub async fn upload_info_js(
        &self,
        id: u64,
        uploaded: bool,
    ) -> Result<UploadGalleryInfo, JsValue> {
        let data = self.inner.upload_info(id, uploaded).await.map_err(js_err)?;
        Ok(data)
    }

    #[wasm_bindgen(js_name = bounty_info)]
    pub async fn bounty_info_js(&self, id: u64) -> Result<BountyInfo, JsValue> {
        let data = self.inner.bounty_info(id).await.map_err(js_err)?;
        Ok(data)
    }
}

// Small helper to convert Rust errors to JS exceptions.
fn js_err<E: core::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&e.to_string())
}
