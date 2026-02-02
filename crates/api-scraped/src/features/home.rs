use scraper::Selector;
use serde::Serialize;

use crate::{Session, selector, unit};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct HomeInfo {
    pub gp: GP,
    pub ehtracker: EHTracker,
    pub toplist: Vec<TopListPlace>,
    pub power: ModerationPower,
    pub user_id: u64,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct EHTracker {
    pub key: String,
    pub seedmins: u64,
    pub gallery_completes: u64,
    pub torrent_completes: u64,
    pub uploaded_bytes: u64,
    pub downloaded_bytes: u64,
    pub up_down_ratio: f64,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct GP {
    pub gallery_visits: u32,
    pub torrent_completions: u32,
    pub archive_downloads: u32,
    pub hentai_at_home: u32,
}
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct TopListPlace {
    pub position: u32,
    pub name: String,
    pub toplist_id: u32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct ModerationPower {
    pub current: u16,
    pub base: f32,
    pub awards: f32,
    pub tagging: f32,
    pub level: f32,
    pub donations: f32,
    pub forum_activity: f32,
    pub uploads: f32,
    pub account_age: f32,
    pub sum: f32,
}

impl Session {
    pub async fn home(&self) -> anyhow::Result<HomeInfo> {
        let html = self.get_html("https://e-hentai.org/home.php").await?;
        let box1 = selector(".homebox .c1");
        let box1 = html
            .select(&box1)
            .map(|v| v.text().collect::<String>().trim().to_owned())
            .collect::<Vec<_>>();
        let uid = selector(".homebox a");
        let uid = html
            .select(&uid)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .split_once("u=")
            .unwrap()
            .1
            .to_owned()
            .parse::<u64>()?;
        let torrent_key = selector(".homebox > p > span");
        let torrent_key = html
            .select(&torrent_key)
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_owned();
        let homeboxes = selector(".homebox");

        let mut homeboxes = html.select(&homeboxes).skip(2);
        let bold = Selector::parse(r#"[style*="font-weight:bold"]"#).unwrap();

        let gp = selector("table > tbody > tr > td:nth-child(1)");
        let gp = homeboxes
            .next()
            .unwrap()
            .select(&gp)
            .map(|v| {
                v.text()
                    .collect::<String>()
                    .trim()
                    .replace(",", "")
                    .parse::<u32>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let toplists = homeboxes
            .next()
            .unwrap()
            .select(&selector("td>table td"))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|v| TopListPlace {
                position: v[0].text().collect::<String>().trim()[1..].parse().unwrap(),
                name: v[1]
                    .select(&selector("a"))
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_owned(),
                toplist_id: v[1]
                    .select(&selector("a"))
                    .next()
                    .unwrap()
                    .attr("href")
                    .unwrap()
                    .to_owned()
                    .replace("https://e-hentai.org/toplist.php?tl=", "")
                    .parse()
                    .unwrap(),
            })
            .collect::<Vec<_>>();
        let power = homeboxes
            .next()
            .unwrap()
            .select(&bold)
            .map(|v| v.text().collect::<String>().trim().to_owned())
            .filter(|v| !v.is_empty())
            .map(|v| ceanup_numbers(&v))
            .collect::<Vec<_>>();
        Ok(HomeInfo {
            gp: GP {
                gallery_visits: gp[0],
                torrent_completions: gp[1],
                archive_downloads: gp[2],
                hentai_at_home: gp[3],
            },
            ehtracker: EHTracker {
                key: torrent_key,
                seedmins: box1[5].replace(",", "").parse().unwrap(),
                gallery_completes: box1[4].replace(",", "").parse().unwrap(),
                torrent_completes: box1[3].replace(",", "").parse().unwrap(),
                uploaded_bytes: unit::parse_to_bytes(&box1[0]).unwrap(),
                downloaded_bytes: unit::parse_to_bytes(&box1[1]).unwrap(),
                up_down_ratio: box1[2].replace(",", "").parse().unwrap(),
            },
            toplist: toplists,
            power: ModerationPower {
                current: power[0] as u16,
                base: power[1],
                awards: power[2],
                tagging: power[3],
                level: power[5],
                donations: power[6],
                forum_activity: power[7],
                uploads: power[8],
                account_age: power[9],
                sum: power[10],
            },
            user_id: uid,
        })
    }
}

fn ceanup_numbers(mut s: &str) -> f32 {
    s = s.trim();
    let prefixes = ['+', '='];
    for prefix in prefixes {
        if s.starts_with(prefix) {
            s = &s[1..];
            break;
        }
    }
    s.trim().replace(",", "").parse().unwrap()
}
