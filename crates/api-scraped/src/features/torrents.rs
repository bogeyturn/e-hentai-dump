use std::time::Duration;

use chrono::{NaiveDateTime, TimeZone as _, Utc};
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::{Session, unit};

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
pub enum TorrentStatus {
    All,
    Seeded,
    Unseeded,
}

impl TorrentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TorrentStatus::All => "",
            TorrentStatus::Seeded => "seeded",
            TorrentStatus::Unseeded => "unseeded",
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
pub enum Sort {
    Downloads,
    Peers,
    Seeds,
    Size,
    Date,
}

impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Sort::Date => "a",
            Sort::Size => "b",
            Sort::Downloads => "c",
            Sort::Peers => "d",
            Sort::Seeds => "s",
        }
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Torrent {
    added: Duration,
    name: String,
    gallery: u64,
    size: u64,
    seeds: u64,
    peers: u64,
    dls: u64,
    uploader: String,
    uploader_id: u64,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct TorrentPage {
    count: u64,
    name: Option<String>,
    user_id: u64,
    items: Vec<Torrent>,
}

impl Session {
    pub async fn torrents(
        &self,
        query: &str,
        page: u32,
        status: TorrentStatus,
        my_torrents: Option<u32>,
        order: Sort,
        asc: bool,
    ) -> anyhow::Result<TorrentPage> {
        assert!(page > 0);
        let t = my_torrents.map(|v| format!("&u={}", v)).unwrap_or_default();
        let url = format!(
            "https://e-hentai.org/torrents.php?search={query}&s={}{t}&o={}{}{}",
            status.as_str(),
            order.as_str(),
            if asc { "a" } else { "d" },
            if page > 1 {
                format!("&page={}", page - 1)
            } else {
                "".to_owned()
            }
        );
        let content = self.get_text(url).await?;
        let html = Html::parse_document(&content);
        let select = Selector::parse("table.itg > tbody > tr > td").unwrap();
        let ip = Selector::parse(".ip").unwrap();
        let re = Regex::new(r"u\.value=(\d+);").unwrap();
        let uid = re
            .captures(&content)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let name = Selector::parse("#torrentform a").unwrap();
        let name = match html.select(&name).rev().next().unwrap().inner_html().trim() {
            "Only My Torrents" => None,
            v => Some(v.to_owned()),
        };

        let count = html.select(&ip).next().unwrap().text().collect::<String>();
        let count = match count
            .trim()
            .starts_with("You cannot add torrents directly to this page")
        {
            true => 0,
            false => count
                .trim()
                .split_whitespace()
                .rev()
                .next()
                .unwrap()
                .replace(",", "")
                .parse()
                .unwrap(),
        };

        Ok(TorrentPage {
            user_id: uid,
            count,
            name,
            items: html
                .select(&select)
                .collect::<Vec<_>>()
                .chunks(8)
                .map(|v| Torrent {
                    added: {
                        let date_str = v[0].text().collect::<String>();
                        let naive =
                            NaiveDateTime::parse_from_str(date_str.trim(), "%Y-%m-%d %H:%M")
                                .expect("invalid datetime");

                        Duration::from_secs(Utc.from_utc_datetime(&naive).timestamp() as u64)
                    },
                    name: v[1].text().collect::<String>().trim().to_owned(),
                    gallery: v[2].text().collect::<String>().trim().parse().unwrap(),
                    size: unit::parse_to_bytes(v[3].text().collect::<String>().trim()).unwrap(),
                    seeds: v[4].text().collect::<String>().trim().parse().unwrap(),
                    peers: v[5].text().collect::<String>().trim().parse().unwrap(),
                    dls: v[6].text().collect::<String>().trim().parse().unwrap(),
                    uploader: v[7].text().collect::<String>().trim().to_owned(),
                    uploader_id: split_opt(
                        v[7].child_elements()
                            .next()
                            .unwrap()
                            .child_elements()
                            .next()
                            .unwrap()
                            .attr("href")
                            .unwrap()
                            .split_once("u=")
                            .unwrap()
                            .1,
                        '&',
                    )
                    .parse::<u64>()
                    .unwrap(),
                })
                .collect(),
        })
    }
}

fn split_opt(s: &str, c: char) -> String {
    s.split_once(c).map(|v| v.0).unwrap_or(s).to_owned()
}
