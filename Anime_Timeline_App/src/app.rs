use crate::api::models::AnimeItem;
use iced::widget::{button, column, row, text, text_input, Column};
use iced::{Element, Task, Theme};
use crate::api::jikan;


#[derive(Default)]
pub struct AnimeTimeline{
    search_text:String,
    search_result:Vec<AnimeItem>,
    is_loading:bool,
}

#[derive(Debug, Clone)]
pub enum Message{
    SearchTextChanged(String),
    SearchRequested,
    SearchResLoaded(Result<Vec<AnimeItem>, String>),
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
      }
    }
    pub fn view(&self) -> Element<'_,Message>{
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
            Column::with_children(self.search_result.iter().map(|anime| text(&anime.title).size(24).into())).spacing(10).into()
        };
        column![
            search_bar,
            content
        ].padding(20).spacing(20).into()
    }
}