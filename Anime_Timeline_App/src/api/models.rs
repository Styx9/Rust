#![allow(clippy::to_string_trait_impl)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]

pub struct AnimeResponse{
    pub data: Vec<AnimeItem>,
}


#[derive(Debug,Clone, Deserialize,Serialize)]
pub struct AnimeItem{
    pub mal_id: u32,
    pub title: String,
    pub images: Images,
    pub synopsis: Option<String>,
    pub score: Option<f64>,
    pub aired : Aired //jikan n-il da intrun obj
}
#[derive(Debug, Deserialize, Clone,Serialize)]
pub struct Aired{
    pub string: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct EpisodeResponse{
    pub data: Vec<EpisodeItem>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct EpisodeSingleResponse{
    pub data: EpisodeItem,  //special ca sa putem adauga synopsis
}
#[derive(Debug, Deserialize, Clone)]
pub struct EpisodeItem{
    pub mal_id: u32,
    pub title: String,
    pub aired: Option<String>,
    pub synopsis: Option<String>,
}
#[derive(Debug,Clone, Deserialize,Serialize)]
pub struct Images{
    pub jpg: ImageFormats,
}
#[derive(Debug,Clone, Deserialize,Serialize)]
pub struct ImageFormats{
    pub image_url: String,
}
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum AnimeType {
    Tv,Movie,Ova,Special,
}
impl AnimeType{
    pub const ALL: [AnimeType; 4] = [
        AnimeType::Tv,
        AnimeType::Movie,
        AnimeType::Ova,
        AnimeType::Special,
    ];
    pub fn query_val(&self) -> &'static str{ //pt query in api
        match self{
            AnimeType::Tv => "tv",
            AnimeType::Movie => "movie",
            AnimeType::Ova => "ova",
            AnimeType::Special => "special",
        }
    }
}
impl ToString for AnimeType {
    fn to_string(&self) -> String{
        match self{
            Self::Tv => "TV",
            Self::Movie => "Movie",
            Self::Ova => "Ova",
            Self::Special => "Special",
        }.to_string()
    }
}
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum SortMode{
    Default,
    ScoreDesc,
    PopularityAsc,
}
impl SortMode{
    pub const ALL: [SortMode; 3] = [
        SortMode::ScoreDesc,
        SortMode::PopularityAsc,
        SortMode::Default,
    ];

        pub fn order_by(&self) -> &'static str {
        match self {
            SortMode::ScoreDesc => "score",
            SortMode::PopularityAsc => "popularity",
            SortMode::Default => "",
        }
    }

    pub fn direction(&self) -> &'static str {
        match self {
            SortMode::ScoreDesc => "desc",
            SortMode::PopularityAsc => "asc",
            SortMode::Default => "",
        }
    }
}
    impl ToString for SortMode{
        fn to_string(&self) -> String{
            match self{
                Self::ScoreDesc => "Score (high -> low)",
                Self::PopularityAsc => "Popularity (best->worst)",
                Self::Default => "Default",
            }.to_string()
        }
    }
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub struct Genre{
    pub id:u32,
    pub name: &'static str,
}
impl ToString for Genre {
    fn to_string(&self) -> String{
        self.name.to_string()
    }
}
pub const BASIC_GENRES: [Genre; 10] = [
    Genre { id: 1,  name: "Action" },
    Genre { id: 2,  name: "Adventure" },
    Genre { id: 4,  name: "Comedy" },
    Genre { id: 8,  name: "Drama" },
    Genre { id: 10, name: "Fantasy" },
    Genre { id: 14, name: "Horror" },
    Genre { id: 22, name: "Romance" },
    Genre { id: 24, name: "Sci-Fi" },
    Genre { id: 30, name: "Sports" },
    Genre { id: 36, name: "Slice of Life" },
];
