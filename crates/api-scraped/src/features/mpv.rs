use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Session;
#[derive(Serialize, Deserialize)]
struct Root1 {
    d: String,
    i: String,
    lf: Option<String>,
    ll: String,
    lo: String,
    ls: Option<String>,
    o: String,
    s: String,
    xres: String,
    yres: String,
}

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Image {
    url: String,
    original: Option<String>,
    hash: Option<String>,
    w: u32,
    h: u32,
}
impl Session {
    pub async fn mpv_page(
        &self,
        gid: u64,
        idx: u32,
        imgkey: &str,
        mpvkey: &str,
    ) -> anyhow::Result<Image> {
        let d:Root1 = self.api(json!({"method":"imagedispatch","gid":gid,"page":idx,"imgkey":imgkey,"mpvkey":mpvkey})).await?.json().await?;
        let hash =
            d.ls.and_then(|v| v.split_once("f_shash=").map(|v| v.1.to_owned()));
        let lf = d.lf.map(|v| format!("https://exhentai.org/{v}"));
        Ok(Image {
            url: d.i,
            w: d.xres.parse().unwrap(),
            h: d.yres.parse().unwrap(),
            hash,
            original: lf,
        })
    }
    pub async fn mpv_info(
        &self,
        id: u64,
        secret: &str,
    ) -> anyhow::Result<(Vec<ImagePage>, String)> {
        let url = format!("https://exhentai.org/mpv/{id}/{secret}/");
        let html = self.get_text(url).await?;
        let mpvkey = html
            .split_once("var mpvkey = \"")
            .unwrap()
            .1
            .split_once("\"")
            .unwrap()
            .0;
        let html = Html::parse_document(&html);

        let selector = Selector::parse("script").unwrap();
        let size = Selector::parse("#pane_thumbs div").unwrap();
        let ratios = html
            .select(&size)
            .map(|v| {
                let mut split = v.attr("style").unwrap().split([';', ':']).skip(1);
                let item = split.next().unwrap();
                let item2 = split.skip(1).next().unwrap();
                (
                    item.replace("px", "").parse::<u32>().unwrap(),
                    item2.replace("px", "").parse::<u32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let html = html
            .select(&selector)
            .map(|v| v.inner_html().trim().to_owned())
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(html.len(), 2);
        let item = html
            .last()
            .unwrap()
            .split_once("var imagelist = ")
            .unwrap()
            .1
            .split_once(";\n")
            .unwrap()
            .0;
        let v: Vec<MpvItem> = serde_json::from_str(item)?;
        assert_eq!(v.len(), ratios.len());
        Ok((
            v.into_iter()
                .zip(ratios)
                .enumerate()
                .map(|(i, (v, (w, h)))| ImagePage {
                    id: i as u32 + 1,
                    key: v.k,
                    name: v.n,
                    width: w,
                    height: h,
                    ratio: (w, h),
                    url: v.t,
                })
                .collect::<Vec<_>>(),
            mpvkey.to_owned(),
        ))
    }

    pub async fn mpv_info_bypass(&self, id: u64, secret: &str) -> anyhow::Result<Vec<ImagePage>> {
        let mut page = 1;
        let mut res = vec![];
        loop {
            let info = self.info(id, secret, page).await?;
            res.extend(info.pages);
            if info.files as usize <= res.len() {
                break;
            }
            page += 1;
        }
        Ok(res)
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct ImagePage {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub ratio: (u32, u32),
    pub key: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
struct MpvItem {
    k: String,
    n: String,
    t: String,
}
