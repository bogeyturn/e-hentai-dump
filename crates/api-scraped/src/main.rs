use std::io::Write;
use std::{
    fs::{OpenOptions, read_to_string},
    time::Duration,
};

use api_scraped::{features::search::SearchQuery, format_data};
use nord_proxy::{Proxy, ProxyTrait};
use rand::rng;
use rand::seq::IndexedRandom;
use serde::Deserialize;
use tokio::time::sleep;

#[derive(Deserialize)]
struct Pox {
    proxy: String,
}

async fn get_proxy() -> reqwest::Proxy {
    let mut rng = rng();
    let client = reqwest::Client::new();
    let resp: Vec<Pox> = client
        .get("https://cdn.jsdelivr.net/gh/proxifly/free-proxy-list@main/proxies/all/data.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    reqwest::Proxy::all(&resp.choose(&mut rng).unwrap().proxy).unwrap()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut s = api_scraped::Session::new("ipb_member_id=6615055; ipb_pass_hash=a9dec2bc94fa2db88220b1b303f446a2; sk=ek4z7u8ia3iz8ms17331zpow7xfq; igneous=hzanhope4do7y31of; hath_perks=q-a6fc64bca3".to_owned(), None, None);
    let proxy = Proxy::new().await;
    let mut proxies = proxy
        .proxies("joZXzSeqP5Jz2RN3G7a2Q4JG", "jUTRnsn2bVhmvGRpXUnMZWWf")
        .into_iter()
        .filter(|v| v.load > 20)
        .collect::<Vec<_>>();
    proxies.sort_by_key(|v| v.load as i32 * 1);
    let p = &proxies[0..4];
    let p = p.iter().map(|v| v.proxy.clone()).collect::<Vec<_>>();
    let mut last = read_to_string("ids")
        .unwrap()
        .lines()
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<u64>().unwrap())
        .max()
        .unwrap();

    s.set_proxies(p);
    let mut rng = rand::rng();

    loop {
        println!("{}", last);
        let d;
        loop {
            let temp = s
                .search(SearchQuery {
                    query: None,
                    pid: Some(last),
                    forward: false,
                    cat: None,
                    advanced: None,
                })
                .await;
            match temp {
                Ok(data) => {
                    d = data;
                    break;
                }
                Err(err) => {
                    println!("Error: {}", err);
                    let proxy = Proxy::new().await;
                    let proxies = proxy
                        .proxies("joZXzSeqP5Jz2RN3G7a2Q4JG", "jUTRnsn2bVhmvGRpXUnMZWWf")
                        .into_iter()
                        .collect::<Vec<_>>();
                    s.set_proxies(vec![proxies.choose(&mut rng).unwrap().proxy.clone()]);
                    sleep(Duration::from_secs(20)).await;
                }
            }
        }
        last = d.items.first().unwrap().id;
        let (ids, ratings, favs) = format_data(d);
        let file = |file_path| {
            OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .unwrap()
        };

        writeln!(&mut file("ids"), "\n{}", ids).unwrap();
        if !ratings.is_empty() {
            writeln!(&mut file("ratings"), "\n{}", ratings).unwrap();
        }
        if !favs.is_empty() {
            writeln!(&mut file("favs"), "\n{}", favs).unwrap();
        }

        sleep(Duration::from_secs(3)).await
    }
}
