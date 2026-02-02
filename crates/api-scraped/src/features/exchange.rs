use std::time::Duration;

use chrono::{Local, NaiveTime, TimeZone as _, Utc};
use log::warn;
use scraper::{ElementRef, Selector};
use serde::Serialize;

use crate::{Session, id, selector};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct ExchangePage {
    pub actionkey: String,
    pub credits: u32,
    pub currency: u32,
    pub bid_count: u32,
    pub bid_price: u32,
    pub ask_count: u32,
    pub ask_price: u32,
    pub l8h: Stats,
    pub l24h: Stats,
    pub active_bid: Vec<Active>,
    pub active_ask: Vec<Active>,
    pub recent: Vec<Recent>,
}
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Active {
    pub vol: u32,
    pub price: u32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Recent {
    pub time: Duration,
    pub seller: String,
    pub buyer: String,
    pub vol: u32,
    pub price: u32,
}
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Stats {
    pub high: u32,
    pub low: u32,
    pub avg: u32,
    pub vol: u32,
}

impl Session {
    pub async fn exhange_bid(&self, hath: bool, count: u32, price: u32) -> anyhow::Result<()> {
        let info = self.exchange_info(hath).await?;

        self.form(
            &format!(
                "https://e-hentai.org/exchange.php?t={}",
                if hath { "hath" } else { "gp" }
            ),
            &[
                ("actionkey", info.actionkey),
                ("bid_count", count.to_string()),
                ("bid_price", price.to_string()),
            ],
        )
        .await?;

        Ok(())
    }

    pub async fn exhange_ask(&self, hath: bool, count: u32, price: u32) -> anyhow::Result<()> {
        let info = self.exchange_info(hath).await?;

        self.form(
            &format!(
                "https://e-hentai.org/exchange.php?t={}",
                if hath { "hath" } else { "gp" }
            ),
            &[
                ("actionkey", info.actionkey),
                ("ask_count", count.to_string()),
                ("ask_price", price.to_string()),
            ],
        )
        .await?;

        Ok(())
    }

    pub async fn exchange_info(&self, hath: bool) -> anyhow::Result<ExchangePage> {
        let base = format!(
            "https://e-hentai.org/exchange.php?t={}",
            if hath { "hath" } else { "gp" }
        );
        let actionkey = selector("#buyform > input");

        let selector = Selector::parse(".stuffbox > .outer").unwrap();
        let html = self.get_html(base).await?;
        let mut containers = html.select(&selector);
        let container = containers.next().unwrap();
        let last = Selector::parse("div > div > div > div").unwrap();
        let stats = container.select(&last);
        let bid_count = id("bid_count");
        let bid_price = id("bid_price");
        let ask_count = id("ask_count");
        let ask_price = id("ask_price");
        let get_input = |item| {
            html.select(&item)
                .next()
                .unwrap()
                .attr("value")
                .unwrap()
                .parse::<u32>()
                .unwrap()
        };
        let actionkey = html
            .select(&actionkey)
            .next()
            .unwrap()
            .attr("value")
            .unwrap()
            .to_owned();
        let bid_count = get_input(bid_count);
        let bid_price = get_input(bid_price);
        let ask_count = get_input(ask_count);
        let ask_price = get_input(ask_price);
        let mut stats = stats.map(|v| {
            v.children()
                .filter_map(|v| match v.value() {
                    scraper::Node::Text(text) => text
                        .to_string()
                        .split(" ")
                        .collect::<Vec<_>>()
                        .get(1)
                        .map(|v| v.replace(",", "").parse::<u32>().ok())
                        .flatten(),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        let last8 = stats.next().unwrap();
        let last24 = stats.next().unwrap();
        let container = containers.next().unwrap();

        let sel = Selector::parse("div > div:nth-child(3)").unwrap();
        let credits = container
            .select(&sel)
            .filter_map(|v| {
                v.text()
                    .next()
                    .unwrap()
                    .split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .replace(",", "")
                    .parse::<u32>()
                    .ok()
            })
            .collect::<Vec<_>>();
        let table = Selector::parse("table > tbody").unwrap();
        let container = containers.next().unwrap();
        let mut tables = container.select(&table);
        let a1 = tables.next().unwrap();
        let a2 = tables.next().unwrap();
        let r = tables.next().unwrap();
        let trtd = Selector::parse("tr > td").unwrap();
        let parse_int = |v: &ElementRef<'_>| {
            v.text()
                .next()
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .replace(",", "")
                .parse::<u32>()
                .expect(v.text().next().unwrap())
        };
        let extract = |a: ElementRef<'_>| {
            a.select(&trtd)
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|v| Active {
                    vol: parse_int(&v[0]),
                    price: parse_int(&v[2]),
                })
                .collect::<Vec<_>>()
        };
        let extract2 = |a: ElementRef<'_>| {
            a.select(&trtd)
                .collect::<Vec<_>>()
                .chunks(5)
                .map(|v| Recent {
                    seller: v[1].text().collect::<String>().trim().to_string(),
                    buyer: v[2].text().collect::<String>().trim().to_string(),
                    time: time_str_to_past_timestamp(v[0].text().collect::<String>().trim()),
                    vol: parse_int(&v[3]),
                    price: parse_int(&v[4]),
                })
                .collect::<Vec<_>>()
        };

        Ok(ExchangePage {
            actionkey,
            credits: credits[0],
            currency: credits[1],
            l8h: Stats {
                high: last8[0],
                low: last8[1],
                avg: last8[2],
                vol: last8[3],
            },
            l24h: Stats {
                high: last24[0],
                low: last24[1],
                avg: last24[2],
                vol: last24[3],
            },
            active_bid: extract(a1),
            active_ask: extract(a2),
            recent: extract2(r),
            bid_count,
            bid_price,
            ask_count,
            ask_price,
        })
    }
}

fn time_str_to_past_timestamp(time_str: &str) -> Duration {
    let now = Utc::now();

    let time = NaiveTime::parse_from_str(time_str, "%H:%M").expect("invalid time format");

    let today = now.date_naive();
    let naive = today.and_time(time);

    let mut dt = Local
        .from_local_datetime(&naive)
        .single()
        .expect("ambiguous local time");

    if dt > now {
        warn!("date in the future");
        dt -= chrono::Duration::days(1);
    }

    Duration::from_secs(dt.timestamp() as u64)
}
