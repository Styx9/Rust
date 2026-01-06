use crate::api::models::{AnimeItem,EpisodeItem,AnimeType,SortMode,Genre,BASIC_GENRES};
use iced::widget::{Column, Container, button,pick_list, column, checkbox, image, row, scrollable, text, text_input};
use iced::{Element, Task,Length,Alignment};
use iced::widget::image::Handle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::api::jikan;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct AnimePreview{
    mal_id:u32,
    title:String,
    image_url:String,
}

#[derive(Debug, Clone)]
pub enum Screen{
    Search,
    Detail(Box<DetailScreen>),
    SearchOverlay,
}
#[derive(Debug, Clone)]
pub struct DetailScreen {
    pub anime: AnimeItem,
    pub episodes: Vec<EpisodeItem>,
    pub img: Option<Handle>,
    pub back_to: Box<Screen>,
}

pub struct AnimeTimeline{
    search_text:String,
    search_result:Vec<AnimeItem>,
    is_loading:bool,
    loading_episode: Option<u32>,
    current_screen: Screen,
    favorites: Vec<AnimeItem>,
    fav_thumbs: HashMap<u32,Handle>, //key este mal_id
    selected_genre: Option<Genre>,
    selected_type: Option<AnimeType>,
    selected_sort: SortMode,
    watched: HashMap<u32,Vec<u32>>,
    recent_watch: Vec<u32>,
    recent_info: HashMap<u32,AnimePreview>
}
impl Default for AnimeTimeline{
    fn default() -> Self{
        let (watched_map,recent_vec,recent_info_map) = crate::storage::load_watched();
        Self{
            search_text: String::new(),
            search_result: Vec::new(),
            is_loading: false,
            current_screen: Screen::Search,
            favorites: crate::storage::load_fav(),
            fav_thumbs: HashMap::new(),
            selected_genre: None,
            selected_type: None,
            selected_sort: SortMode::Default,
            loading_episode:None,
            watched: watched_map,
            recent_watch: recent_vec,
            recent_info: recent_info_map,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message{
    //Pt partea de search
    SearchTextChanged(String),
    SearchRequested,
    SearchResLoaded(Result<Vec<AnimeItem>, String>),
    //Pt Navigatie
    AnimeSelected(AnimeItem),//cand facem click
    EpisodesLoaded(Result<Vec<EpisodeItem>,String>),
    Back,
    ImageLoaded(Result<Handle,String>),//pt incarcarea imaginii
    ToggleFavorite(AnimeItem), //pt a da "inima" unei serii
    ClearSearch,
    LoadFavThumbs,
    FavThumbLoaded{mal_id:u32,result: Result<Handle,String>},
    LoadRecentThumbs,
   GenreChanged(Genre),
    TypeChanged(AnimeType),
    SortChanged(SortMode),
    ClearFilters,
    EpisodeClicked(u32),
    EpisodeDetailLoaded{ep_nr:u32, result:Result<EpisodeItem,String>},
    ToggleWatched{ anime_id: u32, ep_nr:u32},
}
impl AnimeTimeline{
   
    pub fn update(&mut self, message: Message) -> Task<Message> {
      match message {
        Message::SearchTextChanged(text) =>{
            self.search_text = text;
            Task::none()
        }
        Message::SearchRequested => {
            self.current_screen = Screen::SearchOverlay;
            self.is_loading = true;
            let query = self.search_text.clone();
             if query.is_empty() {
            self.search_result.clear();
            self.is_loading = false;
            return Task::none();
            }
            
            let genre = self.selected_genre.map(|g| g.id);
            let types = self.selected_type;
            let sort = self.selected_sort;
            Task::perform(
            async move{
                jikan::search_anime(query,genre,types,sort).await.map_err(|e| e.to_string())
            },
                Message::SearchResLoaded,
            )
        }
        Message::SearchResLoaded(Ok(results)) => {
            self.search_result = results;
            self.is_loading = false;
            Task::none()
        }
        Message::SearchResLoaded(Err(e)) => {
            eprintln!("Error fetching anime data: {}", e);
            self.search_result.clear();
            self.is_loading = false;
            Task::none()
        }
        Message::AnimeSelected(anime)=>{
            let prev = self.current_screen.clone();
            self.current_screen = Screen::Detail(Box::new(DetailScreen{anime: anime.clone(), episodes: Vec::new(),img:None,back_to: Box::new(prev),}));
            self.is_loading = true;

            let id = anime.mal_id;
            let img_url = anime.images.jpg.image_url.clone();
            Task::batch(vec![
                    Task::perform(async move {
                        jikan::get_episodes(id).await.map_err(|e| e.to_string()) },
                        Message::EpisodesLoaded
                    ),
                    Task::perform(async move {
                        let bytes = jikan::get_image(img_url).await?;
                        Ok(Handle::from_bytes(bytes))
                    },
                    Message::ImageLoaded
                )
            ])
        }
       Message::EpisodesLoaded(Ok(episodes)) => {
    if let Screen::Detail(detail) = &self.current_screen {
        self.current_screen = Screen::Detail(Box::new(DetailScreen {
            anime: detail.anime.clone(),
            episodes,
            img: detail.img.clone(),
            back_to: detail.back_to.clone(),
        }));
    } // daca ni sa incarcat imaginea inainte facem update la screen dar pastram aceeasi imagine
    self.is_loading = false;
    Task::none()
}

        Message::EpisodesLoaded(Err(e)) =>{
            eprintln!("Episode Error: {}", e);
            self.is_loading = false;
            Task::none()
        }
        Message::Back => {
            if let Screen::Detail(detail) = &self.current_screen {
                self.current_screen = *detail.back_to.clone();
                if let Screen::Search = self.current_screen{
                    return Task::done(Message::LoadFavThumbs);
                }
                return Task::none();
            }
            Task::none()
        }
        Message::ImageLoaded(Ok(handle)) => {
        if let Screen::Detail(detail) = &self.current_screen {
            self.current_screen = Screen::Detail(Box::new(DetailScreen {
                anime: detail.anime.clone(),
                episodes: detail.episodes.clone(),
                img: Some(handle),
                back_to: detail.back_to.clone(),
            }));
            }
            Task::none()
        }
        Message::ImageLoaded(Err(e)) =>{
            eprintln!("Image failed to load: {}",e);
            Task::none()
        }
        Message::ToggleFavorite(anime)=>{
            if let Some(index) = self.favorites.iter().position(|x| x.mal_id == anime.mal_id){ //daca avem deja animeul in favorite si mai apasam odata pe inima acesta va fi eliminat
                self.favorites.remove(index);
            }
            else{
                self.favorites.push(anime);
            }
            crate::storage::save_fav(&self.favorites);
            Task::done(Message::LoadFavThumbs)
        }
        Message::ClearSearch => {
            self.search_text = String::new();
            self.search_result.clear();
            self.current_screen =Screen::Search;
            Task::none()
        }
        Message::LoadFavThumbs =>{
            let mut tasks = Vec::new();

            for anime in &self.favorites{
                let id = anime.mal_id;
                if self.fav_thumbs.contains_key(&id){
                    continue;
                }
                let url = anime.images.jpg.image_url.clone();
                tasks.push(Task::perform(
                    async move{
                        let bytes = jikan::get_image(url).await.map_err(|e| e.to_string())?;
                        Ok(Handle::from_bytes(bytes))
                    },
                    move |res| Message::FavThumbLoaded{ mal_id : id, result: res},
                ));
            }
            Task::batch(tasks)
        }
        Message::LoadRecentThumbs =>{
            let mut tasks = Vec::new();
            for &anime_id in &self.recent_watch{
                if self.fav_thumbs.contains_key(&anime_id){
                    continue;
                }
                let preview = match self.recent_info.get(&anime_id){
                    Some(p) => p,
                    None => continue,
                };
                let url = preview.image_url.clone();
                tasks.push(Task::perform(
                    async move {
                        let bytes = jikan::get_image(url).await.map_err(|e| e.to_string())?;
                        Ok(Handle::from_bytes(bytes))
                    },
                    move |res| Message::FavThumbLoaded{ mal_id: anime_id, result: res}
                ));
            }
            Task::batch(tasks)
        }
        Message::FavThumbLoaded { mal_id, result } =>{
            if let Ok(handle) = result{
                self.fav_thumbs.insert(mal_id,handle );
            }
            Task::none()
        }
        Message::TypeChanged(t) => {
            self.selected_type = Some(t);
            let query = self.search_text.trim().to_string();
            if query.is_empty(){
                return Task::none();
            }
            self.is_loading = true;

            Task::done(Message::SearchRequested)
        }
        Message::GenreChanged(g) =>{
            self.selected_genre = Some(g);
            let query = self.search_text.trim().to_string();
            if query.is_empty(){
                return Task::none();
            }
            self.is_loading = true;
            Task::done(Message::SearchRequested)
        }
        Message::SortChanged(s) => {
            self.selected_sort = s;
            let query = self.search_text.trim().to_string();
            if query.is_empty(){
                return Task::none();
            }
            self.is_loading = true;

            Task::done(Message::SearchRequested)
        }
        Message::ClearFilters =>{
            self.selected_genre = None;
            self.selected_type = None;
            self.selected_sort = SortMode::Default;

            let query = self.search_text.trim().to_string();
            if query.is_empty(){
                Task::none()
            }
            else {
                Task::done(Message::SearchRequested)
            }
        }
       Message::EpisodeClicked(ep_nr) =>{
        let anime_id = match &self.current_screen{
            Screen::Detail(detail) => detail.anime.mal_id,
            _ => return Task::none(), //daca nu suntem in detail nu trb sa facem nimic
        };
        if let Screen::Detail(detail) = &self.current_screen
            && let Some(ep) = detail.episodes.iter().find(|e| e.mal_id == ep_nr)
            && ep.synopsis.is_some(){
                return Task::none(); //daca am avut deja synopisul incarcat nu trb sa facem refetch
            }
        self.loading_episode = Some(ep_nr);
        Task::perform(
            async move {
                jikan::get_episode_detail(anime_id,ep_nr).await.map_err(|e| e.to_string())
            },
            move |res| Message::EpisodeDetailLoaded { ep_nr, result: res },
        )
    }
        Message::EpisodeDetailLoaded { ep_nr, result } =>{
            self.loading_episode = None;

            if let Ok(detail) = result && 
            let Screen::Detail(detail_screen) = &mut self.current_screen{
                if let Some(ep) = detail_screen.episodes.iter_mut().find(|e| e.mal_id == ep_nr){
                    ep.synopsis = detail.synopsis;
                }
                self.current_screen = Screen::Detail(Box::new(DetailScreen{
                    anime: detail_screen.anime.clone(),
                    episodes: detail_screen.episodes.clone(),
                    img: detail_screen.img.clone(),
                    back_to: detail_screen.back_to.clone(),
                }));
}
            Task::none()
        }
        Message::ToggleWatched { anime_id, ep_nr } =>{
            let list = self.watched.entry(anime_id).or_default();
            let before = list.len();
            list.retain(|&x| x!= ep_nr);
            if list.len() == before {
                list.push(ep_nr);
                if let Some(x) = self.recent_watch.iter().position(|&x| x == anime_id) {self.recent_watch.remove(x);}
                self.recent_watch.insert(0, anime_id);
            }
            let max_recent = 15;
            if self.recent_watch.len() > max_recent{
                self.recent_watch.truncate(max_recent);
            }
                if let Screen::Detail(detail)  = &self.current_screen 
                   && detail.anime.mal_id == anime_id{
                        let preview = AnimePreview{
                            mal_id: anime_id,
                            title: detail.anime.title.clone(),
                            image_url: detail.anime.images.jpg.image_url.clone(),
                        };
                        self.recent_info.insert(anime_id, preview);
                    
                }
            let remove_key = match self.watched.get(&anime_id){ //daca am dat untoggle la toate episoadele stergem si cheia
                Some(v) => v.is_empty(),
                None => false,
            };
            if remove_key{
                self.watched.remove(&anime_id);
                if let Some(x) = self.recent_watch.iter().position(|&x| x == anime_id){
                    self.recent_watch.remove(x);
                }
                self.recent_info.remove(&anime_id);
            }
            crate::storage::save_watched(&self.watched,&self.recent_watch,&self.recent_info);
            Task::done(Message::LoadRecentThumbs)
        }
      }
    }
    pub fn view(&self) -> Element<'_, Message>{
        match &self.current_screen{
            Screen::Search => self.view_search(),
            Screen::Detail(detail) => self.view_detail(&detail.anime, &detail.episodes, &detail.img),
            Screen::SearchOverlay=> self.view_search_overlay(),
        }
    }
    pub fn view_search(&self) -> Element<'_,Message>{
        let search_bar = row![
            text_input(
                "Search anime...",
                &self.search_text,
            ).on_input(Message::SearchTextChanged).on_submit(Message::SearchRequested),
            button("Search").on_press(Message::SearchRequested),
            if !self.search_text.is_empty() || !self.search_result.is_empty() {
                button("Clear X").on_press(Message::ClearSearch).style(button::danger)
            }
            else{
                button("").width(0).padding(0)
            }
        ].spacing(10).align_y(Alignment::Center);
        column![
            search_bar,
            self.view_fav(),
            self.view_watched_recent(),
        ].padding(20).spacing(20).into()
    }
  fn view_fav(&self) -> Element<'_, Message> {
    use iced::widget::{button, column, container, row, scrollable, text, Container};
    use iced::widget::scrollable::Direction;
    use iced::{Alignment, Length};

    if self.favorites.is_empty() {
        return text("No favorites yet")
            .size(14)
            .style(text::secondary)
            .into();
    }

    let fav_row = row(self.favorites.iter().map(|anime| {
        let thumb: Element<Message> = if let Some(handle) = self.fav_thumbs.get(&anime.mal_id){
            image(handle.clone()).width(Length::Fixed(100.0)).height(Length::Fixed(140.0)).into()
        }
        else {
        Container::new(text("IMG").size(10))
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(140.0))
            .style(|_| container::Style {
                background: Some(iced::Color::from_rgb(1.0, 0.0, 1.0).into()),
                border: iced::Border {
                    color: iced::Color::from_rgb(1.0, 0.0, 0.0),
                    width: 3.0,
                    radius: 5.0.into(),
                },
                text_color: Some(iced::Color::WHITE),
                ..Default::default()
            }).into()};
        let card_content = column![
            thumb,
            text(&anime.title)
                .size(12)
                .width(Length::Fixed(100.0))
                .wrapping(text::Wrapping::Word)
                .align_x(Alignment::Center),
        ]
        .spacing(5)
        .align_x(Alignment::Center);

        button(card_content)
            .padding(5)
            .on_press(Message::AnimeSelected(anime.clone()))
            .into()
    }))
    .spacing(15)
    .padding([0, 10]); 

    column![
        text(format!("Favorites ({})", self.favorites.len()))
            .size(20)
            .font(iced::font::Font::MONOSPACE),

        scrollable(fav_row)
            .direction(Direction::Horizontal(scrollable::Scrollbar::default()))
            .height(Length::Fixed(230.0))
            .width(Length::Fill),
    ]
    .spacing(10)
    .into()
}
 fn view_watched_recent(&self) -> Element<'_,Message>
 {
    use iced::widget::{button, column, container, row, scrollable, text, Container};
    use iced::widget::scrollable::Direction;
    use iced::{Alignment, Length};

    if self.recent_watch.is_empty() {
        return text("No recent watched yet")
            .size(14)
            .style(text::secondary)
            .into();
    }

    let watched_row = row(self.recent_watch.iter().map(|&id| {
        let preview = match self.recent_info.get(&id) {
            Some(p) => p,
            None => {
                return Container::new(text("Missing info")).width(Length::Fixed(75.0)).into();
            }
        };
        let thumb: Element<Message> = if let Some(handle) = self.fav_thumbs.get(&id){
            image(handle.clone()).width(Length::Fixed(100.0)).height(Length::Fixed(140.0)).into()
        }
        else {
        Container::new(text("IMG").size(10))
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(140.0))
            .style(|_| container::Style {
                background: Some(iced::Color::from_rgb(1.0, 0.0, 1.0).into()),
                border: iced::Border {
                    color: iced::Color::from_rgb(1.0, 0.0, 0.0),
                    width: 3.0,
                    radius: 5.0.into(),
                },
                text_color: Some(iced::Color::WHITE),
                ..Default::default()
            }).into()};
        let card_content = column![
            thumb,
            text(&preview.title)
                .size(12)
                .width(Length::Fixed(100.0))
                .wrapping(text::Wrapping::Word)
                .align_x(Alignment::Center),
        ]
        .spacing(5)
        .align_x(Alignment::Center);
            if let Some(anime) = self.favorites.iter().find(|a| a.mal_id == id){
                button(card_content)
            .padding(5)
            .on_press(Message::AnimeSelected(anime.clone()))
            .into()
            }
            else {
                button(card_content).padding(5).into()
            }
    }))
    .spacing(15)
    .padding([0, 10]); 

    column![
        text(format!("Recently watched:"))
            .size(20)
            .font(iced::font::Font::MONOSPACE),

        scrollable(watched_row)
            .direction(Direction::Horizontal(scrollable::Scrollbar::default()))
            .height(Length::Fixed(230.0))
            .width(Length::Fill),
    ]
    .spacing(10)
    .into()
 }
    fn view_search_overlay(&self) -> Element<'_,Message>
    {
         let search_bar = row![
            text_input(
                "Search anime...",
                &self.search_text,
            ).on_input(Message::SearchTextChanged).on_submit(Message::SearchRequested),
            button("Search").on_press(Message::SearchRequested),
            if !self.search_text.is_empty() || !self.search_result.is_empty() {
                button("Clear X").on_press(Message::ClearSearch).style(button::danger)
            }
            else{
                button("").width(0).padding(0)
            }
        ].spacing(10).align_y(Alignment::Center);
        let type_pick = pick_list(
            AnimeType::ALL.to_vec(),
            self.selected_type,
            Message::TypeChanged,
        ).placeholder("Type");
        let sort_pick = pick_list(
            SortMode::ALL.to_vec(),
            Some(self.selected_sort),
            Message::SortChanged,
        ).placeholder("Sort");
        let genre_pick = pick_list(
            BASIC_GENRES.to_vec(),
            self.selected_genre,
            Message::GenreChanged,
        );
        let filters = row![
            genre_pick,
            sort_pick,
            type_pick,
            button("Clear filters").on_press(Message::ClearFilters),
        ].spacing(10).align_y(Alignment::Center);
        let results_column:Element<Message> = if self.is_loading
        {
            text ("Loading...").size(20).into()
        } else { Column::with_children(
            self.search_result.iter().map(|anime|{
                button(text(&anime.title).size(18)).padding(10).on_press(Message::AnimeSelected(anime.clone())).width(iced::Length::Fill).into()   
            })
        )
        .spacing(5).into()};
        let results_area :Element<Message> = scrollable(results_column).height(Length::Fill).into();
        column![
            search_bar,
            filters,
            results_area,
        ].padding(20).spacing(20).into()
    }

    fn view_detail(&self, anime:&AnimeItem, episodes: &[EpisodeItem], img_handle:&Option<Handle>) -> Element<'_,Message>{
        let is_fav = self.favorites.iter().any(|f| f.mal_id == anime.mal_id);
        let fav_btn = if is_fav{
            button("Unfavorite").on_press(Message::ToggleFavorite(anime.clone()))
        }
        else {
            button("Favorite").on_press(Message::ToggleFavorite(anime.clone()))
        };
        let img_element: Element<Message> = match img_handle{
            Some(handle) => image(handle.clone()).width(iced::Length::Fixed(200.0)).height(iced::Length::Fixed(300.0)).into(),
            None => Container::new(text("Loading Image..."))
                .width(iced::Length::Fixed(200.0)).height(iced::Length::Fixed(300.0))
                .center_x(iced::Length::Fill).center_y(iced::Length::Fill).into(),
        };//aici facem imaginea
        let info_element = column![
            text(anime.title.clone()).size(30),
            match &anime.synopsis {
                Some(s) => text(s.clone()).size(14),
                None => text("No synopsis available.").size(14),
            },
            fav_btn,
            text(match anime.score{
                Some(score) => format!("Score: {:.2}",score),
                None => "Score:N/A".to_string(),
            }).size(16),
        ].spacing(10).width(iced::Length::Fill);
        let header_row = row![
            img_element,
            info_element,
        ].spacing(20);
        let episode_list:Element<Message> = if self.is_loading{
            text("Loading episodes...").into()
        }
        else{
           scrollable( Column::with_children(
                episodes.iter().enumerate().map(|(i,ep)|{
                    let date_display = match &ep.aired{
                        Some(date_str) =>{
                            match date_str.split('T').next(){
                                Some(good_date) => good_date,
                                None => date_str,
                            }
                        }
                        None => "Unknown date"
                    };
                    let ep_nr = (i as u32) + 1;
                    let anime_id = anime.mal_id;
                    let is_watched = match self.watched.get(&anime_id){
                        Some(list) => list.contains(&ep_nr),
                        None=>false,
                    };
                    let synopsis = if self.loading_episode == Some(ep_nr){
                        "Loading synopsis".to_string()
                    }
                    else{
                        match &ep.synopsis{
                            Some(s) => s.trim().replace('\n', " "),
                            None => String::new(),
                        }
                    };
                   row![
                    checkbox("",is_watched).on_toggle(move |_| {
                        Message::ToggleWatched {anime_id,ep_nr}
                    }),
                    button(
                column![
                    text(format!("{}. {} - {}", i + 1, date_display, ep.title)).size(16),
                    text(synopsis).size(14),
                    ].spacing(2)
                    ).on_press(Message::EpisodeClicked(ep_nr)).width(iced::Length::Fill).padding(6),
                   ].spacing(10).align_y(iced::Alignment::Center).into()
                })
            ).spacing(5)
        ).height(iced::Length::Fill).into()
    };
        column![
            button("<- Back").on_press(Message::Back),
            header_row,
            text("Episodes:").size(22),//Header
            episode_list
        ].padding(20).spacing(20).into()
    }
}