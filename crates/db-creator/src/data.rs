use std::ops::Range;

use crate::arena::StrRef;

pub struct Torrent {
    pub added: u64,
    pub fsize: u64,
    pub hash: StrRef,
    pub name: Option<StrRef>,
    pub tsize: u64,
}

#[derive(Debug)]
pub struct Item {
    pub gid: u64,
    pub token: StrRef,
    pub current_gid: u64,
    pub first_gid: Option<u64>,
    pub parent_gid: Option<u64>,

    pub title: StrRef,
    pub title_jpn: Option<StrRef>,
    pub thumb: StrRef,
    pub category: u16,
    pub rating: f64,

    pub tags: Range<usize>,

    pub filecount: u32,
    pub filesize: u64,
    pub torrentcount: u32,
    pub torrents: Range<usize>,

    pub uploader: Option<usize>,
    pub posted: u64,
    pub dumped: u64,
    pub expunged: bool,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.gid == other.gid
    }
}
impl Eq for Item {}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Tag {
    pub id: usize,
    pub category: u8,
}
