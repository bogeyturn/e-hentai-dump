use std::time::Duration;

use anyhow::bail;
use chrono::{NaiveDateTime, TimeZone as _, Utc};

pub fn parse_date(date_str: &str) -> Option<Duration> {
    let naive = NaiveDateTime::parse_from_str(date_str.trim(), "%Y-%m-%d %H:%M").ok()?;

    Some(Duration::from_secs(
        Utc.from_utc_datetime(&naive).timestamp() as u64,
    ))
}
pub fn parse_to_bytes(input: &str) -> anyhow::Result<u64> {
    let s = input.trim().to_lowercase();

    let mut split = s.splitn(2, |c: char| !c.is_ascii_digit() && c != '.');
    let value_str = split.next().ok_or(anyhow::anyhow!("InvalidFormat"))?;
    let unit_str = split.next().unwrap_or("").trim();

    let value: f64 = value_str.parse()?;

    let is_bit = unit_str.contains("bit") || unit_str.ends_with('b') && !unit_str.contains("ib");
    let is_byte = unit_str.contains("byte") || unit_str.contains("b");

    if !is_bit && !is_byte {
        bail!("UnknownUnit");
    }

    let multiplier = if unit_str.contains("kib") {
        1024_f64
    } else if unit_str.contains("mib") {
        1024_f64.powi(2)
    } else if unit_str.contains("gib") {
        1024_f64.powi(3)
    } else if unit_str.contains("tib") {
        1024_f64.powi(4)
    } else if unit_str.contains('k') {
        1_000_f64
    } else if unit_str.contains('m') {
        1_000_f64.powi(2)
    } else if unit_str.contains('g') {
        1_000_f64.powi(3)
    } else if unit_str.contains('t') {
        1_000_f64.powi(4)
    } else {
        1_f64
    };

    let mut bytes_per_sec = value * multiplier;

    if is_bit {
        bytes_per_sec /= 8.0;
    }

    Ok(bytes_per_sec.round() as u64)
}
