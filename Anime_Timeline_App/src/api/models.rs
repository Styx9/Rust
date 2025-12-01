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
}
#[derive(Debug,Clone, Deserialize)]
pub struct Images{
    pub jpg: ImageFormats,
}
#[derive(Debug,Clone, Deserialize)]
pub struct ImageFormats{
    pub image_url: String,
}
