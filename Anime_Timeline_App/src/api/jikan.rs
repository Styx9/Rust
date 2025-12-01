use serde::Deserialize;
use reqwest::Error;
use crate::api::models::{AnimeResponse, AnimeItem};

pub async fn search_anime(query:String) -> Result<Vec<AnimeItem>,Error>{
    let url = format!("https://api.jikan.moe/v4/anime?q={}",query);
    let response = reqwest::get(url).await?.json::<AnimeResponse>().await?;
    Ok(response.data)
}