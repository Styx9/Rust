use crate::api::models::{AnimeItem,EpisodeItem,AnimeType,SortMode,Genre,BASIC_GENRES};
use iced::widget::shader::wgpu::QuerySet;
use iced::widget::{Column, Container, button,pick_list, column, container, image, row, scrollable, text, text_input};
use iced::{Element, Task, Theme,Length,Alignment};
use iced::border::Border;
use iced::widget::image::Handle;
use iced::widget::scrollable::Direction;
use reqwest::header;
use std::collections::HashMap;
use crate::api::jikan;

#[derive(Debug, Clone)]
pub enum Screen{
    Search,
    Detail(AnimeItem,Vec<EpisodeItem>,Option<Handle>),
    SearchOverlay,
}

pub struct AnimeTimeline{
    search_text:String,
    search_result:Vec<AnimeItem>,
    is_loading:bool,
    current_screen: Screen,
    favorites: Vec<AnimeItem>,
    fav_thumbs: HashMap<u32,Handle>, //key este mal_id
    selected_genre: Option<Genre>,
    selected_type: Option<AnimeType>,
    selected_sort: SortMode,
}
impl Default for AnimeTimeline{
    fn default() -> Self{
        Self{
            search_text: String::new(),
            search_result: Vec::new(),
            is_loading: false,
            current_screen: Screen::Search,
            favorites: crate::storage::load_fav(),
            fav_thumbs: HashMap::new(),
            selected_genre: None,
            selected_type: None,
            selected_sort: SortMode::ScoreDesc,
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
    BackToSearch,
    ImageLoaded(Result<Handle,String>),//pt incarcarea imaginii
    ToggleFavorite(AnimeItem), //pt a da "inima" unei serii
    ClearSearch,
    LoadFavThumbs,
    FavThumbLoaded{mal_id:u32,result: Result<Handle,String>},
   GenreChanged(Genre),
    TypeChanged(AnimeType),
    SortChanged(SortMode),
    ClearFilters,
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
            self.current_screen = Screen::Detail(anime.clone(), Vec::new(),None);
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
        Message::EpisodesLoaded(Ok(episodes)) =>{
            if let Screen::Detail(anime,_,img_hndl) = &self.current_screen{
                self.current_screen = Screen::Detail(anime.clone(),episodes, img_hndl.clone())
            } // daca ni sa incarcat imaginea inainte facem update la screen dar pastram aceeasi imagine
            self.is_loading = false;
            Task::none()
        }
        Message::EpisodesLoaded(Err(e)) =>{
            eprintln!("Episode Error: {}", e);
            self.is_loading = false;
            Task::none()
        }
        Message::BackToSearch => {
            self.current_screen = Screen::Search;
            Task::done(Message::LoadFavThumbs)
        }
        Message::ImageLoaded(Ok(handle)) => {
            if let Screen::Detail(anime, episodes, _) = &self.current_screen{
                self.current_screen = Screen::Detail(anime.clone(), episodes.clone(), Some(handle));
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
            return Task::batch(tasks);
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
            self.selected_sort = SortMode::ScoreDesc;

            let query = self.search_text.trim().to_string();
            if query.is_empty(){
                Task::none()
            }
            else {
                Task::done(Message::SearchRequested)
            }
        }
      }
    }
    pub fn view(&self) -> Element<'_, Message>{
        match &self.current_screen{
            Screen::Search => self.view_search(),
            Screen::Detail(anime,episodes,img_handle ) => self.view_detail(anime,episodes, img_handle),
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

    fn view_detail(&self, anime:&AnimeItem, episodes: &Vec<EpisodeItem>, img_handle:&Option<Handle>) -> Element<'_,Message>{
        let is_fav = self.favorites.iter().any(|f| f.mal_id == anime.mal_id);
        let fav_btn = if is_fav{
            button("Unfavorite").on_press(Message::ToggleFavorite((anime.clone())))
        }
        else {
            button("Favorite").on_press(Message::ToggleFavorite((anime.clone())))
        };
        let img_element: Element<Message> = match img_handle{
            Some(handle) => image(handle.clone()).width(iced::Length::Fixed((200.0))).height(iced::Length::Fixed(300.0)).into(),
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
                    text(format!("{}. {} - {}",i+1,date_display,ep.title)).size(16).into()
                })
            ).spacing(5)).height(iced::Length::Fill).into()
        };
        column![
            button("<- Back").on_press(Message::BackToSearch),
            header_row,
            text("Episodes:").size(22),//Header
            episode_list
        ].padding(20).spacing(20).into()
    }
}