use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Data,
    Detail,
}

impl Mode {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "data" => Ok(Self::Data),
            "detail" => Ok(Self::Detail),
            _ => Err(format!(
                "unsupported mode '{value}', expected 'data' or 'detail'"
            )),
        }
    }
}

#[derive(Debug)]
pub enum Tag {
    Other(String),
    Female(String),
    Male(String),
    Mixed(String),
    Language(String),
    Reclass(String),
    Parody(String),
    Character(String),
    Group(String),
    Artist(String),
    Cosplayer(String),
    Location(String),
    Temp(String),
    None(String),
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Tag::None(v) => v.clone(),
            Tag::Artist(v) => format!("a:{v}"),
            Tag::Character(v) => format!("c:{v}"),
            Tag::Cosplayer(v) => format!("co:{v}"),
            Tag::Female(v) => format!("f:{v}"),
            Tag::Group(v) => format!("g:{v}"),
            Tag::Language(v) => format!("l:{v}"),
            Tag::Location(v) => format!("lo:{v}"),
            Tag::Male(v) => format!("m:{v}"),
            Tag::Mixed(v) => format!("mi:{v}"),
            Tag::Other(v) => format!("o:{v}"),
            Tag::Parody(v) => format!("p:{v}"),
            Tag::Reclass(v) => format!("r:{v}"),
            Tag::Temp(v) => format!("t:{v}"),
        };

        serializer.serialize_str(&s)
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        if !value.contains(":") {
            return Tag::None(value.to_string());
        }
        let (k, v) = value
            .split_once(":")
            .unwrap_or_else(|| panic!("Invalid tag format: {value}"));
        let value = v.to_string();
        match k {
            "other" => Tag::Other(value),
            "female" => Tag::Female(value),
            "male" => Tag::Male(value),
            "mixed" => Tag::Mixed(value),
            "language" => Tag::Language(value),
            "reclass" => Tag::Reclass(value),
            "parody" => Tag::Parody(value),
            "character" => Tag::Character(value),
            "group" => Tag::Group(value),
            "artist" => Tag::Artist(value),
            "cosplayer" => Tag::Cosplayer(value),
            "location" => Tag::Location(value),
            "temp" => Tag::Temp(value),
            _ => unimplemented!("{}", k),
        }
    }
}

#[derive(Serialize)]
pub struct Item {
    #[serde(rename = "a")]
    pub author: String,
    #[serde(rename = "c")]
    pub categories: Vec<Tag>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "g")]
    pub gid: u64,
    #[serde(rename = "i")]
    pub img: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "p")]
    pub published: u64,
    #[serde(rename = "t")]
    pub token: String,
}

impl Item {
    #[cfg(test)]
    fn new_for_test(gid: u64, token: &str) -> Self {
        Self {
            author: String::new(),
            categories: Vec::new(),
            description: None,
            gid,
            img: String::new(),
            name: String::new(),
            published: 0,
            token: token.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DetailRequest {
    pub gid: u64,
    pub token: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailArgs {
    pub limit: Option<usize>,
    pub requests_path: Option<std::path::PathBuf>,
}

#[derive(Deserialize)]
struct StoredDataItem {
    #[serde(rename = "g")]
    gid: u64,
    #[serde(rename = "t")]
    token: String,
}

pub fn select_new_data_items(items: Vec<Item>, data_dir: &Path) -> Vec<Item> {
    items
        .into_iter()
        .filter(|item| !data_dir.join(format!("{}.json", item.gid)).exists())
        .collect()
}

pub fn write_data_items(items: Vec<Item>, data_dir: &Path) -> anyhow::Result<()> {
    create_dir_all(data_dir)?;
    for item in items {
        let path = data_dir.join(format!("{}.json", item.gid));
        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string(&item)?.as_bytes())?;
    }
    Ok(())
}

pub fn detail_requests_from_items(items: &[Item]) -> Vec<DetailRequest> {
    items
        .iter()
        .map(|item| DetailRequest {
            gid: item.gid,
            token: item.token.clone(),
        })
        .collect()
}

pub fn write_detail_requests(requests: &[DetailRequest], path: &Path) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(serde_json::to_string(requests)?.as_bytes())?;
    Ok(())
}

pub fn load_detail_requests(path: &Path) -> anyhow::Result<Vec<DetailRequest>> {
    let requests = serde_json::from_reader(File::open(path)?)?;
    Ok(requests)
}

pub fn load_missing_detail_requests(
    data_dir: &Path,
    detail_dir: &Path,
    limit: Option<usize>,
) -> anyhow::Result<Vec<DetailRequest>> {
    let mut data_files = Vec::new();
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            data_files.push(path);
        }
    }
    data_files.sort_by_key(|path| numeric_stem(path).unwrap_or(u64::MAX));

    let mut requests = Vec::new();
    for path in data_files {
        let item: StoredDataItem = serde_json::from_reader(File::open(&path)?)?;
        requests.push(DetailRequest {
            gid: item.gid,
            token: item.token,
        });
    }

    Ok(filter_missing_detail_requests(requests, detail_dir, limit))
}

pub fn filter_missing_detail_requests(
    requests: Vec<DetailRequest>,
    detail_dir: &Path,
    limit: Option<usize>,
) -> Vec<DetailRequest> {
    let mut filtered = Vec::new();
    for request in requests {
        if detail_dir.join(format!("{}.json", request.gid)).exists() {
            continue;
        }
        filtered.push(request);
        if limit.is_some_and(|limit| filtered.len() >= limit) {
            break;
        }
    }
    filtered
}

pub fn write_detail_item(
    detail_dir: &Path,
    gid: u64,
    mut value: serde_json::Value,
    dumped: u64,
) -> anyhow::Result<()> {
    create_dir_all(detail_dir)?;
    if let serde_json::Value::Object(map) = &mut value {
        map.insert("dumped".to_string(), serde_json::Value::from(dumped));
    }
    let path = detail_dir.join(format!("{gid}.json"));
    let mut file = File::create(path)?;
    file.write_all(serde_json::to_string(&value)?.as_bytes())?;
    Ok(())
}

fn numeric_stem(path: &Path) -> Option<u64> {
    path.file_stem()?.to_str()?.parse().ok()
}

pub fn parse_limit_arg(args: &[String]) -> Result<Option<usize>, String> {
    parse_detail_args(args).map(|args| args.limit)
}

pub fn parse_detail_args(args: &[String]) -> Result<DetailArgs, String> {
    let mut parsed = DetailArgs {
        limit: None,
        requests_path: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--limit" => {
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| "--limit requires a value".to_string())?;
                parsed.limit = Some(
                    value
                        .parse()
                        .map_err(|_| format!("invalid --limit value '{value}'"))?,
                );
                i += 2;
            }
            "--requests" => {
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| "--requests requires a value".to_string())?;
                parsed.requests_path = Some(value.into());
                i += 2;
            }
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{self, File},
        io::Write,
        path::Path,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("downloader-{name}-{nonce}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn write_json(path: &Path, body: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(body.as_bytes()).unwrap();
    }

    #[test]
    fn mode_parser_accepts_data_and_detail_only() {
        assert_eq!(Mode::parse("data").unwrap(), Mode::Data);
        assert_eq!(Mode::parse("detail").unwrap(), Mode::Detail);
        assert!(Mode::parse("both").is_err());
    }

    #[test]
    fn data_mode_selects_items_missing_from_data_without_detail_dir() {
        let root = temp_dir("data-mode");
        let data_dir = root.join("data");
        fs::create_dir_all(&data_dir).unwrap();
        write_json(&data_dir.join("1.json"), r#"{"g":1,"t":"old"}"#);

        let items = vec![Item::new_for_test(1, "old"), Item::new_for_test(2, "new")];

        let selected = select_new_data_items(items, &data_dir);

        assert_eq!(
            selected
                .into_iter()
                .map(|item| item.gid)
                .collect::<Vec<_>>(),
            vec![2]
        );
    }

    #[test]
    fn detail_mode_loads_missing_requests_from_data_branch_files() {
        let root = temp_dir("detail-mode");
        let data_dir = root.join("data");
        let detail_dir = root.join("detail");
        fs::create_dir_all(&data_dir).unwrap();
        fs::create_dir_all(&detail_dir).unwrap();
        write_json(&data_dir.join("10.json"), r#"{"g":10,"t":"ten"}"#);
        write_json(&data_dir.join("1.json"), r#"{"g":1,"t":"one"}"#);
        write_json(&data_dir.join("2.json"), r#"{"g":2,"t":"two"}"#);
        write_json(&detail_dir.join("2.json"), r#"{"gid":2}"#);

        let requests = load_missing_detail_requests(&data_dir, &detail_dir, Some(1)).unwrap();

        assert_eq!(
            requests,
            vec![DetailRequest {
                gid: 1,
                token: "one".into()
            }]
        );
    }

    #[test]
    fn detail_mode_loads_missing_requests_from_small_request_file() {
        let root = temp_dir("detail-request-file");
        let detail_dir = root.join("detail");
        let request_file = root.join("detail_requests.json");
        fs::create_dir_all(&detail_dir).unwrap();
        write_json(&detail_dir.join("2.json"), r#"{"gid":2}"#);

        let requests = vec![
            DetailRequest {
                gid: 1,
                token: "one".into(),
            },
            DetailRequest {
                gid: 2,
                token: "two".into(),
            },
            DetailRequest {
                gid: 3,
                token: "three".into(),
            },
        ];
        write_detail_requests(&requests, &request_file).unwrap();

        let loaded = load_detail_requests(&request_file).unwrap();
        let missing = filter_missing_detail_requests(loaded, &detail_dir, Some(1));

        assert_eq!(
            missing,
            vec![DetailRequest {
                gid: 1,
                token: "one".into()
            }]
        );
    }
}
