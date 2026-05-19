use std::{collections::HashMap, num::ParseFloatError, time::Duration};

use scraper::{Html, Selector};
use serde::Serialize;

use crate::{Session, unit};

use super::{mpv::ImagePage, search::star_parse};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Info {
    pub id: u64,
    pub token: String,
    pub thumb: ImagePage,
    pub tags: Vec<String>,
    pub rating: Option<f64>,
    pub newer: Vec<(u64, String)>,
    pub category: String,
    pub title: String,
    pub alt_title: Option<String>,
    pub per_page: u32,
    pub pages: Vec<ImagePage>,
    pub posted: Duration,
    pub files: u32,
    pub size: u64,
    pub visible: bool,
    pub language: String,
    pub uploader: String,
    pub uploader_id: Option<u64>,
    pub parent: Option<Parent>,
    pub apiuid: Option<i64>,
    pub apikey: Option<String>,
    pub favorited: u64,
    pub favorite: Option<u8>,
    pub my_stars: Option<u8>,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Parent {
    pub id: u64,
    pub key: String,
}

fn parse_url(url: &str) -> (u64, String) {
    let parts = url.split_once("/g/").unwrap().1.split_once("/").unwrap();
    let id = parts.0.parse().unwrap();
    let token = parts.1.replace("/", "");
    (id, token)
}

fn parse_size_to_bytes(s: &str) -> Result<u64, ParseFloatError> {
    let s = s.trim();
    let mut parts = s.split_whitespace();
    let num_str = parts.next().unwrap_or("0");
    let unit_str = parts.next().unwrap_or("B");

    let num: f64 = num_str.parse()?;

    let bytes = match unit_str {
        "B" => num,
        "KiB" => num * 1024.0,
        "MiB" => num * 1024.0 * 1024.0,
        "GiB" => num * 1024.0 * 1024.0 * 1024.0,
        "KB" => num * 1000.0,
        "MB" => num * 1000.0 * 1000.0,
        "GB" => num * 1000.0 * 1000.0 * 1000.0,
        v => unimplemented!("{}", v),
    };

    Ok(bytes.round() as u64)
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
pub struct Comment {
    pub id: u64,
    text: String,
    votes: Option<i32>,
    voters: Vec<String>,
    posted: String,
    uploader: String,
    updated: Option<String>,
}

fn str_to_parent(s: &str) -> Parent {
    let s = s.split_once("/g/").unwrap().1;
    let mut parts = s.split('/');
    let id = parts.next().unwrap().parse().unwrap();
    let token = parts.next().unwrap();
    Parent {
        id,
        key: token.to_owned(),
    }
}

fn thumb_from_str(s: &str) -> ImagePage {
    let w = s
        .split_once("width:")
        .unwrap()
        .1
        .split_once("px")
        .unwrap()
        .0;
    let h = s
        .split_once("height:")
        .unwrap()
        .1
        .split_once("px")
        .unwrap()
        .0;
    let url = s
        .split_once("url(")
        .unwrap()
        .1
        .split_once(") 0 0")
        .unwrap()
        .0;
    ImagePage {
        id: 0,
        ratio: (w.parse().unwrap(), h.parse().unwrap()),
        key: "".to_owned(),
        name: "".to_owned(),
        width: w.parse().unwrap(),
        height: h.parse().unwrap(),
        url: url.to_owned(),
    }
}

impl Session {
    pub async fn info(&self, id: u64, token: &str, page: u32) -> anyhow::Result<Info> {
        assert!(page > 0);
        let text = self
            .get_text(format!(
                "https://exhentai.org/g/{id}/{token}/?p={}&inline_set=ts_200",
                page - 1
            ))
            .await?;
        let apikey = text
            .split_once("var apikey = \"")
            .unwrap()
            .1
            .split_once("\"")
            .unwrap()
            .0;
        let apiuid: i64 = text
            .split_once("var apiuid = ")
            .unwrap()
            .1
            .split_once(";")
            .unwrap()
            .0
            .parse()
            .unwrap();
        let html = Html::parse_document(&text);
        let alt_title = Selector::parse("#gd2 > #gj").unwrap();
        let title = Selector::parse("#gd2 > #gn").unwrap();
        let uploader = Selector::parse("#gdn").unwrap();
        // let uploader_id = Selector::parse("#gdn a").unwrap();
        let category = Selector::parse("#gdc > div").unwrap();
        let newer = Selector::parse("#gnd a").unwrap();
        let rating = Selector::parse("#gdr #rating_label").unwrap();
        let fav = Selector::parse("#fav > div").unwrap();
        let tags = Selector::parse("#taglist td > div").unwrap();
        let thumb = Selector::parse("#gd1 > div").unwrap();
        let my_stars = Selector::parse("#rating_image").unwrap();
        let page_count = Selector::parse(".ptt td a").unwrap();
        let rating = html
            .select(&rating)
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .to_lowercase();
        let imgs = Selector::parse("#gdt > a").unwrap();
        let table1 = Selector::parse("#gdd td.gdt1").unwrap();
        let table2 = Selector::parse("#gdd td.gdt2").unwrap();
        let a = Selector::parse("a").unwrap();
        let comments = Selector::parse("#cdiv > .c1").unwrap();
        let comments_when = Selector::parse(".c3").unwrap();
        let comments_score = Selector::parse(".c5 > span").unwrap();
        let comments_content = Selector::parse(".c6").unwrap();
        let comment_edit = Selector::parse(".c8 > strong").unwrap();
        let comments_upvotes = Selector::parse(".c7").unwrap();
        let favcount = crate::id("favcount");

        let td1 = html
            .select(&table1)
            .map(|v| v.text().collect::<String>().trim().to_owned())
            .collect::<Vec<_>>();
        let td2 = html.select(&table2).map(|v| v).collect::<Vec<_>>();
        let td3 = td2
            .iter()
            .map(|v| v.text().collect::<String>().trim().to_owned());
        let mapped = td1.clone().into_iter().zip(td3).collect::<HashMap<_, _>>();
        let mapped2 = td1.into_iter().zip(td2).collect::<HashMap<_, _>>();
        let posted = mapped.get("Posted:").unwrap();
        let page_count = html
            .select(&page_count)
            .map(|v| {
                v.attr("href")
                    .unwrap()
                    .split_once("?p=")
                    .map(|v| v.1)
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap()
            })
            .max()
            .unwrap()
            + 1;
        let parent = mapped2
            .get("Parent:")
            .unwrap()
            .select(&a)
            .next()
            .map(|v| str_to_parent(v.attr("href").unwrap()));
        let visible = mapped.get("Visible:").unwrap() == "Yes";
        let language = mapped.get("Language:").unwrap();
        let fsize = mapped.get("File Size:").unwrap();
        let length = mapped
            .get("Length:")
            .unwrap()
            .replace("pages", "")
            .replace("page", "")
            .trim()
            .parse::<u32>()
            .unwrap();
        let hrefs = html
            .select(&imgs)
            .map(|v| {
                let href = v.attr("href").unwrap();
                let id = href.rsplit_once("-").unwrap().1.parse::<u32>().unwrap();
                (
                    href.split_once("/s/")
                        .unwrap()
                        .1
                        .split_once("/")
                        .unwrap()
                        .0
                        .to_owned(),
                    id,
                )
            })
            .collect::<Vec<_>>();
        let fav = html
            .select(&fav)
            .next()
            .and_then(|v| v.attr("style"))
            .map(|v| {
                v.split_once("background-position:0px ")
                    .unwrap()
                    .1
                    .split_once("px;")
                    .unwrap()
                    .0
            })
            .map(|v| match v {
                "-2" => 0,
                "-21" => 1,
                "-40" => 2,
                "-59" => 3,
                "-78" => 4,
                "-97" => 5,
                "-116" => 6,
                "-135" => 7,
                "-154" => 8,
                "-173" => 9,
                v => unimplemented!("{}", v),
            });
        let my_stars = html.select(&my_stars).next().unwrap();
        let voted = my_stars
            .attr("class")
            .unwrap_or_default()
            .split(" ")
            .count()
            > 1;
        let comments = html
            .select(&comments)
            .map(|el| {
                let n = el.select(&comments_content).next().unwrap();
                let content = n.inner_html();
                let id = n
                    .attr("id")
                    .map(|v| v.replace("comment_", "").parse().unwrap())
                    .unwrap();
                let score = el.select(&comments_score).next().map(|v| {
                    v.text()
                        .collect::<String>()
                        .replace("+", "")
                        .parse::<i32>()
                        .unwrap()
                });
                let updated = el
                    .select(&comment_edit)
                    .next()
                    .map(|v| v.text().collect::<String>());
                let voters = el
                    .select(&comments_upvotes)
                    .next()
                    .map(|v| {
                        v.text()
                            .collect::<String>()
                            .split(", ")
                            .map(|user| user.trim().to_string())
                            .collect::<Vec<String>>()
                    })
                    .filter(|v| !v.is_empty())
                    .unwrap_or_default();
                let header = el
                    .select(&comments_when)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>();
                let (posted, uploader) = header
                    .split_once(" by: \u{a0} ")
                    .unwrap_or((header.as_str(), ""));
                Comment {
                    id,
                    text: content,
                    votes: score,
                    voters,
                    posted: posted.to_string(),
                    uploader: uploader.to_string(),
                    updated,
                }
            })
            .collect::<Vec<Comment>>();

        let favorited = html
            .select(&favcount)
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .replace("Never", "0")
            .replace("Once", "1")
            .replace("times", "")
            .replace(" ", "")
            .parse()
            .unwrap();

        Ok(Info {
            parent,
            id,
            apiuid: Some(apiuid),
            comments,
            favorite: fav,
            thumb: thumb_from_str(html.select(&thumb).next().unwrap().attr("style").unwrap()),
            favorited,
            apikey: Some(apikey.to_owned()),
            token: token.to_owned(),
            per_page: f64::ceil(length as f64 / page_count as f64) as u32,
            pages: html
                .select(&imgs)
                .zip(hrefs)
                .map(|(v, (key, id))| {
                    let node = v.child_elements().next().unwrap();

                    let mut style = node.attr("style").unwrap().splitn(6, [':', ';']).skip(1);
                    let w = style.next().unwrap();
                    style.next();
                    let h = style.next().unwrap();
                    let img = style.skip(1).next().unwrap().to_owned();

                    let name = node
                        .attr("title")
                        .unwrap()
                        .split_once(": ")
                        .unwrap()
                        .1
                        .to_owned();
                    ImagePage {
                        id,
                        width: w.replace("px", "").parse().unwrap(),
                        height: h.replace("px", "").parse().unwrap(),
                        ratio: (
                            w.replace("px", "").parse().unwrap(),
                            h.replace("px", "").parse().unwrap(),
                        ),
                        key,
                        name,
                        url: img,
                    }
                })
                .collect(),
            tags: html
                .select(&tags)
                .map(|v| {
                    v.attr("id")
                        .unwrap()
                        .strip_prefix("td_")
                        .unwrap()
                        .replace("_", " ")
                })
                .collect(),

            rating: match rating == "not yet rated" {
                true => None,
                false => Some(rating.rsplit_once(":").unwrap().1.trim().parse()?),
            },
            uploader: html
                .select(&uploader)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            // uploader_id: html.select(&uploader_id).next().map(|v| {
            //     v.attr("href")
            //         .unwrap()
            //         .split_once("showuser=")
            //         .expect(v.attr("href").unwrap())
            //         .1
            //         .parse()
            //         .unwrap()
            // }),
            uploader_id: None,
            newer: html
                .select(&newer)
                .map(|v| parse_url(v.attr("href").unwrap()))
                .collect::<Vec<_>>(),
            category: html
                .select(&category)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            title: html
                .select(&title)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_owned(),
            alt_title: html
                .select(&alt_title)
                .next()
                .map(|v| v.text().collect::<String>().trim().to_owned()),
            posted: unit::parse_date(&posted).unwrap(),
            files: length,
            size: parse_size_to_bytes(fsize).unwrap(),
            visible,
            language: language.to_owned(),
            my_stars: match voted {
                true => Some(star_parse(my_stars)),
                false => None,
            },
        })
    }
}
