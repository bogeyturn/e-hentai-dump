use std::{
    env,
    fs::{OpenOptions, read_dir, read_to_string},
};

use crate::{
    Session,
    features::{
        bounty, fav,
        search::{Search, SearchQuery},
        torrents,
    },
    format_data,
};

fn get_session() -> Session {
    dotenv::dotenv().unwrap();

    Session::cookie(
        &env::var("MEMBER_ID").unwrap(),
        &env::var("PASS_HASH").unwrap(),
        &env::var("SK").unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

#[tokio::test]
async fn favorite() {
    let session = get_session();
    session
        .add_favorite(3510047, "728ab99a36", 1, "abcd")
        .await
        .unwrap();
}

#[tokio::test]
async fn nofavorite() {
    let session = get_session();
    session
        .remove_favorite(3510047, "728ab99a36")
        .await
        .unwrap();
}

#[tokio::test]
async fn full_star() {
    let session = get_session();
    let info = session.info(3510047, "728ab99a36", 1).await.unwrap();
    session
        .rate(info.id, &info.token, info.apiuid, &info.apikey, 1)
        .await
        .unwrap();
}

#[tokio::test]
async fn info() {
    let session = get_session();
    let info = session.info(3510047, "728ab99a36", 1).await.unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn info2() {
    let session = get_session();

    let info = session.info(3510124, "f8175ecfb8", 2).await.unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn search() {
    let session = get_session();

    let info = session
        .search(SearchQuery {
            query: Some("English \"nudity only\"".to_owned()),
            pid: None,
            forward: true,
            advanced: None,
            cat: None,
            range: None,
            seek: None,
            jump: None,
        })
        .await
        .unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn search2() {
    let session = get_session();

    let info = session
        .search(SearchQuery {
            query: None,
            pid: None,
            forward: true,
            advanced: None,
            cat: None,
            range: None,
            seek: None,
            jump: None,
        })
        .await
        .unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn local_search() {
    let session = get_session();

    let info = session
        .local_search(SearchQuery {
            query: None,
            pid: None,
            forward: true,
            advanced: None,
            cat: None,
            range: None,
            seek: None,
            jump: None,
        })
        .await
        .unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn vote_comment() {
    let session = get_session();

    let info = session.info(3510124, "f8175ecfb8", 1).await.unwrap();
    let vote = session
        .vote_comment(
            3510124,
            "f8175ecfb8",
            (&info.comments[1]).id,
            true,
            info.apiuid,
            &info.apikey,
        )
        .await
        .unwrap();
    println!("{:#?}", vote);
}
#[tokio::test]
async fn get_img() {
    let session = get_session();
    let img = session
        .next_img(530350, "cce33ecbeb", 2, None)
        .await
        .unwrap();
    println!("{:#?}", img);
}

#[tokio::test]
async fn tag_vote_error() {
    let session = get_session();

    let info = session.info(3559314, "57c783505b", 1).await.unwrap();
    let vote = session
        .tag_vote(
            3559314,
            "57c783505b",
            "language:spanish",
            true,
            info.apiuid,
            &info.apikey,
        )
        .await
        .unwrap();
    println!("{:#?}", vote);
}

#[tokio::test]
async fn toplist() {
    let session = get_session();

    let toplist = session.toplists().await.unwrap();
    println!("{:#?}", toplist);
}
#[tokio::test]
async fn toplist2() {
    let session = get_session();

    let toplist = session.toplist(21, 1).await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn news() {
    let session = get_session();

    let toplist = session.news().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn bounty() {
    let session = get_session();

    let toplist = session
        .bounty(
            "",
            1,
            bounty::BountyType::All,
            bounty::BountyStatus::Open,
            Some(6940243),
        )
        .await
        .unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn popular() {
    let session = get_session();

    let toplist = session.popular().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn list_fav() {
    let session = get_session();

    let toplist = session
        .list_favorite(fav::FeatureSearchQuery {
            query: None,
            pid: None,
            forward: false,
            cat: None,
            order_by_published: false,
        })
        .await
        .unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn tos() {
    let session = get_session();

    let toplist = session.tos().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn exchange() {
    let session = get_session();

    let toplist = session.exchange_info(false).await.unwrap();
    println!("{:#?}", toplist);
}
#[tokio::test]
async fn exchange2() {
    let session = get_session();

    let toplist = session.exchange_info(true).await.unwrap();
    println!("{:#?}", toplist);
}
#[tokio::test]
async fn perks() {
    let session = get_session();

    let toplist = session.perks().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn hentai_at_home() {
    let session = get_session();

    let toplist = session.hentai_at_home().await.unwrap();
    println!("{:#?}", toplist);
}
#[tokio::test]
async fn credit_log() {
    let session = get_session();

    let toplist = session.credit_logs().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn karma_log() {
    let session = get_session();

    let toplist = session.karma_logs().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn torrents() {
    let session = get_session();

    let toplist = session
        .torrents(
            "",
            1,
            torrents::TorrentStatus::Unseeded,
            None,
            torrents::Sort::Size,
            false,
        )
        .await
        .unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn folder_info() {
    let session = get_session();

    let toplist = session.folder_info().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn folder_delete() {
    let session = get_session();
    //TODO: not working
    let toplist = session.delete_folder(3).await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn folder_create() {
    let session = get_session();
    //TODO: not working
    let toplist = session.create_folder("abcd").await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn list_upload() {
    let session = get_session();

    let toplist = session.list_upload().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn mpv() {
    let session = get_session();
    let toplist = session.mpv_info(3729428, "a2a9ceba5a").await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn mpv_bypass() {
    let session = get_session();

    let toplist = session.mpv_info_bypass(530350, "8b3c7e4a21").await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn stats() {
    let session = get_session();

    let toplist = session.stats(None).await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn buy_hath() {
    let session = get_session();

    let toplist = session.exhange_bid(true, 1, 1).await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn ask_hath() {
    let session = get_session();

    let toplist = session.exhange_ask(false, 1, 500).await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn home() {
    let session = get_session();

    let toplist = session.home().await.unwrap();
    println!("{:#?}", toplist);
}

#[tokio::test]
async fn bounty_info() {
    let session = get_session();

    let toplist = session.bounty_info(25071).await.unwrap();
    println!("{:#?}", toplist);
}
use std::io::Write;

#[test]
fn aaaa() {
    let file = |file_path| {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)
            .unwrap()
    };
    let mut ids_f = file("ids");
    let mut ratings_f = file("ratings");
    let mut favs_f = file("favs");
    for file in read_dir("searches").unwrap() {
        let file = read_to_string(file.unwrap().path()).unwrap();
        let data: Search = serde_json::from_str(&file).unwrap();
        let (ids, ratings, favs) = format_data(data);

        writeln!(&mut ids_f, "\n{}", ids).unwrap();
        if !ratings.is_empty() {
            writeln!(&mut ratings_f, "\n{}", ratings).unwrap();
        }
        if !favs.is_empty() {
            writeln!(&mut favs_f, "\n{}", favs).unwrap();
        }
    }
}
