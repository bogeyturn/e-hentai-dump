use scraper::Selector;

use crate::Session;

impl Session {
    pub async fn tos(&self) -> anyhow::Result<String> {
        let html = self.get_html("https://e-hentai.org/tos.php").await?;
        let sel = Selector::parse(".stuffbox").unwrap();
        let content = html.select(&sel).next().unwrap().inner_html();
        Ok(content)
    }
}
