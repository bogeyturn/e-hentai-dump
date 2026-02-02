use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Session;

impl Session {
    pub async fn vote_comment(
        &self,
        gid: u64,
        token: &str,
        comment_id: u64,
        upvote: bool,
        apiuid: u64,
        apikey: &str,
    ) -> anyhow::Result<CommentVote> {
        let text: CommentVote = self.api(json!({"method":"votecomment","apiuid":apiuid,"apikey":apikey,"gid":gid,"token":token,"comment_id":comment_id,"comment_vote":match upvote {
            true => 1,
            false => -1,
        }}))
            .await?.json().await?;
        Ok(text)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct CommentVote {
    pub comment_id: u64,
    pub comment_score: i32,
}
