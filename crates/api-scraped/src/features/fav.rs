use std::fmt::Display;

use anyhow::bail;
use scraper::Selector;
use serde::{Deserialize, Serialize};

use crate::Session;

use super::search::{InfoSelectors, SearchInfo, extract_info};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Pid {
    Single(u64),
    Range(u64, u64),
}

impl TryFrom<&str> for Pid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((l, r)) = value.split_once("-") {
            let f = l.parse::<u64>();
            let s = r.parse::<u64>();
            if let (Ok(f), Ok(s)) = (f, s) {
                return Ok(Pid::Range(f, s));
            }
        } else {
            let num = value.parse::<u64>();
            if let Ok(v) = num {
                return Ok(Pid::Single(v));
            }
        }
        bail!("invalid id format: {}", value)
    }
}

impl Display for Pid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pid::Single(i) => i.fmt(f),
            Pid::Range(fi, se) => write!(f, "{}-{}", *fi, *se),
        }
    }
}

fn build_url(url: &str, items: &Vec<String>) -> String {
    format!("{url}?{}", items.join("&"))
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
pub struct FeatureSearchQuery {
    pub query: Option<String>,
    pub pid: Option<Pid>,
    pub forward: bool,
    pub cat: Option<u8>,
    pub order_by_published: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Favorites {
    pub counts: Vec<u64>,
    pub items: Vec<SearchInfo>,
    pub next: Option<Pid>,
    pub last: Option<Pid>,
    pub prev: Option<Pid>,
}

impl Session {
    pub async fn list_favorite(&self, search: FeatureSearchQuery) -> anyhow::Result<Favorites> {
        let base = "https://exhentai.org/favorites.php";
        let mut tags = vec!["inline_set=dm_e".to_owned()];
        if let Some(cat) = search.cat {
            assert!(cat > 0 && cat < 10);
            tags.push(format!("favcat={cat}"));
        }
        if let Some(ref query) = search.query {
            tags.push(format!("f_search={query}"));
        }
        if let Some(pid) = search.pid {
            if search.forward {
                tags.push(format!("next={pid}"))
            } else {
                tags.push(format!("prev={pid}"))
            }
        }

        let mut html = self.get_html(build_url(base, &tags)).await?;
        let counts = crate::selector(".nosel > .fp > div:nth-child(1)");
        let counts = html
            .select(&counts)
            .map(|v| v.text().collect::<String>().trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        if html
            .select(&crate::selector(".ido>p"))
            .next()
            .map(|v| v.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            == "No hits found"
        {
            return Ok(Favorites {
                counts,
                items: vec![],
                next: None,
                last: None,
                prev: None,
            });
        }
        let mode = Selector::parse(".searchnav > div:nth-child(1) > select > option").unwrap();

        let mode = html
            .select(&mode)
            .filter(|v| v.attr("selected").is_some())
            .next()
            .unwrap()
            .attr("value")
            .unwrap_or_default();
        let mode_published = match mode {
            "p" => true,
            _ => false,
        };

        if mode_published != search.order_by_published {
            tags.remove(0);
            tags.push(format!(
                "inline_set=fs_{}",
                match search.order_by_published {
                    true => "p",
                    false => "f",
                }
            ));
            html = self.get_html(build_url(base, &tags)).await?;
        }
        let prev = Selector::parse("a#uprev").unwrap();
        let next = Selector::parse("a#unext").unwrap();
        let parse = |id| {
            html.select(&id)
                .next()
                .map(|v| v.attr("href"))
                .flatten()
                .map(|v| v.rsplit_once("="))
                .flatten()
                .map(|v| Pid::try_from(v.1))
                .transpose()
        };
        let next = parse(next)?;
        let prev = parse(prev)?;
        let last = match next {
            Some(_) => Some(match mode_published {
                true => Pid::Single(1),
                false => Pid::Range(1, 0),
            }),
            None => None,
        };

        let v = extract_info(&html, InfoSelectors::default());
        Ok(Favorites {
            counts,
            items: v,
            next,
            last,
            prev,
        })
    }
    pub async fn add_favorite(
        &self,
        gid: u64,
        token: &str,
        favcat: u8,
        favnote: &str,
    ) -> anyhow::Result<()> {
        assert!(favcat < 10);
        self.form(
            format!("https://exhentai.org/gallerypopups.php?gid={gid}&t={token}&act=addfav"),
            &[
                ("favcat", favcat.to_string().as_str()),
                ("favnote", favnote),
                ("apply", "Add to Favorites"),
                ("update", "1"),
            ],
        )
        .await?;
        Ok(())
    }

    pub async fn remove_favorite(&self, gid: u64, token: &str) -> anyhow::Result<()> {
        self.form(
            format!("https://exhentai.org/gallerypopups.php?gid={gid}&t={token}&act=addfav"),
            &[
                ("favcat", "favdel"),
                ("favnote", ""),
                ("apply", "Apply Changes"),
                ("update", "1"),
            ],
        )
        .await?;
        Ok(())
    }
}
