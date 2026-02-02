use std::time::Duration;

use chrono::{NaiveDateTime, TimeZone as _, Utc};
use scraper::Selector;
use serde::Serialize;
use tsify::Tsify;

use crate::{Session, selector};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct CreditLog {
    pub date: Duration,
    pub ammount: i64,
    pub information: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct KarmaLog {
    pub date: Duration,
    pub ammount: i64,
    pub from: String,
    pub from_id: u64,
    pub topic: String,
    pub comment: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct KarmaPage {
    pub karma: i32,
    pub logs: Vec<KarmaLog>,
}

impl Session {
    pub async fn karma_logs(&self) -> anyhow::Result<KarmaPage> {
        let html = self
            .get_html("https://e-hentai.org/logs.php?t=karma")
            .await?;
        let karma = selector("body > div > span");
        let karma = html
            .select(&karma)
            .next()
            .unwrap()
            .inner_html()
            .trim()
            .replace(['+', ','], "")
            .parse::<i32>()
            .unwrap();
        let selector = Selector::parse(".logbox>table>tbody>tr>td").unwrap();
        Ok(KarmaPage {
            karma,
            logs: html
                .select(&selector)
                .collect::<Vec<_>>()
                .chunks(5)
                .map(|v| KarmaLog {
                    date: {
                        let date_str = v[0].text().collect::<String>();
                        let naive =
                            NaiveDateTime::parse_from_str(date_str.trim(), "%Y-%m-%d %H:%M")
                                .expect("invalid datetime");

                        Duration::from_secs(Utc.from_utc_datetime(&naive).timestamp() as u64)
                    },
                    ammount: v[1]
                        .text()
                        .collect::<String>()
                        .replace(['+', ','], "")
                        .trim()
                        .parse()
                        .unwrap(),
                    from: v[2].text().collect::<String>().trim().to_owned(),
                    from_id: v[2]
                        .child_elements()
                        .next()
                        .unwrap()
                        .attr("href")
                        .unwrap()
                        .split_once("showuser=")
                        .unwrap()
                        .1
                        .parse()
                        .unwrap(),
                    topic: v[3].text().collect::<String>().trim().to_owned(),
                    comment: v[4].text().collect::<String>().trim().to_owned(),
                })
                .collect::<Vec<_>>(),
        })
    }

    pub async fn credit_logs(&self) -> anyhow::Result<Vec<CreditLog>> {
        let html = self
            .get_html("https://e-hentai.org/logs.php?t=credits")
            .await?;
        let selector = Selector::parse(".logbox>table>tbody>tr>td").unwrap();
        Ok(html
            .select(&selector)
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|v| CreditLog {
                date: {
                    let date_str = v[0].text().collect::<String>();
                    let naive = NaiveDateTime::parse_from_str(date_str.trim(), "%Y-%m-%d %H:%M")
                        .expect("invalid datetime");

                    Duration::from_secs(Utc.from_utc_datetime(&naive).timestamp() as u64)
                },
                ammount: v[1]
                    .text()
                    .collect::<String>()
                    .replace(['+', ','], "")
                    .trim()
                    .parse()
                    .unwrap(),
                information: v[2].text().collect::<String>().trim().to_owned(),
            })
            .collect::<Vec<_>>())
    }
}
