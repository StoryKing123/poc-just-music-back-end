use std::{
    borrow::{Borrow, Cow},
    io::Error,
    result, thread,
    time::Duration,
};

use reqwest::header;

use serde::{Deserialize, Serialize};
// use tauri::api::process::{Command, CommandEvent};
use unm_engine_bilibili::{BilibiliEngine, ENGINE_ID as BILIBILI_ENGINE_ID};
use unm_engine_kugou::ENGINE_ID as KU_GOU_ENGINE_ID;
use unm_engine_kuwo::ENGINE_ID as KU_WO_ENGINE_ID;
use unm_engine_migu::ENGINE_ID as MIGU_ENGINE_ID;
use unm_engine_pyncm::ENGINE_ID as NET_EAST_ENGINE_ID;
use unm_engine_ytdl::ENGINE_ID as YTDL_ENGINE_ID;
// use unm_engine_qq::ENGINE_ID as QQ_ENGINE_ID;
use unm_api_utils::executor::build_full_executor;
use unm_types::{Artist, Context, ContextBuilder, SearchMode, Song};

pub async fn get_song_url(name: String, artist: String) -> Result<String, String> {
    let song = Song::builder()
        .name(name.to_string())
        .artists(vec![Artist::builder().name(artist.to_string()).build()])
        .build();
    // 34sjkhk

    let context = ContextBuilder::default()
        .enable_flac(true)
        .search_mode(SearchMode::OrderFirst)
        .build()
        .unwrap();

    let executor = unm_api_utils::executor::build_full_executor();
    let engines_to_use = std::env::var("ENGINES")
        .unwrap_or_else(|_| executor.list().join(" "))
        .split_whitespace()
        .map(|v| Cow::Owned(v.to_string()))
        .collect::<Vec<Cow<'static, str>>>();
    let engines_to_use = [
        std::borrow::Cow::Borrowed(MIGU_ENGINE_ID),
        std::borrow::Cow::Borrowed(NET_EAST_ENGINE_ID),
        std::borrow::Cow::Borrowed(BILIBILI_ENGINE_ID),
        std::borrow::Cow::Borrowed(KU_GOU_ENGINE_ID),
        std::borrow::Cow::Borrowed(YTDL_ENGINE_ID),
        // std::borrow::Cow::Borrowed(KU_WO_ENGINE_ID)
    ];

    let search_result = executor.search(&engines_to_use, &song, &context).await;

    let retrieved_result = executor.retrieve(&search_result.unwrap(), &context).await;

    let retrieved_result = retrieved_result.expect("can't be retrieved");

    println!(
        "[Retrieved] {} - {}: {} (from {})",
        artist, name, retrieved_result.url, retrieved_result.source
    );

    async fn handle_bilibili_url(url: &str) -> Result<String, reqwest::Error> {
        println!("parbili");
        let mut h = header::HeaderMap::new();
        h.insert(
            "Accept",
            header::HeaderValue::from_static("application/json"),
        );
        h.insert(
            "Referer",
            header::HeaderValue::from_static("https://www.bilibili.com/"),
        );
        h.insert(
            "User-Agent",
            header::HeaderValue::from_static("okhttp/3.4.1"),
        );
        // Ok(String::from("abc"))
        #[derive(Serialize, Deserialize, Debug)]
        struct Response {
            data: String,
        }
        let client = reqwest::Client::builder().default_headers(h).build()?;
        let song_res = client.get(url).send().await?.text().await?;
        let encoded_data = base64::encode(song_res);
        // println!("{:?}", song_res);
        // println!("end");
        // base64
        Ok(encoded_data)
    }

    if retrieved_result.url.contains("bilivideo.com") {
        let res = handle_bilibili_url(&retrieved_result.url).await;
        let result = match res {
            Ok(buffer) => String::from(buffer),
            Err(_) => String::from(""),
        };
        return Ok(result);
    }
    Ok(retrieved_result.url)
}
