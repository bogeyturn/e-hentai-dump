use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Session, id};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Resp {
    pub prev: Option<String>,
    pub next: Option<String>,
    pub showkey: String,
    pub url: String,
    pub original: Option<String>,
    pub size: String,
    pub name: String,
    pub hash: String,
    pub w: u64,
    pub h: u64,
    pub count: u64,
    pub gallery_key: String,
}

impl Session {
    pub async fn next_img(
        &self,
        gid: u64,
        imgkey: &str,
        page: u32,
        showkey: Option<String>,
    ) -> anyhow::Result<Resp> {
        let (showkey, data) = match showkey {
            Some(showkey) => {
                let data:Data = self.api(json!({"method":"showpage","gid":gid,"page":page,"imgkey":imgkey,"showkey":showkey})).await?.json().await?;
                (showkey, data)
            }
            None => {
                let html = self
                    .get_text(format!("https://exhentai.org/s/{imgkey}/{gid}-{page}"))
                    .await?;

                let showkey = html
                    .split_once("var showkey=\"")
                    .unwrap()
                    .1
                    .split_once("\";")
                    .unwrap()
                    .0
                    .to_string();
                let html = Html::parse_document(&html);
                (
                    showkey,
                    Data {
                        n: html.select(&id("i2")).next().unwrap().inner_html(),
                        i: html
                            .select(&crate::selector("#i2>div:nth-child(2)"))
                            .next()
                            .unwrap()
                            .html(),
                        i3: html.select(&id("i3")).next().unwrap().inner_html(),
                        i5: html.select(&id("i5")).next().unwrap().inner_html(),
                        i6: html.select(&id("i6")).next().unwrap().inner_html(),
                    },
                )
            }
        };
        let next = Html::parse_fragment(&data.i3);
        let last = Html::parse_fragment(&data.n);
        let i4 = Html::parse_fragment(&data.i);
        let i5 = Html::parse_fragment(&data.i5);
        let i6 = Html::parse_fragment(&data.i6);
        let div = Selector::parse("div").unwrap();

        let segments = i4.select(&div).next().unwrap().inner_html();
        let mut segments = segments.split(" :: ");
        let name = segments.next().unwrap().trim().to_owned();
        let (w, h) = segments.next().unwrap().trim().split_once(" x ").unwrap();
        let (width, height) = (w.parse::<u64>().unwrap(), h.parse::<u64>().unwrap());
        let size = segments.next().unwrap().trim().to_owned();

        let img_s = Selector::parse("img#img").unwrap();
        let a_s = Selector::parse("a").unwrap();
        let prev = Selector::parse("a#prev").unwrap();
        let original = i6
            .select(&a_s)
            .find(|v| v.inner_html().contains("Download original "))
            .map(|v| v.attr("href").unwrap().to_owned());
        let hash = i6
            .select(&a_s)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .split_once("f_shash=")
            .unwrap()
            .1
            .to_owned();
        let gallery_key = i5
            .select(&a_s)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .split("/")
            .filter(|v| !v.trim().is_empty())
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .to_string();

        let img = next.select(&img_s).next().unwrap().attr("src").unwrap();
        let pages = last
            .select(&a_s)
            .filter_map(|v| v.attr("href"))
            .filter_map(|v| v.split_once("-"))
            .filter_map(|v| v.1.parse::<u64>().ok())
            .max()
            .unwrap_or(1);
        let next = next
            .select(&a_s)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .split_once("/s/")
            .unwrap()
            .1
            .split_once("/")
            .unwrap()
            .0;
        let prev = last
            .select(&prev)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .split_once("/s/")
            .unwrap()
            .1
            .split_once("/")
            .unwrap()
            .0;

        Ok(Resp {
            gallery_key: gallery_key,
            hash,
            original,
            name: name.to_owned(),
            w: width,
            h: height,
            size,
            count: pages,
            prev: match prev == imgkey {
                true => None,
                false => Some(prev.to_owned()),
            },
            next: match next == imgkey {
                true => None,
                false => Some(next.to_owned()),
            },
            showkey,
            url: img.to_owned(),
        })
    }
}

#[derive(Deserialize)]
struct Data {
    n: String,
    i: String,
    i3: String,
    i5: String,
    i6: String,
}
