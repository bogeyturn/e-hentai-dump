mod arena;
mod data;
mod parser;

pub use arena::StrRef;
pub use data::Item;
pub use data::Tag;
pub use data::Torrent;
pub use parser::Category;
pub use parser::Tag as ParseTag;
pub use parser::TagPrefix;
pub use parser::Torrents1;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::ops::Range;
use std::{
    collections::{BTreeMap, HashMap},
    fs::{File, read_to_string},
    hash::Hash,
};

use crate::{
    arena::{Arena, StringArena},
    parser::Root1,
};

pub struct HashSetIdBuilder<T: Hash + Eq + PartialEq> {
    data: HashMap<T, usize>,
    counter: usize,
}

impl<T: Hash + Eq + PartialEq> Default for HashSetIdBuilder<T> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            counter: 0,
        }
    }
}

impl<T: Hash + Eq + PartialEq> HashSetIdBuilder<T> {
    pub fn insert(&mut self, item: T) -> usize {
        if let Some(name) = self.data.get(&item) {
            *name
        } else {
            self.data.insert(item, self.counter);
            self.counter += 1;
            self.counter - 1
        }
    }

    pub fn build(self) -> Vec<T> {
        let mut v = self
            .data
            .into_iter()
            .map(|(v, k)| (k, v))
            .collect::<Vec<_>>();
        v.sort_by_key(|v| v.0);
        v.into_iter().map(|(_, v)| v).collect()
    }
}

pub fn build() -> Db {
    let mut users = HashSetIdBuilder::default();
    let mut tags = HashSetIdBuilder::default();
    let mut arena = StringArena::new();
    let mut items = Vec::with_capacity(3_000_000);
    let mut t_arena = Arena::new();
    let mut to_arena = Arena::new();
    let disowned = read_to_string("disowned")
        .unwrap()
        .lines()
        .filter_map(|v| v.split_once(":"))
        .map(|v| (v.0.parse::<u64>().unwrap(), v.1.to_owned()))
        .collect::<HashMap<_, _>>();
    let mut cur = HashMap::new();
    for file in std::fs::read_dir("archive").unwrap() {
        let file = file.unwrap().path();
        let file: Vec<Root1> = simd_json::serde::from_reader(File::open(&file).unwrap()).unwrap();
        for file in file {
            let item = transform(
                file,
                &mut users,
                &mut tags,
                &disowned,
                &mut arena,
                &mut t_arena,
                &mut to_arena,
                &mut cur,
            );
            items.push((item.gid, item));
        }
    }
    for file in std::fs::read_dir("detail").unwrap() {
        let file = file.unwrap().path();
        let file: Root1 = serde_json::from_reader(File::open(file).unwrap()).unwrap();
        let item = transform(
            file,
            &mut users,
            &mut tags,
            &disowned,
            &mut arena,
            &mut t_arena,
            &mut to_arena,
            &mut cur,
        );
        items.push((item.gid, item));
    }
    let cur = cur
        .into_iter()
        .flat_map(|(_, v)| {
            let max = *v.iter().max().unwrap();
            v.into_iter().map(move |v| (v, max))
        })
        .collect::<HashMap<_, _>>();

    let mut new_items = BTreeMap::new();
    new_items.extend(items);
    let len = new_items.len();
    let (seek, items) = new_items.into_iter().rev().enumerate().fold(
        (BTreeMap::new(), Vec::with_capacity(len)),
        |(mut map, mut vec), (i, (gid, mut value))| {
            map.insert(gid, i);
            value.current_gid = *cur.get(&gid).unwrap();
            vec.push(value);
            (map, vec)
        },
    );

    let mut db = Db {
        users: users
            .build()
            .into_iter()
            .map(|v| arena.add(&v))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        tags: tags
            .build()
            .into_iter()
            .map(|v| arena.add(&v))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        items,
        arena,
        t_arena,
        seek,
        to_arena,
    };
    db.arena.finalize();
    db.to_arena.finalize();
    db.t_arena.finalize();
    db
}

pub fn log_db_memory(db: &Db) {
    let items_stack = size_of_val(&db.items);
    let items_heap: usize = db.items.iter().map(|item| item.deep_size()).sum();

    let users_heap = db.users.len() * std::mem::size_of::<StrRef>();
    let tags_heap = db.tags.len() * std::mem::size_of::<StrRef>();
    let arena_heap = db.arena.data.capacity(); // in bytes
    let to_arena_heap = db.to_arena.data.capacity() * std::mem::size_of::<Torrent>();
    let t_arena_heap = db.t_arena.data.capacity() * std::mem::size_of::<Tag>();

    println!(
        "Db memory usage:\n\
        - items: {} entries, stack ~{}, heap ~{}\n\
        - users: {} entries, heap ~{}\n\
        - tags: {} entries, heap ~{}\n\
        - arena: ~{} bytes\n\
        - torrent_arena: ~{} bytes\n\
        - tag_arena: ~{} bytes",
        db.items.len(),
        items_stack,
        items_heap,
        db.users.len(),
        users_heap,
        db.tags.len(),
        tags_heap,
        arena_heap,
        to_arena_heap,
        t_arena_heap,
    );
}

impl Item {
    fn deep_size(&self) -> usize {
        let mut size = 0;

        size += size_of::<Item>();
        size
    }
}

pub struct Db {
    users: Box<[StrRef]>,
    tags: Box<[StrRef]>,
    arena: StringArena,
    to_arena: Arena<Torrent>,
    t_arena: Arena<Tag>,
    pub items: Vec<Item>,
    pub seek: BTreeMap<u64, usize>,
}

impl Db {
    pub fn get_torrents(&self, range: Range<usize>) -> &[Torrent] {
        self.to_arena.get_range(range)
    }
    pub fn get_tags(&self, range: Range<usize>) -> &[Tag] {
        self.t_arena.get_range(range)
    }
    pub fn get_user(&self, id: usize) -> &str {
        self.get_str(self.users[id])
    }
    pub fn get_str(&self, id: StrRef) -> &str {
        self.arena.get(id)
    }
    pub fn get_tag(&self, id: usize) -> &str {
        self.get_str(self.tags[id])
    }
    pub fn find_nearest(&self, id: u64, after: bool) -> Option<(&u64, &usize)> {
        if after {
            self.seek.range(id..).next()
        } else {
            self.seek.range(..=id).next_back()
        }
    }
}

fn transform(
    file: Root1,
    users: &mut HashSetIdBuilder<String>,
    tags: &mut HashSetIdBuilder<String>,
    disowned: &HashMap<u64, String>,
    arena: &mut StringArena,
    tag_arena: &mut Arena<Tag>,
    torrent_arena: &mut Arena<Torrent>,
    cur_map: &mut HashMap<u64, Vec<u64>>,
) -> Item {
    let uid = if file.uploader == "(Disowned)" {
        disowned.get(&file.gid).cloned()
    } else {
        Some(file.uploader)
    };

    let uid = uid.map(|v| users.insert(v));
    let mut _tags = vec![];
    for tag in file.tags {
        _tags.push(data::Tag {
            id: tags.insert(tag.tag),
            category: tag.prefix as u8,
        });
    }
    cur_map
        .entry(file.first_gid.unwrap_or(file.gid))
        .or_insert(vec![])
        .push(file.gid);
    Item {
        gid: file.gid,
        token: arena.add(&file.token),
        first_gid: file.first_gid,
        parent_gid: file.parent_gid,
        current_gid: 0,
        title: arena.add(&file.title),
        title_jpn: file.title_jpn.map(|v| arena.add(&v)),
        thumb: arena.add(&file.thumb.replace("https://ehgt.org/", "")),
        category: file.category.bits(),
        rating: file.rating,
        tags: tag_arena.add_slice(_tags),
        filecount: file.filecount,
        filesize: file.filesize,
        torrentcount: file.torrentcount,
        torrents: torrent_arena.add_slice(
            file.torrents
                .into_iter()
                .map(|v| Torrent {
                    added: v.added,
                    fsize: v.fsize,
                    hash: arena.add(&v.hash),
                    name: v.name.map(|v| arena.add(&v)),
                    tsize: v.tsize,
                })
                .collect(),
        ),
        uploader: uid,
        posted: file.posted,
        dumped: file.dumped,
        expunged: file.expunged,
    }
}
