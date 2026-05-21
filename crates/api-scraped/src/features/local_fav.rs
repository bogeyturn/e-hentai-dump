use serde::Serialize;

use crate::Session;

#[derive(Serialize)]
pub struct FavoriteRequest {
    pub gid: u64,
    pub fav: u8,
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct FavoriteDeleteRequest {
    pub gid: u64,
}

impl Session {
    pub async fn add_favorite_local(
        &self,
        gid: u64,
        _: &str,
        favcat: u8,
        favnote: &str,
    ) -> anyhow::Result<()> {
        assert!(favcat < 10);
        self.local_api(
            "set-favorite",
            &FavoriteRequest {
                gid,
                fav: favcat,
                note: if favnote.trim().is_empty() {
                    None
                } else {
                    Some(favnote.trim().to_string())
                },
            },
        )
        .await?;
        Ok(())
    }

    pub async fn remove_favorite_local(&self, gid: u64, _: &str) -> anyhow::Result<()> {
        self.local_api("remove-favorite", &FavoriteDeleteRequest { gid })
            .await?;
        Ok(())
    }
}
