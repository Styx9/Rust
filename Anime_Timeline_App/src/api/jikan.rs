use reqwest::Error;
use crate::api::models::{AnimeResponse, AnimeItem,EpisodeItem,EpisodeResponse,AnimeType,SortMode};

pub async fn search_anime(query:String,genre:Option<u32>,types: Option<AnimeType>,sort: SortMode) -> Result<Vec<AnimeItem>,Error>{
    let mut params: Vec<(&str,String)> = vec![("q",query)];
    if let Some(g) = genre {
        params.push(("genres",g.to_string()));
    } 
    if let Some(t) = types {
        params.push(("type",t.query_val().to_string()));
    }
    params.push(("order_by",sort.order_by().to_string()));
    params.push(("sort",sort.direction().to_string()));
    let client = reqwest::Client::new();

    let response = client.get("https://api.jikan.moe/v4/anime").query(&params).send().await?.json::<AnimeResponse>().await?;
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