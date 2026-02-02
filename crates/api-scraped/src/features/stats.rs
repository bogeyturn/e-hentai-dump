use scraper::ElementRef;
use serde::Serialize;

use crate::{Session, id, selector};

fn parse_abbreviated_number(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (num_part, multiplier) = match s.chars().last().unwrap() {
        'K' | 'k' => (&s[..s.len() - 1], 1_000.0),
        'M' | 'm' => (&s[..s.len() - 1], 1_000_000.0),
        'B' | 'b' => (&s[..s.len() - 1], 1_000_000_000.0),
        _ => (s, 1.0),
    };

    num_part
        .parse::<f64>()
        .ok()
        .map(|n| (n * multiplier).round() as u64)
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct VisitStat {
    pub title: String,
    pub visits: u64,
    pub hits: u64,
    pub hits_norm: f64,
    pub visits_norm: f64,
}
fn parse_stats(html: &ElementRef<'_>) -> Vec<VisitStat> {
    let title = selector("tr:nth-child(3) > .stdk");
    let title = html
        .select(&title)
        .map(|v| v.text().collect::<String>().trim().to_owned())
        .collect::<Vec<_>>();
    let visits = selector("tr:nth-child(4) >.stdv");
    let visits = html
        .select(&visits)
        .map(|v| parse_abbreviated_number(&v.text().collect::<String>()).unwrap())
        .collect::<Vec<_>>();
    let hits = selector("tr:nth-child(5) > .stdv");
    let hits = html
        .select(&hits)
        .map(|v| parse_abbreviated_number(&v.text().collect::<String>()).unwrap())
        .collect::<Vec<_>>();
    let hits_max = hits.iter().max().copied().unwrap_or(1);
    let visits_max = visits.iter().max().copied().unwrap_or(1);

    assert_eq!(title.len(), visits.len());
    assert_eq!(visits.len(), hits.len());
    title
        .into_iter()
        .zip(visits.iter().zip(hits))
        .map(|(title, (visits, hits))| VisitStat {
            title,
            visits: *visits,
            hits,
            hits_norm: (hits as f64 / hits_max as f64),
            visits_norm: (*visits as f64 / visits_max as f64),
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct StatRanking {
    pub all: String,
    pub year: String,
    pub month: String,
    pub yesterday: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct StatsPage {
    pub title: String,
    pub ranking: Option<StatRanking>,
    pub total: u64,
    pub daily: Vec<VisitStat>,
    pub monthly: Vec<VisitStat>,
    pub yearly: Vec<VisitStat>,
}

impl Session {
    pub async fn stats(&self, gallery: Option<(u64, String)>) -> anyhow::Result<StatsPage> {
        let gallery = gallery
            .map(|v| format!("?gid={}&t={}", v.0, v.1))
            .unwrap_or_default();
        let html = self
            .get_html(format!("https://e-hentai.org/stats.php{}", gallery))
            .await?; //
        let tables = crate::selector("#graphs table");
        let title = html
            .select(&id("gn"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_owned();
        let mut tables = html.select(&tables);
        let stats = html
            .select(&selector(".stuffbox > table td"))
            .map(|v| v.text().collect::<String>().trim().to_owned())
            .collect::<Vec<_>>();
        let stats = match stats.len() {
            0 => None,
            8 => Some(StatRanking {
                all: stats[1].clone(),
                year: stats[3].clone(),
                month: stats[5].clone(),
                yesterday: stats[7].clone(),
            }),
            _ => unimplemented!(),
        };

        let daily = tables.next().unwrap();
        let monthly = tables.next().unwrap();
        let yearly = tables.next().unwrap();
        let count = html
            .select(&selector(".stuffbox > p > strong"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .replace(",", "")
            .parse::<u64>()
            .unwrap();
        Ok(StatsPage {
            title,
            ranking: stats,
            total: count,
            daily: parse_stats(&daily),
            monthly: parse_stats(&monthly),
            yearly: parse_stats(&yearly),
        })
    }
}
