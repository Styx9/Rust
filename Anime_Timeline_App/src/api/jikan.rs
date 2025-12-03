use reqwest::Error;
use crate::api::models::{AnimeResponse, AnimeItem,EpisodeItem,EpisodeResponse};

pub async fn search_anime(query:String) -> Result<Vec<AnimeItem>,Error>{
    let url = format!("https://api.jikan.moe/v4/anime?q={}",query);
    let response = reqwest::get(url).await?.json::<AnimeResponse>().await?;
    Ok(response.data)
}
pub async fn get_episodes(id:u32) -> Result<Vec<EpisodeItem>,Error>{
    let url = format!("https://api.jikan.moe/v4/anime/{}/episodes",id);
    let response = reqwest::get(url).await?.json::<EpisodeResponse>().await?;
    Ok(response.data)
}