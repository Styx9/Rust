use reqwest::Error;
use crate::api::models::{AnimeItem, AnimeResponse, AnimeType, EpisodeItem, EpisodeResponse, EpisodeSingleResponse, SortMode};

pub async fn search_anime(query:String,genre:Option<u32>,types: Option<AnimeType>,sort: SortMode) -> Result<Vec<AnimeItem>,Error>{
    let mut params: Vec<(&str,String)> = vec![("q",query)];
    if let Some(g) = genre {
        params.push(("genres",g.to_string()));
    } 
    if let Some(t) = types {
        params.push(("type",t.query_val().to_string()));
    }
    if sort != SortMode::Default{
    params.push(("order_by",sort.order_by().to_string()));
    params.push(("sort",sort.direction().to_string()));}
    let client = reqwest::Client::new();

    let response = client.get("https://api.jikan.moe/v4/anime").query(&params).send().await?.json::<AnimeResponse>().await?;
    Ok(response.data)
}
pub async fn get_episode_detail(anime_id:u32,ep_nr:u32) ->Result<EpisodeItem, Error>
{
    let url = format!("https://api.jikan.moe/v4/anime/{}/episodes/{}",anime_id,ep_nr);
    let response = reqwest::get(url).await?.json::<EpisodeSingleResponse>().await?;
    Ok(response.data)
}
pub async fn get_episodes(id:u32) -> Result<Vec<EpisodeItem>,Error>{
    let url = format!("https://api.jikan.moe/v4/anime/{}/episodes",id);
    let response = reqwest::get(url).await?.json::<EpisodeResponse>().await?;
    Ok(response.data)
}
pub async fn get_image(url: String) -> Result<Vec<u8>, String> {
    let bytes = reqwest::get(url).await.map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;
    Ok(bytes.to_vec())
}