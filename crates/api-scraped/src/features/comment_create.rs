use crate::Session;

impl Session {
    pub async fn comment_new(&self, gid: u64, token: &str, comment: &str) -> anyhow::Result<()> {
        self.form(
            format!("https://exhentai.org/g/{gid}/{token}/"),
            &("commenttext_new", comment),
        )
        .await?;

        Ok(())
    }

    pub async fn comment_update(
        &self,
        gid: u64,
        token: &str,
        comment_id: u64,
        comment: &str,
    ) -> anyhow::Result<()> {
        self.form(
            format!("https://exhentai.org/g/{gid}/{token}/"),
            &(("edit_comment", comment_id), ("commenttext_edit", comment)),
        )
        .await?;

        Ok(())
    }
}
