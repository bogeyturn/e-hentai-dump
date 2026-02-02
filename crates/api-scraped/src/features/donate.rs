use crate::Session;

impl Session {
    pub async fn donate_info(&self) -> anyhow::Result<()> {
        let html = self
            .get_html("https://e-hentai.org/bitcoin.php?coin=BCH")
            .await?;
        todo!()
    }
}
