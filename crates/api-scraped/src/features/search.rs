use std::fmt::Display;

use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::Session;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct AdvancedConfig {
    explunged: Option<bool>,
    require_torrent: Option<bool>,
    min_pages: Option<u32>,
    max_pages: Option<u32>,
    min_rating: Option<u8>,
    disable_lang: Option<bool>,
    disable_uploader: Option<bool>,
    disable_tags: Option<bool>,
}

impl AdvancedConfig {
    fn to_string(&self) -> Option<String> {
        let mut items = vec!["advsearch=1".to_owned()];
        if self.explunged.unwrap_or_default() {
            items.push("f_sh=on".to_owned());
        }

        if self.require_torrent.unwrap_or_default() {
            items.push("f_sto=on".to_owned());
        }

        if let Some(min_pages) = self.min_pages {
            items.push(format!("f_spf={min_pages}"));
        }

        if let Some(max_pages) = self.max_pages {
            items.push(format!("f_spt={max_pages}"));
        }

        if let Some(min_rating) = self.min_rating {
            items.push(format!("f_srdd={min_rating}"));
        }

        if self.disable_lang.unwrap_or_default() {
            items.push("f_sfl=on".to_owned());
        }

        if self.disable_uploader.unwrap_or_default() {
            items.push("f_sfu=on".to_owned());
        }

        if self.disable_tags.unwrap_or_default() {
            items.push("f_sft=on".to_owned());
        }

        if items.len() != 1 {
            Some(items.join("&"))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct SearchInfo {
    pub img: Option<String>,
    pub id: u64,
    pub token: String,
    pub category: String,
    pub publisher: String,
    pub published: String,
    pub disowned: bool,
    pub new: bool,
    pub pages: u32,
    pub rating: u8,
    pub tags: Vec<String>,
    pub title: String,
    pub voted: bool,
    pub favorite: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Search {
    progress_min: f64,
    progress_max: f64,
    pub items: Vec<SearchInfo>,
    first: bool,
    last: bool,
    count: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct SearchQuery {
    pub query: Option<String>,
    pub pid: Option<u64>,
    pub forward: bool,
    pub cat: Option<u16>,
    pub advanced: Option<AdvancedConfig>,
}

impl Display for SearchQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items = vec!["inline_set=dm_e".to_string()];
        if let Some(v) = &self.query {
            items.push(format!("f_search={}", urlencoding::encode(v)));
        }
        if let Some(adv) = self.advanced.as_ref().map(|v| v.to_string()).flatten() {
            items.push(adv)
        }

        if let Some(categories) = self.cat {
            items.push(format!("f_cats={categories}"));
        }
        if let Some(v) = &self.pid {
            items.push(format!(
                "{}={}",
                match self.forward {
                    true => "next",
                    false => "prev",
                },
                v
            ));
        }

        write!(f, "{}", items.join("&"))
    }
}

pub fn star_parse(s: ElementRef) -> u8 {
    let s = s
        .attr("style")
        .unwrap_or_default()
        .split_once("background-position:")
        .unwrap()
        .1;
    let mut ss = s.match_indices("px").map(|v| v.0);
    ss.next();
    let s = &s[..ss.next().unwrap()];
    match s.trim() {
        "0px -1" => 10,
        "-16px -1" => 8,
        "-32px -1" => 6,
        "-48px -1" => 4,
        "-64px -1" => 2,
        "-80px -1" => 0,
        "0px -21" => 9,
        "-16px -21" => 7,
        "-32px -21" => 5,
        "-48px -21" => 3,
        "-64px -21" => 1,
        r => unimplemented!("{}", r),
    }
}

pub struct InfoSelectors {
    table: Selector,
    img: Selector,
    link: Selector,
    category: Selector,
    published: Selector,
    uploader: Selector,
    pages: Selector,
    rating: Selector,
    title: Selector,
    tags: Selector,
}

impl Default for InfoSelectors {
    fn default() -> Self {
        Self::search()
    }
}

impl InfoSelectors {
    pub fn search() -> Self {
        let table = Selector::parse("table.glte > tbody > tr").unwrap();
        let img = Selector::parse("td.gl1e > div > a > img").unwrap();
        let link = Selector::parse("td.gl1e > div > a").unwrap();
        let category = Selector::parse(".gl3e > div:nth-child(1)").unwrap();
        let published = Selector::parse(".gl3e > div:nth-child(2)").unwrap();
        let uploader = Selector::parse(".gl3e > div:nth-child(4)").unwrap();
        let pages = Selector::parse(".gl3e > div:nth-child(5)").unwrap();
        let rating = Selector::parse(".gl3e > div:nth-child(3)").unwrap();
        let title = Selector::parse(".gl4e > .glink").unwrap();
        let tags = Selector::parse(".gl4e td > div").unwrap();
        Self {
            table,
            img,
            link,
            category,
            published,
            uploader,
            pages,
            rating,
            title,
            tags,
        }
    }
}
pub fn extract_info(html: &Html, selectors: InfoSelectors) -> Vec<SearchInfo> {
    let mut generated = vec![];
    let InfoSelectors {
        table,
        img,
        link,
        category,
        published,
        uploader,
        pages,
        rating,
        title,
        tags,
    } = selectors;

    for child in html.select(&table) {
        let img = child
            .select(&img)
            .next()
            .and_then(|v| v.attr("src").map(|v| v.to_owned()));
        let href = child
            .select(&link)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .to_owned();
        let category = child
            .select(&category)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let published_node = child.select(&published).next().unwrap();
        assert!(
            published_node
                .attr("id")
                .unwrap_or_default()
                .starts_with("posted_"),
        );
        let fav = match published_node.attr("style") {
            Some(v) => {
                let color = v
                    .split_once("border-color:")
                    .unwrap()
                    .1
                    .split_once(";")
                    .unwrap()
                    .0
                    .trim();
                Some(match color {
                    "rgb(0, 0, 0)" | "#000" => 0,
                    "rgb(255, 0, 0)" | "#f00" => 1,
                    "rgb(255, 170, 0)" | "#fa0" => 2,
                    "rgb(221, 221, 0)" | "#dd0" => 3,
                    "rgb(0, 136, 0)" | "#080" => 4,
                    "rgb(153, 255, 68)" | "#9f4" => 5,
                    "rgb(68, 187, 255)" | "#4bf" => 6,
                    "rgb(0, 0, 255)" | "#00f" => 7,
                    "rgb(85, 0, 136)" | "#508" => 8,
                    "rgb(238, 136, 238)" | "#e8e" => 9,
                    v => unimplemented!("{v}"),
                })
            }
            None => None,
        };
        let disowned = published_node.inner_html().contains("<s");
        let new = published_node.html().contains("class=\"glnew\"");

        let published = published_node.text().collect::<String>();
        let uploader = child
            .select(&uploader)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let pages = child
            .select(&pages)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let title = child
            .select(&title)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let tags = child
            .select(&tags)
            .map(|v| v.attr("title").unwrap().to_string())
            .collect::<Vec<_>>();
        let rating_px = child.select(&rating).next().unwrap();

        let sub = href.split_once("/g/").unwrap().1.split_once("/").unwrap();
        let info = SearchInfo {
            favorite: fav,
            img,
            voted: rating_px
                .attr("class")
                .unwrap_or_default()
                .split(" ")
                .count()
                > 1,
            id: sub.0.trim().parse().unwrap(),
            token: sub.1.trim().to_string().replace("/", ""),
            category: category.trim().to_owned(),
            publisher: uploader.trim().to_owned(),
            published: published.trim().to_owned(),
            new,
            disowned,
            pages: pages
                .to_lowercase()
                .replace("pages", "")
                .replace("page", "")
                .trim()
                .parse()
                .expect(&pages.to_lowercase()),
            rating: star_parse(rating_px),
            tags,
            title,
        };
        generated.push(info);
    }
    generated
}

impl Session {
    pub async fn search(&self, query: SearchQuery) -> anyhow::Result<Search> {
        let content = self
            .get_text(format!("https://exhentai.org/?{}", query.to_string()))
            .await?;
        let html = Html::parse_document(&content);
        if html
            .select(&crate::selector(".ido > div > p"))
            .next()
            .map(|v| v.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            == "No hits found"
        {
            return Ok(Search {
                progress_min: 0.0,
                progress_max: 1.0,
                items: Vec::new(),
                first: true,
                last: true,
                count: String::new(),
            });
        }

        let progress_min = content
            .split_once("var rangemin=")
            .expect(&content)
            .1
            .split_once(";")
            .unwrap()
            .0
            .trim()
            .parse::<f64>()
            .unwrap()
            / 100.0;
        let progress_max = content
            .split_once("var rangemax=")
            .unwrap()
            .1
            .split_once(";")
            .unwrap()
            .0
            .trim()
            .parse::<f64>()
            .unwrap()
            / 98.0;

        let count = Selector::parse(".searchtext > p").unwrap();
        12;
        let before = Selector::parse("#uprev").unwrap();
        let next = Selector::parse("#unext").unwrap();
        let count = html
            .select(&count)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        let start = html
            .select(&before)
            .next()
            .map(|v| v.attr("href").unwrap_or_default())
            .unwrap_or_default()
            .trim()
            .is_empty();
        let end = html
            .select(&next)
            .next()
            .map(|v| v.attr("href").unwrap_or_default())
            .unwrap_or_default()
            .trim()
            .is_empty();
        let generated = extract_info(&html, Default::default());

        Ok(Search {
            progress_min,
            progress_max,
            items: generated,
            first: start,
            last: end,
            count,
        })
    }
}
