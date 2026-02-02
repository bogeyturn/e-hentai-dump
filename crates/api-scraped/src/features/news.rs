use scraper::Selector;
use serde::Serialize;

use crate::Session;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct News {
    pub link: String,
    pub title: String,
    pub content: String,
    pub link2: Option<String>,
    pub date: Option<String>,
}

impl Session {
    pub async fn news(&self) -> anyhow::Result<Vec<News>> {
        let html = self.get_html("https://e-hentai.org/news.php").await?;
        let imp = Selector::parse("#nt .nwo > h2 > a").unwrap();
        let imp_content = Selector::parse("#nt .nwo > .nwi").unwrap();
        let news = Selector::parse("#nt .newstable").unwrap();
        let general_title = Selector::parse(".newstitle a").unwrap();
        let general_date = Selector::parse(".newsdate").unwrap();
        let general_content = Selector::parse(".newstext").unwrap();
        let general_link = Selector::parse(".newslink").unwrap();
        let mut items = html
            .select(&imp)
            .zip(html.select(&imp_content))
            .map(|v| News {
                link: v.0.attr("href").unwrap().to_owned(),
                title: v.0.text().collect::<String>(),
                content: v.1.inner_html(),
                link2: None,
                date: None,
            })
            .collect::<Vec<_>>();
        items.extend(html.select(&news).map(|v| {
            let t = v.select(&general_title).next().unwrap();
            News {
                link: t.attr("href").unwrap().to_owned(),
                title: t.text().collect::<String>(),
                content: v.select(&general_content).next().unwrap().inner_html(),
                link2: Some(v.select(&general_link).next().unwrap().inner_html()),
                date: Some(
                    v.select(&general_date)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<String>(),
                ),
            }
        }));
        Ok(items)
    }
}
