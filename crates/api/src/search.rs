use chrono::{DateTime, Datelike as _, Duration, TimeZone as _, Timelike as _, Utc};
use db_creator::{Category, Db, Item, TagPrefix};
use log::warn;
use search_parser::{Operator, SearchTerm};

type FilterData = Vec<SearchTerm>;

fn matches(filters: &[(bool, bool)]) -> bool {
    let mut i = 0;
    while i < filters.len() {
        let (is_or, matched) = filters[i];

        if is_or {
            let mut or_chain_matched = matched;
            i += 1;
            while i < filters.len() && filters[i].0 {
                or_chain_matched |= filters[i].1;
                i += 1;
            }
            if !or_chain_matched {
                return false;
            }
        } else {
            if !matched {
                return false;
            }
            i += 1;
        }
    }
    true
}

pub fn filter_func(db: &Db, item: &Item, search: &SearchData) -> bool {
    let mut filters = vec![];
    if item.current_gid != item.gid {
        return false;
    }

    if item.filecount < search.min_pages || item.filecount > search.max_pages {
        return false;
    }
    if item.rating < search.min_rating {
        return false;
    }
    if let Some(v) = search.explunged {
        if item.expunged != v {
            return false;
        }
    }

    if item.torrentcount == 0 && search.torrent {
        return false;
    }

    if !search
        .category
        .contains(Category::from_bits(item.category).unwrap())
    {
        return false;
    }
    let tags = db.get_tags(item.tags.clone());
    for filter in &search.filter {
        let matcher = |internal: &str| match &filter.value {
            search_parser::Value::Exact(s) => internal == s.trim(),
            search_parser::Value::Wildcard(v) => internal.contains(v),
            search_parser::Value::None(v) => internal.contains(v),
        };

        macro_rules! tag {
            ($category:expr) => {
                tags.iter()
                    .any(|v| v.category == $category as u8 && matcher(db.get_tag(v.id)))
            };
        }

        let matched = match &filter.namespace {
            Some(filt) => match filt {
                search_parser::Namespace::Artist => tag!(TagPrefix::Artist),
                search_parser::Namespace::Character => tag!(TagPrefix::Character),
                search_parser::Namespace::Cosplayer => tag!(TagPrefix::Cosplayer),
                search_parser::Namespace::Female => tag!(TagPrefix::Female),
                search_parser::Namespace::Male => tag!(TagPrefix::Male),
                search_parser::Namespace::Group => tag!(TagPrefix::Group),
                search_parser::Namespace::Mixed => tag!(TagPrefix::Mixed),
                search_parser::Namespace::Other => tag!(TagPrefix::Other),
                search_parser::Namespace::Parody => tag!(TagPrefix::Parody),
                search_parser::Namespace::Reclass => tag!(TagPrefix::Reclass),
                search_parser::Namespace::Language => tag!(TagPrefix::Language),
                search_parser::Namespace::Location => tag!(TagPrefix::Location),

                search_parser::Namespace::Tag => tags.iter().any(|v| matcher(db.get_tag(v.id))),
                search_parser::Namespace::Uploader => match item.uploader {
                    Some(s) => matcher(db.get_user(s)),
                    None => false,
                },
                search_parser::Namespace::Title => {
                    matcher(db.get_str(item.title))
                        || item
                            .title_jpn
                            .map(|v| matcher(db.get_str(v)))
                            .unwrap_or_default()
                }

                search_parser::Namespace::UploadUID => todo!(),
                search_parser::Namespace::Comment => todo!(),
                search_parser::Namespace::Favnote => todo!(),

                search_parser::Namespace::GID => {
                    item.gid == filter.value.str().parse::<u64>().unwrap_or(0)
                }
                search_parser::Namespace::Unknown(a) => {
                    warn!("Unknown namespace: {} {}", a, filter.value.str());
                    false
                }
            },
            None => {
                tags.iter().any(|v| matcher(db.get_tag(v.id)))
                    || matcher(db.get_str(item.title))
                    || item
                        .title_jpn
                        .map(|v| matcher(db.get_str(v)))
                        .unwrap_or_default()
            }
        };

        match filter.operator {
            Some(Operator::Exclude) => {
                if matched {
                    return false;
                }
            }
            Some(Operator::Or) => filters.push((true, matched)),
            None => filters.push((false, matched)),
        };
    }

    matches(&filters)
}

pub trait Search {
    fn search<'a>(
        &'a self,
        search: SearchData,
        size: usize,
    ) -> (Vec<&'a Item>, f64, u64, bool, bool);
    fn search_fast<'a>(&'a self, search: SearchData, size: usize) -> (Vec<&'a Item>, bool, bool);
}
pub struct SearchData {
    pub filter: FilterData,
    pub pagination: Pagination,
    pub explunged: Option<bool>,
    pub torrent: bool,
    pub min_rating: f64,
    pub min_pages: u32,
    pub max_pages: u32,
    pub category: Category,
}

pub enum Unit {
    Year,
    Month,
    Day,
    Week,
}
pub enum Pagination {
    Jump { unit: Unit, value: u32 },
    Range(u8),
    Seek(DateTime<Utc>),
    Id { id: Option<u64>, forward: bool },
}

impl Default for SearchData {
    fn default() -> Self {
        Self {
            filter: Vec::new(),
            explunged: Some(false),
            torrent: false,
            min_rating: 0.0,
            min_pages: 0,
            max_pages: u32::MAX,
            category: Category::all(),
            pagination: Pagination::Id {
                id: None,
                forward: true,
            },
        }
    }
}
fn last_day_of_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let first_next = Utc.with_ymd_and_hms(ny, nm, 1, 0, 0, 0).single().unwrap();
    let last = first_next - Duration::days(1);
    last.day()
}

fn subtract_months(dt: DateTime<Utc>, months: i64) -> DateTime<Utc> {
    let year = dt.year();
    let month0 = dt.month0() as i32;
    let total = year as i64 * 12 + month0 as i64 - months;

    let new_year = (total / 12) as i32;
    let new_month0 = (total % 12) as u32;
    let new_month = new_month0 + 1;

    let day = dt.day();
    let max_day = last_day_of_month(new_year, new_month);
    let clamped_day = day.min(max_day);

    Utc.with_ymd_and_hms(
        new_year,
        new_month,
        clamped_day,
        dt.hour(),
        dt.minute(),
        dt.second(),
    )
    .single()
    .unwrap()
        + Duration::nanoseconds(dt.nanosecond() as i64)
}

impl Search for Db {
    fn search_fast<'a>(&'a self, search: SearchData, size: usize) -> (Vec<&'a Item>, bool, bool) {
        let rsize = size;
        let mut size = size + 1;
        let mut out = Vec::with_capacity(size);
        macro_rules! mapping {
            ($iter:expr) => {
                while let Some(item) = $iter.next() {
                    if filter_func(self, item, &search) {
                        out.push(item);
                        if out.len() >= size {
                            break;
                        }
                    }
                }
            };
        }

        let (offset, forward) = match search.pagination {
            Pagination::Range(r) => (self.items.len() * r.clamp(0, 100) as usize / 100, true),
            Pagination::Id { id, forward } => (
                match id {
                    Some(page) => match self.find_nearest(page, forward) {
                        Some((_, offset)) => *offset,
                        None => match forward {
                            true => 0,
                            false => self.items.len(),
                        },
                    },
                    None => match forward {
                        true => 0,
                        false => self.items.len(),
                    },
                },
                forward,
            ),
            _ => todo!(),
        };

        let mut first = true;
        let mut last = true;

        if forward {
            let mut map = self.items[offset..].iter();
            mapping!(map);
            if out.len() > rsize {
                out.pop();
                last = false;
            }

            let mut map = self.items[..offset].iter().rev();
            size = 1;
            mapping!(map);
            if out.len() > rsize {
                out.pop();
                first = false
            }
        } else {
            let mut map = self.items[..offset].iter().rev();
            mapping!(map);
            if out.len() > rsize {
                out.pop();
                first = false;
            }

            out.reverse();

            let mut map = self.items[offset..].iter();
            size = 1;
            mapping!(map);
            if out.len() > rsize {
                out.pop();
                last = false;
            }
        };
        (out, first, last)
    }

    fn search<'a>(
        &'a self,
        mut search: SearchData,
        size: usize,
    ) -> (Vec<&'a Item>, f64, u64, bool, bool) {
        let filtered: Vec<&Item> = self
            .items
            .iter()
            .filter(|item| filter_func(self, item, &search))
            .collect();
        let count = filtered.len();
        if let Pagination::Jump { unit, value } = search.pagination {
            let now = chrono::Utc::now();

            search.pagination = Pagination::Seek(match unit {
                Unit::Day => now - Duration::days(value as i64),
                Unit::Week => now - Duration::weeks(value as i64),

                Unit::Month => subtract_months(now, value as i64),
                Unit::Year => subtract_months(now, value as i64 * 12),
            });
        }

        let start_idx = match search.pagination {
            Pagination::Jump { .. } => unreachable!(),
            Pagination::Range(i) => {
                (i.clamp(0, 100) as f64 / 100.0 * filtered.len() as f64).round() as usize
            }
            Pagination::Seek(duration) => filtered
                .iter()
                .position(|item| item.posted <= duration.timestamp() as u64)
                .unwrap_or(0),
            Pagination::Id { id, forward } => match id {
                Some(page_id) => {
                    if forward {
                        filtered
                            .iter()
                            .position(|item| item.gid >= page_id)
                            .unwrap_or(0)
                    } else {
                        filtered
                            .iter()
                            .position(|item| item.gid <= page_id)
                            .unwrap_or(filtered.len())
                    }
                }
                None => {
                    if forward {
                        0
                    } else {
                        filtered.len() - 1
                    }
                }
            },
        };

        let forward = matches!(search.pagination, Pagination::Id { id, forward: true });

        let out: Vec<&Item> = if forward {
            filtered
                .iter()
                .skip(start_idx)
                .take(size)
                .copied()
                .collect()
        } else {
            let end_idx = start_idx + 1;
            let start = end_idx.saturating_sub(size);
            filtered[start..end_idx].iter().copied().collect()
        };
        let first_item_idx = self
            .items
            .iter()
            .position(|item| &item == out.first().unwrap())
            .unwrap_or(0);
        let progress = first_item_idx as f64 / self.items.len() as f64;

        let first = out.first() == filtered.first();
        let last = out.last() == filtered.last();

        (out, progress, count as u64, first, last)
    }
}
