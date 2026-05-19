use scraper::Selector;
use serde::Serialize;

use crate::{Session, selector};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Perk {
    pub title: String,
    pub description: String,
    pub free: Option<u64>,
    pub price: u64,
    pub disabled: bool,
    pub purchased: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct PerkPage {
    pub hath: u64,
    pub perks: Vec<Vec<Perk>>,
}

fn group_by_root<T>(input: Vec<(T, bool)>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();

    for (value, is_root) in input {
        if is_root {
            result.push(vec![value]);
        } else if let Some(last) = result.last_mut() {
            last.push(value);
        } else {
            unreachable!("shouldnt be reachable")
        }
    }

    result
}

impl Session {
    pub async fn perks(&self) -> anyhow::Result<PerkPage> {
        let html = self.get_html("https://e-hentai.org/hathperks.php").await?;
        let hath = Selector::parse(".stuffbox> div:nth-child(2)>p>span").unwrap();
        let hath = html
            .select(&hath)
            .collect::<Vec<_>>()
            .pop()
            .unwrap()
            .inner_html()
            .parse::<u64>()?;
        let table = Selector::parse(".stuffbox table > tbody > tr").unwrap();
        let perks = html
            .select(&table)
            .skip(1)
            .map(|v| {
                let mut children = v.child_elements();
                let title = children.next().unwrap();
                let root = title
                    .attr("style")
                    .unwrap_or_default()
                    .contains("padding-left:5px;");
                let input = v.select(&selector("input")).next().is_some();
                let title = title.text().next().unwrap().to_owned();
                let desc = children.next().unwrap().text().collect::<String>();
                let (desc, free) = desc
                    .as_str()
                    .split_once("Free with a $")
                    .map(|(a, b)| {
                        (
                            a.trim().to_owned(),
                            Some(b.split(" ").next().unwrap().parse().unwrap()),
                        )
                    })
                    .unwrap_or_else(|| (desc.clone(), None));
                (
                    Perk {
                        purchased: !input,
                        title,
                        description: desc,
                        free,
                        price: children
                            .next()
                            .unwrap()
                            .text()
                            .next()
                            .unwrap()
                            .split(" ")
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        disabled: v.attr("style") == Some("opacity:0.7"),
                    },
                    root,
                )
            })
            .collect::<Vec<_>>();
        Ok(PerkPage {
            hath,
            perks: group_by_root(perks),
        })
    }

    pub async fn buy_perk(&self, perk: u16) -> anyhow::Result<()> {
        self.form(
            "https://e-hentai.org/hathperks.php",
            &[
                ("purchase_id", perk.to_string().as_str()),
                ("purchase", "Purchase"),
            ],
        )
        .await?;
        Ok(())
    }
}

//hath_perks=q-a6fc64bca3
// curl '' \
//   -X POST \
//   -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:146.0) Gecko/20100101 Firefox/146.0' \
//   -H 'Content-Type: application/x-www-form-urlencoded' \
//   --data-raw 'purchase_id=71&purchase=Purchase'
