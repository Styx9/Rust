use crate::api::models::{AnimeItem,EpisodeItem};
use iced::widget::{button, column, row,scrollable, text, text_input, Column,Container};
use iced::{Element, Task, Theme};
use crate::api::jikan;

#[derive(Debug, Clone)]
pub enum Screen{
    Search,
    Detail(AnimeItem,Vec<EpisodeItem>),
}

pub struct AnimeTimeline{
    search_text:String,
    search_result:Vec<AnimeItem>,
    is_loading:bool,
    current_screen: Screen,
}
impl Default for AnimeTimeline{
    fn default() -> Self{
        Self{
            search_text: String::new(),
            search_result: Vec::new(),
            is_loading: false,
            current_screen: Screen::Search,
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
}
impl AnimeTimeline{
   
    pub fn update(&mut self, message: Message) -> Task<Message> {
      match message {
        Message::SearchTextChanged(text) =>{
            self.search_text = text;
            Task::none()
        }
        Message::SearchRequested => {
            self.is_loading = true;
            let query = self.search_text.clone();
            Task::perform(
            async move{
                jikan::search_anime(query).await.map_err(|e| e.to_string())
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
            self.current_screen = Screen::Detail(anime.clone(), Vec::new());
            self.is_loading = true;

            let id = anime.mal_id;
            Task::perform(
                    async move { jikan::get_episodes(id).await.map_err(|e| e.to_string()) },
                Message::EpisodesLoaded)
        }
        Message::EpisodesLoaded(Ok(episodes)) =>{
            self.is_loading = false;
            let new_screen = if let Screen::Detail(anime,_) = &self.current_screen{
                Some(Screen::Detail(anime.clone(),episodes))
            }
            else{
                None
            };
            if let Some(screen) = new_screen{
                self.current_screen = screen;
            }
            Task::none()
        }
        Message::EpisodesLoaded(Err(e)) =>{
            eprintln!("Episode Error: {}", e);
            self.is_loading = false;
            Task::none()
        }
        Message::BackToSearch => {
            self.current_screen = Screen::Search;
            Task::none()
        }
      }
    }
    pub fn view(&self) -> Element<'_, Message>{
        match &self.current_screen{
            Screen::Search => self.view_search(),
            Screen::Detail(anime,episodes ) => self.view_detail(anime,episodes),
        }
    }
    pub fn view_search(&self) -> Element<'_,Message>{
        let search_bar = row![
            text_input(
                "Search anime...",
                &self.search_text,
            ).on_input(Message::SearchTextChanged).on_submit(Message::SearchRequested),
            button("Search").on_press(Message::SearchRequested),
        ].spacing(10);

        let content: Element<Message> = if self.is_loading{
            text ("Loading...").size(20).into()
        }
        else{
            Column::with_children(
                self.search_result.iter().map(|anime|{
                    button(text(&anime.title).size(18)).padding(10).on_press(Message::AnimeSelected(anime.clone())).width(iced::Length::Fill).into()   
                })
            )
            .spacing(5).into()
        };
        column![
            search_bar,
            content
        ].padding(20).spacing(20).into()
    }
    fn view_detail(&self, anime:&AnimeItem, episodes: &Vec<EpisodeItem>) -> Element<'_,Message>{
        let header = column![
            button("<- Back").on_press(Message::BackToSearch),
            text(anime.title.clone()).size(30),
            match &anime.synopsis {
                Some(s) => text(s.clone()).size(14),
                None => text("No synopsis available.").size(14),
            },
            text(match anime.score{
                Some(score) => format!("Score: {:.2}",score),
                None => "Score:N/A".to_string(),
            }).size(16),
        ].spacing(10);
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
            header,
            episode_list
        ].padding(20).spacing(20).into()
    }
}