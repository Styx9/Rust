use std::fs;
use crate::api::models::AnimeItem;
use std::collections::HashMap;
const F_NAME: &str = "favorites.json";
const WF_NAME: &str = "watched.json";

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
pub fn save_watched(watched: &HashMap<u32,Vec<u32>>){
    let json_txt  = match serde_json::to_string_pretty(watched){
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
pub fn load_watched() ->HashMap<u32,Vec<u32>>{
    match fs::read_to_string(WF_NAME){
        Ok(json_text) => match serde_json::from_str::<HashMap<u32,Vec<u32>>>(&json_text){
            Ok(map) =>map,
            Err(e) =>{
                eprint!("Error parsing watched.json {}",e);
                HashMap::new()
            }
        },
        Err(_) => HashMap::new(),
    }
}