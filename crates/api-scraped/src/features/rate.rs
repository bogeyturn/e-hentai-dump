use serde_json::json;

use crate::Session;

impl Session {
    pub async fn rate(
        &self,
        gid: u64,
        token: &str,
        apiuid: i64,
        apikey: &str,
        rating: u8,
    ) -> anyhow::Result<()> {
        self.api(json!({
            "method": "rategallery",
            "apiuid": apiuid,
            "apikey": apikey,
            "gid": gid,
            "token": token,
            "rating": rating
        }))
        .await?;

        Ok(())
    }
}
