use scraper::Selector;
use serde::Serialize;

use crate::{Session, unit::parse_to_bytes};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Stat {
    pub name: String,
    pub load_bytes: u64,
    pub hits_sec: Option<f64>,
    pub hits_gb: Option<f64>,
    pub quality: Option<u64>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct File {
    pub file: String,
    pub link: String,
    pub size: u64,
    pub hash: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct HentaiAtHome {
    pub clients: Vec<Stat>,
    pub client_download: Vec<File>,
}
impl Session {
    pub async fn hentai_at_home(&self) -> anyhow::Result<HentaiAtHome> {
        let html = self
            .get_html("https://e-hentai.org/hentaiathome.php")
            .await?;
        let select = Selector::parse("#hathstats > tbody > tr td").unwrap();
        let clients = html
            .select(&select)
            .collect::<Vec<_>>()
            .chunks(7)
            .map(|v| Stat {
                name: v[0].text().collect::<String>().trim().to_owned(),
                load_bytes: parse_to_bytes(&v[3].text().collect::<String>()).unwrap(),
                hits_sec: v[4].text().collect::<String>().parse().ok(),
                hits_gb: v[5].text().collect::<String>().parse().ok(),
                quality: v[6].text().collect::<String>().parse().ok(),
            })
            .collect::<Vec<_>>();
        let select = Selector::parse("table").unwrap();
        let select2 = Selector::parse("tbody > tr td").unwrap();
        Ok(HentaiAtHome {
            clients,
            client_download: html
                .select(&select)
                .skip(1)
                .next()
                .unwrap()
                .select(&select2)
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|v| File {
                    link: v[0]
                        .child_elements()
                        .next()
                        .unwrap()
                        .attr("href")
                        .unwrap()
                        .to_owned(),
                    file: v[0].text().collect::<String>().trim().to_owned(),
                    size: v[1]
                        .text()
                        .collect::<String>()
                        .trim()
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap(),
                    hash: v[2].text().collect::<String>().trim().to_owned(),
                })
                .collect(),
        })
    }
}
