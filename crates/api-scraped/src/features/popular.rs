use crate::Session;

use super::search::{SearchInfo, extract_info};

impl Session {
    pub async fn popular(&self) -> anyhow::Result<Vec<SearchInfo>> {
        let html = self
            .get_html("https://exhentai.org/popular?inline_set=dm_e")
            .await?;
        Ok(extract_info(&html, Default::default()))
    }
}
