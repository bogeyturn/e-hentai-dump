use crate::{Namespace, Operator};

pub fn parse_op(input: &str) -> (Option<Operator>, &str) {
    let op = match input.chars().next() {
        Some('-') => Some(Operator::Exclude),
        Some('~') => Some(Operator::Or),
        _ => None,
    };
    let opsome = op.is_some();
    (op, if opsome { &input[1..] } else { input })
}

pub fn parse_prefix(input: &str) -> Option<(Namespace, &str)> {
    let input = input.trim();
    let (ns_key, input) = input.split_once(':')?;
    let namespace = match ns_key {
        "artist" | "a" => Namespace::Artist,
        "character" | "c" | "char" => Namespace::Character,
        "cosplayer" | "cos" => Namespace::Cosplayer,
        "female" | "f" => Namespace::Female,
        "male" | "m" => Namespace::Male,
        "group" | "g" | "circle" => Namespace::Group,
        "mixed" | "x" => Namespace::Mixed,
        "other" | "o" => Namespace::Other,
        "parody" | "p" => Namespace::Parody,
        "reclass" | "r" => Namespace::Reclass,
        "language" | "l" => Namespace::Language,
        "loc" => Namespace::Location,
        "tag" => Namespace::Tag,
        "title" => Namespace::Title,
        "uploader" => Namespace::Uploader,
        "uploaduid" => Namespace::UploadUID,
        "gid" => Namespace::GID,
        "comment" => Namespace::Comment,
        "favnote" => Namespace::Favnote,
        _ => Namespace::Unknown(ns_key.to_string()),
    };
    Some((namespace, input))
}
