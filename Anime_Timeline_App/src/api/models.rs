use serde::Deserialize;
#[derive(Debug, Deserialize)]

pub struct AnimeResponse{
    pub data: Vec<AnimeItem>,
}


#[derive(Debug,Clone, Deserialize)]
pub struct AnimeItem{
    pub mal_id: u32,
    pub title: String,
    pub images: Images,
    pub synopsis: Option<String>,
    pub score: Option<f64>,
    pub aired : Aired //jikan n-il da intrun obj
}
#[derive(Debug, Deserialize, Clone)]
pub struct Aired{
    pub string: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct EpisodeResponse{
    pub data: Vec<EpisodeItem>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct EpisodeItem{
    pub mal_id: u32,
    pub title: String,
    pub aired: Option<String>,
}
#[derive(Debug,Clone, Deserialize)]
pub struct Images{
    pub jpg: ImageFormats,
}
#[derive(Debug,Clone, Deserialize)]
pub struct ImageFormats{
    pub image_url: String,
}
