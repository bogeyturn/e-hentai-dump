use std::{mem, time::Duration};

use chrono::{NaiveDateTime, TimeZone, Utc};
use scraper::{ElementRef, Html, Selector};
use serde::Serialize;

use crate::{Session, selector, unit::parse_date};

fn extract_table(html: &Html, sel: &Selector) -> Vec<(String, Vec<Gallery>)> {
    let span = Selector::parse("span").unwrap();
    let mut builder = vec![];
    let mut name = "".to_string();
    let mut data = vec![];

    for item in html.select(&sel) {
        if let Some(v) = item.attr("class") {
            if v == "l" {
                let mut d = vec![];
                mem::swap(&mut d, &mut builder);
                if !d.is_empty() {
                    data.push((name.clone(), d));
                }
                name = item.select(&span).next().unwrap().inner_html();
            } else if v.starts_with("gtc") {
                builder.push(item);
            }
        }
    }
    data.push((name, builder));

    data.into_iter()
        .map(|v| {
            (
                v.0,
                v.1.chunks(6)
                    .map(|v| {
                        let name = v[0].child_elements().next().unwrap();
                        Gallery {
                            name: name.inner_html(),
                            uid: name
                                .attr("href")
                                .unwrap()
                                .rsplit_once("=")
                                .unwrap()
                                .1
                                .parse()
                                .unwrap(),
                            date: {
                                let date_str = v[1].text().collect::<String>();
                                let naive = NaiveDateTime::parse_from_str(
                                    date_str.trim(),
                                    "%Y-%m-%d %H:%M",
                                )
                                .expect("invalid datetime");

                                Duration::from_secs(
                                    Utc.from_utc_datetime(&naive).timestamp() as u64
                                )
                            },
                            files: v[2].inner_html().parse().unwrap(),
                            category: v[3].text().collect::<String>().trim().to_owned(),
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct GalleryPage {
    published: Vec<(String, Vec<Gallery>)>,
    unpublished: Vec<(String, Vec<Gallery>)>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct UploadGalleryInfo {
    pub id: u64,
    pub uploaded: bool,
    pub title_en: String,
    pub title_jp: String,
    pub category: u32,
    pub categories: Vec<(String, u64)>,
    pub langs: Vec<(String, Vec<(u32, String)>)>,
    pub lang: u32,
    pub langtype: u8,
    pub folderid: u32,
    pub folders: Vec<(String, u64)>,
    pub comment: String,
    pub imgs: Vec<UploadImg>,
    pub added: Duration,
    pub posted: Option<Duration>,
    pub files: u32,
    pub file_size: u64,
    pub parent: Option<u64>,
    pub explunged: bool,
    pub visible: bool,
    pub token: Option<String>,
    pub not_ml: Option<bool>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct UploadImg {
    pub page: u32,
    pub preview: String,
    pub token: Option<String>,
    pub fileindex: Option<u32>,
}

fn new_img(data: (Option<String>, String, Option<String>)) -> UploadImg {
    let preview = data.1;
    if let Some(data) = data.0 {
        let page = data.rsplit_once("-").unwrap().1.parse().unwrap();
        let token = data
            .split("/")
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>();
        let token = token.iter().rev().skip(1).next().unwrap();
        return UploadImg {
            page,
            preview,
            token: Some(token.to_string()),
            fileindex: None,
        };
    }
    let items = data.2.unwrap();
    let items = items.split(",").collect::<Vec<_>>();
    UploadImg {
        page: items[1].parse().unwrap(),
        preview,
        token: None,
        fileindex: Some(items.last().unwrap().replace(")", "").parse().unwrap()),
    }
}

impl Session {
    pub async fn list_upload(&self) -> anyhow::Result<GalleryPage> {
        let url = format!("https://upload.e-hentai.org/manage");
        let doc = self.get_html(url).await?;
        let sel = Selector::parse("#gtableu td").unwrap();
        let unpublished = extract_table(&doc, &sel);
        let sel = Selector::parse("#gtablep td").unwrap();
        let published = extract_table(&doc, &sel);

        Ok(GalleryPage {
            published,
            unpublished,
        })
    }

    pub async fn upload_info(&self, id: u64, uploaded: bool) -> anyhow::Result<UploadGalleryInfo> {
        let html = self
            .get_html(format!(
                "https://upload.e-hentai.org/managegallery?{}={}",
                if uploaded { "gid" } else { "ulgid" },
                id
            ))
            .await?;
        let name = crate::id("gname_en");
        let gname_jp = crate::id("gname_jp");
        let category = selector("#category option");
        let lang = selector("#langtag optgroup");
        let mylang = selector("#langtag option");

        let langtype = selector(r#"input[name="langtype"]"#);
        let folderid = selector("#folderid option");
        let comment = selector("#ulcomment");
        let langctl = selector("#langctl");
        let info = selector("#d .v");
        let gallery = selector("#d .n a");

        let imgs = selector("#t img");

        let gallery_key = html
            .select(&gallery)
            .next()
            .map(|v| {
                if v.attr("href").unwrap().contains("managegallery") {
                    return None;
                }
                Some(
                    v.attr("href")
                        .unwrap()
                        .split("/")
                        .filter(|v| !v.is_empty())
                        .last()
                        .unwrap()
                        .to_owned(),
                )
            })
            .flatten();

        let langs = html
            .select(&lang)
            .map(|v| {
                (
                    v.attr("label").unwrap().to_owned(),
                    v.child_elements()
                        .map(|v| {
                            (
                                v.attr("value").unwrap().parse::<u32>().unwrap(),
                                v.text().collect::<String>().trim().to_owned(),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        let mylang = html
            .select(&mylang)
            .find(|v| v.attr("selected").is_some())
            .unwrap()
            .attr("value")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let title_en = html
            .select(&name)
            .next()
            .unwrap()
            .attr("value")
            .unwrap()
            .to_owned();
        let title_ja = html
            .select(&gname_jp)
            .next()
            .unwrap()
            .attr("value")
            .unwrap()
            .to_owned();

        let comment = html
            .select(&comment)
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_owned();
        let info = html
            .select(&info)
            .map(|v| v.text().collect::<String>().trim().to_owned())
            .collect::<Vec<_>>();
        let imgs = html
            .select(&imgs)
            .map(|v| {
                (
                    ElementRef::wrap(v.parent().unwrap())
                        .unwrap()
                        .attr("href")
                        .map(|v| v.to_owned()),
                    v.attr("src").unwrap().to_owned(),
                    v.attr("onclick").map(|v| v.to_owned()),
                )
            })
            .map(new_img)
            .collect::<Vec<_>>();
        let not_ml = html
            .select(&langctl)
            .next()
            .map(|v| v.attr("checked").is_some());
        let categories = html
            .select(&category)
            .map(|v| {
                (
                    v.text().collect::<String>().trim().to_owned(),
                    v.attr("value").unwrap().parse::<u64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let category = html
            .select(&category)
            .find(|v| v.attr("selected").is_some())
            .map(|v| v.attr("value").unwrap().parse::<u32>().unwrap());

        let folders = html
            .select(&folderid)
            .map(|v| {
                (
                    v.text().collect::<String>().trim().to_owned(),
                    v.attr("value").unwrap().parse::<u64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let folderid = html
            .select(&folderid)
            .find(|v| v.attr("selected").is_some())
            .map(|v| v.attr("value").unwrap().parse::<u32>().unwrap());

        let langtype = html
            .select(&langtype)
            .find(|v| v.attr("checked").is_some())
            .map(|v| v.attr("value").unwrap().parse::<u32>().unwrap())
            .unwrap();

        Ok(UploadGalleryInfo {
            id,
            uploaded,
            title_en,
            title_jp: title_ja,
            comment,
            category: category.unwrap(),
            categories,
            langs,
            lang: mylang,
            langtype: langtype as u8,
            folderid: folderid.unwrap(),
            folders,
            imgs,
            token: gallery_key,
            added: parse_date(info[0].as_str()).unwrap(),
            posted: parse_date(info[1].as_str()),
            files: info[2].parse().unwrap(),
            file_size: crate::unit::parse_to_bytes(&info[3]).unwrap(),
            parent: info[4].parse().ok(),
            explunged: info[5] != "No",
            visible: info[6] == "Yes",
            not_ml,
        })
    }
}
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Gallery {
    name: String,
    uid: u64,
    date: Duration,
    files: u64,
    category: String,
}
