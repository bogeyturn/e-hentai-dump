use serde::Deserialize;
use serde_json::json;

use crate::Session;

#[derive(Deserialize)]
struct Error {
    error: Option<String>,
}
impl Session {
    pub async fn tag_vote(
        &self,
        gid: u64,
        token: &str,
        tags: &str,
        upvote: bool,
        apiuid: u64,
        apikey: &str,
    ) -> anyhow::Result<Option<String>> {
        let text: Error = self.api(json!({"method":"taggallery","apiuid":apiuid,"apikey":apikey,"gid":gid,"token":token,"tags":tags,"vote":match upvote { true => 1, false => -1 }})).await?
            .json().await?;
        Ok(text.error)
    }
}
