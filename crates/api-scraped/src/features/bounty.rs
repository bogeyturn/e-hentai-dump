use std::{collections::HashMap, ops::Add, time::Duration};

use scraper::{ElementRef, Selector};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    Session, id, selector,
    unit::{self, parse_date},
};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct BountyPage {
    pub pages: u32,
    pub title: String,
    pub items: Vec<Bounty>,
    pub msg: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct BountyInfo {
    pub title: String,
    pub img: Option<String>,
    pub description: String,
    pub reward: String,
    pub reward_detail: Vec<RewardDetail>,
    pub accepted: Vec<AccceptedDetail>,
    pub credits_owned: u64,
    pub hath_owned: u64,
    pub accepted_delivery: String,
    pub required_rank: String,
    pub status: String,
    pub r#type: String,
    pub posted: Duration,
    pub updated: Option<Duration>,
    pub posted_by: String,
    pub posted_by_id: u64,
    pub can_accept: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct RewardDetail {
    pub date: Option<Duration>,
    pub amount: String,
    pub added_by: String,
    pub added_by_id: u64,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct AccceptedDetail {
    pub date: Duration,
    pub status: String,
    pub person: String,
    pub person_id: u64,
    pub rating: String,
    pub comment: Option<String>,
}

impl Session {
    pub async fn bounty_info(&self, id: u64) -> anyhow::Result<BountyInfo> {
        let html = self
            .get_html(format!("https://e-hentai.org/bounty.php?bid={id}"))
            .await?;
        let labels = html
            .select(&selector("#d .l"))
            .map(|v| v.inner_html())
            .collect::<Vec<_>>();
        let info = html.select(&selector("#d .r")).collect::<Vec<_>>();
        let info = labels.into_iter().zip(info).collect::<HashMap<_, _>>();
        let description = html.select(&selector("#x")).next().unwrap().inner_html();
        let currency = html
            .select(&selector("#g > p:nth-child(2) > strong"))
            .map(|v| v.inner_html())
            .collect::<Vec<_>>();
        let wins = html.select(&selector("#g > table td")).collect::<Vec<_>>();
        let wins = wins.chunks(4);
        let wins = wins.map(|v| {
            let date = v[0].text().collect::<String>();
            RewardDetail {
                date: if date.trim() == "Original Bounty" {
                    None
                } else {
                    Some(parse_date(date.trim()).unwrap())
                },
                amount: v[1].text().collect::<String>().trim().to_owned(),
                added_by: v[2].text().collect::<String>().trim().to_owned(),
                added_by_id: v[2]
                    .child_elements()
                    .next()
                    .unwrap()
                    .attr("href")
                    .unwrap()
                    .rsplit_once("=")
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
            }
        });
        let mut accepted = html.select(&selector("#h > table td")).collect::<Vec<_>>();

        if accepted.len() == 1 {
            accepted = vec![]
        }
        let accepted = accepted.chunks(5).map(|v| {
            let date = v[0].text().collect::<String>();
            let comment = v[4].inner_html();
            AccceptedDetail {
                comment: if comment == "n/t" {
                    None
                } else {
                    Some(comment)
                },
                date: parse_date(date.trim()).unwrap(),
                status: v[1].text().collect::<String>().trim().to_owned(),
                person: v[2].text().collect::<String>().trim().to_owned(),
                person_id: v[2]
                    .child_elements()
                    .next()
                    .unwrap()
                    .attr("href")
                    .unwrap()
                    .rsplit_once("=")
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
                rating: v[3].text().collect::<String>().trim().to_owned(),
            }
        });
        let img = html
            .select(&selector("#b2 img"))
            .next()
            .unwrap()
            .attr("src")
            .unwrap()
            .to_owned();

        let title = html.select(&selector("#b>h1")).next().unwrap().inner_html();
        let dates = info
            .get("Posted Date:")
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_owned();
        let (posted, updated) = dates
            .split_once("(")
            .map(|v| {
                let b = v.1.replace("Updated", "").replace(")", "");
                (
                    v.0,
                    Some(
                        b.trim()
                            .strip_prefix(":")
                            .unwrap_or(b.trim())
                            .trim()
                            .to_owned(),
                    ),
                )
            })
            .unwrap_or((dates.as_str(), None));
        let cantaccept = html
            .select(&selector("#n input"))
            .next()
            .unwrap()
            .attr("disabled")
            .is_some();
        Ok(BountyInfo {
            title,
            img: if img.contains("/g/wanted-blank.png") {
                None
            } else {
                Some(img)
            },
            description,
            credits_owned: currency[0]
                .replace("Credits", "")
                .replace(",", "")
                .trim()
                .parse()
                .unwrap(),
            hath_owned: currency[1]
                .replace("Hath", "")
                .replace(",", "")
                .trim()
                .parse()
                .unwrap(),
            reward: info.get("Current Reward:").unwrap().inner_html(),
            accepted_delivery: info.get("Accepted Delivery:").unwrap().inner_html(),
            required_rank: info
                .get("Min Hunter Rank:")
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            status: info
                .get("Bounty Status:")
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            r#type: info
                .get("Bounty Type:")
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            posted_by: info
                .get("Bounty Posted By:")
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            posted_by_id: info
                .get("Bounty Posted By:")
                .unwrap()
                .child_elements()
                .next()
                .unwrap()
                .attr("href")
                .unwrap()
                .rsplit_once("=")
                .unwrap()
                .1
                .parse()
                .unwrap(),
            posted: unit::parse_date(posted).unwrap(),
            updated: updated.map(|v| unit::parse_date(&v).expect(&v)),
            reward_detail: wins.collect(),
            accepted: accepted.collect(),
            can_accept: !cantaccept,
        })
    }

    pub async fn bounty(
        &self,
        query: &str,
        page: u32,
        btype: BountyType,
        status: BountyStatus,
        user: Option<u32>,
    ) -> anyhow::Result<BountyPage> {
        assert!(page > 0);
        let url = format!(
            "https://e-hentai.org/bounty.php?search={}&t={}&s={}&p={}{}",
            query,
            btype.to_value(),
            status.to_value(),
            page - 1,
            user.map(|v| format!("&u={}", v)).unwrap_or_default()
        );
        let html = self.get_html(url).await?;
        let title = html
            .select(&crate::selector("#s > h1"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_owned();
        let pages = crate::selector(".ptt a");
        let pages = html
            .select(&pages)
            .map(|v| v.attr("href").unwrap_or_default())
            .filter(|v| v.contains("p="))
            .map(|v| v.split_once("p=").unwrap().1)
            .map(|v| {
                v.split_once("&")
                    .map(|v| v.0)
                    .unwrap_or(v)
                    .parse::<u32>()
                    .unwrap()
            })
            .max()
            .unwrap_or_default()
            .add(1);
        let table = Selector::parse(".itg").unwrap();
        let node = html.select(&table).next().unwrap();
        let table_ = parse_table(node);
        let msg = html
            .select(&id("r"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_owned();
        if table_.len() == 1 && table_[0].len() == 1 {
            return Ok(BountyPage {
                pages: 1,
                title,
                msg,
                items: vec![],
            });
        }
        let items = table_
            .into_iter()
            .map(|v| hashmap_to_struct(v))
            .collect::<Result<Vec<BountyInternal>, serde_json::Error>>()?;
        let a = html
            .select(&table)
            .next()
            .unwrap()
            .select(&selector("a"))
            .filter_map(|v| v.attr("href"))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|v| {
                (
                    v[0].rsplit_once("bid=").unwrap().1.parse().unwrap(),
                    v[1].rsplit_once("u=").unwrap().1.parse().unwrap(),
                )
            })
            .collect::<Vec<(u32, u32)>>();

        assert_eq!(a.len(), items.len());

        Ok(BountyPage {
            title,
            pages,
            msg: html
                .select(&id("r"))
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            items: items
                .into_iter()
                .zip(a)
                .map(|v| Bounty {
                    last_updated: unit::parse_date(&v.0.last_updated).unwrap(),
                    headline: v.0.headline,
                    bounty_type: v.0.bounty_type,
                    status: v.0.status,
                    total: v.0.total,
                    posted_by: v.0.posted_by,
                    uid: v.1.1,
                    id: v.1.0,
                })
                .collect(),
        })
    }
}

pub fn hashmap_to_struct<T>(map: HashMap<String, String>) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let json = serde_json::to_string(&map)?;
    let result = serde_json::from_str(&json)?;
    Ok(result)
}
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Bounty {
    last_updated: Duration,
    headline: String,
    bounty_type: String,
    status: String,
    total: String,
    posted_by: String,
    uid: u32,
    id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct BountyInternal {
    #[serde(rename(deserialize = "Last Updated"))]
    last_updated: String,
    #[serde(rename(deserialize = "Bounty Headline"))]
    headline: String,
    #[serde(rename(deserialize = "Bounty Type"))]
    bounty_type: String,
    #[serde(rename(deserialize = "Bounty Status"))]
    status: String,
    #[serde(rename(deserialize = "Total Bounty"))]
    total: String,
    #[serde(rename(deserialize = "Posted By"))]
    posted_by: String,
}

fn parse_table(document: ElementRef<'_>) -> Vec<HashMap<String, String>> {
    let row_sel = Selector::parse("tr").unwrap();
    let th_sel = Selector::parse("th").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    let mut rows = document.select(&row_sel);

    let headers: Vec<String> = rows
        .next()
        .unwrap()
        .select(&th_sel)
        .map(|h| h.text().collect::<String>().trim().to_string())
        .collect();

    let mut result = Vec::new();

    for row in rows {
        let cells: Vec<String> = row
            .select(&td_sel)
            .map(|c| c.text().collect::<String>().trim().to_string())
            .collect();

        let mut map = HashMap::new();
        for (header, cell) in headers.iter().zip(cells.iter()) {
            map.insert(header.clone(), cell.clone());
        }

        if !map.is_empty() {
            result.push(map);
        }
    }

    result
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
pub enum BountyType {
    All,
    Standard,
    Translation,
    Editing,
}

impl BountyType {
    pub fn to_value(&self) -> &str {
        match self {
            BountyType::All => "",
            BountyType::Standard => "s",
            BountyType::Translation => "t",
            BountyType::Editing => "e",
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
pub enum BountyStatus {
    Open,
    Reserved,
    Claimed,
    Completed,
    PostedMe,
    BoostedMe,
    AcceptedMe,
    ReservedMe,
    ClaimedMe,
    CompletedMe,
}

impl BountyStatus {
    pub fn to_value(&self) -> &str {
        match self {
            BountyStatus::Open => "",
            BountyStatus::Reserved => "r",
            BountyStatus::Claimed => "c",
            BountyStatus::Completed => "d",
            BountyStatus::PostedMe => "m1",
            BountyStatus::BoostedMe => "m2",
            BountyStatus::AcceptedMe => "m3",
            BountyStatus::ReservedMe => "m6",
            BountyStatus::ClaimedMe => "m4",
            BountyStatus::CompletedMe => "m5",
        }
    }
}
