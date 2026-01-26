use std::ops::Range;

use crate::arena::StrRef;

pub struct Torrent {
    pub added: u64,
    pub fsize: u64,
    pub hash: StrRef,
    pub name: Option<StrRef>,
    pub tsize: u64,
}

pub struct Item {
    pub gid: u64,
    pub token: StrRef,
    pub first_gid: Option<u64>,
    pub parent_gid: Option<u64>,

    pub title: StrRef,
    pub title_jpn: Option<StrRef>,
    pub thumb: StrRef,
    pub category: u8,
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

pub struct Tag {
    pub id: usize,
    pub category: u8,
}
