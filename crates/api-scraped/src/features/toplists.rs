use std::collections::HashMap;

use scraper::Selector;
use serde::{Deserialize, Serialize};

use crate::Session;

use super::bounty::hashmap_to_struct;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Toplist {
    id: String,
    name: String,
    items: Vec<TopListItem>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Toplists {
    name: String,
    toplists: Vec<Toplist>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct ToplistItem {
    #[serde(rename = "Rank")]
    rank: String,
    #[serde(rename = "Score")]
    score: String,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct TopListItem {
    idx: u32,
    name: String,
    link: ToplistLink,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub enum ToplistLink {
    User(String),
    Gallery(u64, String),
}

impl ToplistLink {
    pub fn new(url: &str) -> Self {
        if let Some((_, url)) = url.split_once("/g/") {
            let (id, key) = url.split_once("/").unwrap();
            ToplistLink::Gallery(id.parse().unwrap(), key.replace("/", ""))
        } else if let Some((_, name)) = url.split_once("/uploader/") {
            ToplistLink::User(name.replace("/", ""))
        } else {
            panic!("Invalid url")
        }
    }
}

impl Session {
    pub async fn toplist(&self, id: u32, page: u32) -> anyhow::Result<Vec<ToplistItem>> {
        assert!(page > 0);
        let url = format!("https://e-hentai.org/toplist.php?tl={id}&p={}", page - 1);
        let doc = self.get_html(url).await?;
        let row_selector = Selector::parse(".itg tr").unwrap();
        let header_selector = Selector::parse("th").unwrap();
        let col_selector = Selector::parse("td").unwrap();

        let mut rows = doc.select(&row_selector);

        let headers: Vec<String> = rows
            .next()
            .unwrap()
            .select(&header_selector)
            .map(|h| h.text().collect::<String>().trim().to_string())
            .collect();

        let mut table = Vec::new();

        for row in rows {
            let cells: Vec<String> = row
                .select(&col_selector)
                .map(|c| c.text().collect::<String>().trim().to_string())
                .collect();

            // handle left + right item
            for chunk in cells.chunks(headers.len() / 2) {
                let mut map = HashMap::new();
                for (header, value) in headers.iter().take(chunk.len()).zip(chunk) {
                    map.insert(header.clone(), value.clone());
                }
                if !map.is_empty() {
                    table.push(hashmap_to_struct(map)?);
                }
            }
        }

        Ok(table)
    }

    pub async fn toplists(&self) -> anyhow::Result<Vec<Toplists>> {
        let url = "https://e-hentai.org/toplist.php";
        let doc = self.get_html(url).await?;
        let items = Selector::parse(".tdo").unwrap();
        let title = Selector::parse("p > a").unwrap();
        let numbers = Selector::parse(".pso").unwrap();
        let texts = Selector::parse(".tun > a").unwrap();
        let titles = Selector::parse("div > h2").unwrap();
        let titles = doc.select(&titles).map(|v| v.text().collect::<String>());
        let mut tl = doc
            .select(&items)
            .map(|v| {
                let t = v.select(&title).next().unwrap();
                let id = t.attr("href").unwrap().split_once("tl=").unwrap().1;
                let name = t.text().collect::<String>();
                let items = v
                    .select(&numbers)
                    .map(|v| {
                        v.text()
                            .collect::<String>()
                            .replace("#", "")
                            .parse::<u32>()
                            .unwrap()
                    })
                    .zip(v.select(&texts).map(|v| v))
                    .map(|(idx, node)| TopListItem {
                        idx,
                        link: ToplistLink::new(node.attr("href").unwrap()),
                        name: node.text().collect::<String>(),
                    })
                    .collect::<Vec<_>>();
                Toplist {
                    id: id.to_owned(),
                    name,
                    items,
                }
            })
            .rev()
            .collect::<Vec<_>>();
        let mut out = vec![];
        for title in titles {
            let items = vec![
                tl.pop().unwrap(),
                tl.pop().unwrap(),
                tl.pop().unwrap(),
                tl.pop().unwrap(),
            ];
            out.push(Toplists {
                name: title,
                toplists: items,
            });
        }

        Ok(out)
    }
}
