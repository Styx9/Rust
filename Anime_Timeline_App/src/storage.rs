use std::{fs};
use serde::{Deserialize, Serialize};

use crate::{api::models::AnimeItem, app::AnimePreview};
use std::collections::HashMap;
const F_NAME: &str = "favorites.json";
const WF_NAME: &str = "watched.json";

#[derive(Debug,Deserialize,Serialize)]
struct Watched { 
  watched: HashMap<u32,Vec<u32>>,
  recent_watch: Vec<u32>,
  recent_info: HashMap<u32,AnimePreview>
}
type WatchedLoad = (
    HashMap<u32, Vec<u32>>,
    Vec<u32>,
    HashMap<u32, AnimePreview>,
);
pub fn save_fav(favorites: &Vec<AnimeItem>)
{
    let json_txt = match serde_json::to_string_pretty(favorites){
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error converting favorites to json: {}",e);
            return;
        }
    };
    match fs::write(F_NAME,json_txt){
        Ok(_) => println!("Favorites saved okay"),
        Err(e)=> eprintln!("Error writing to file: {}",e),
    }
}

pub fn load_fav() -> Vec<AnimeItem>{
    match fs::read_to_string(F_NAME){
        Ok(json_txt) =>{
            match serde_json::from_str::<Vec<AnimeItem>>(&json_txt) { //transformam json in binary data inapoi
                Ok(items) => items,
                Err(e) =>{
                    eprintln!("Error parsing favorites.json: {}",e);
                    Vec::new()
                }
            }
        }
        Err(_) => {
            Vec::new() //daca fisierul nu exista (il rulam pt prima oara) il creaza
        }
    }
}
pub fn save_watched(watched: &HashMap<u32,Vec<u32>>,recent:&[u32],recent_info: &HashMap<u32,AnimePreview>){
    let data = Watched { watched: watched.clone(), recent_watch: recent.to_owned(),recent_info: recent_info.clone()};
    let json_txt  = match serde_json::to_string_pretty(&data){
        Ok(text) =>text,
        Err(e) =>{
            eprint!("Error converting watched to json: {}",e);
            return;
        }
    };
    match fs::write(WF_NAME,json_txt){
        Ok(_) => println!("Watched saved okay"),
        Err(e)=> eprintln!("Error writing to file: {}",e),
    }
}
pub fn load_watched() ->WatchedLoad{
    match fs::read_to_string(WF_NAME){
        Ok(json_text) => match serde_json::from_str::<Watched>(&json_text){
            Ok(map) =>(map.watched,map.recent_watch,map.recent_info),
            Err(e) =>{
                eprint!("Error parsing watched.json {}",e);
                (HashMap::new(),Vec::new(),HashMap::new())
            }
        },
        Err(_) => (HashMap::new(),Vec::new(),HashMap::new()),
    }
}