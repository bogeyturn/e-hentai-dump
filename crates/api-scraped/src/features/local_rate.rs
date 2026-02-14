use serde::Serialize;

use crate::Session;

#[derive(Serialize)]
pub struct RatingRequest {
    pub gid: u64,
    pub rating: u8,
}
impl Session {
    pub async fn rate_local(
        &self,
        gid: u64,
        _: &str,
        _: u64,
        _: &str,
        rating: u8,
    ) -> anyhow::Result<()> {
        self.local_api("set-rating", &RatingRequest { gid, rating })
            .await?;
        Ok(())
    }
}
