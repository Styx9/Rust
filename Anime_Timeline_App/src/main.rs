mod api;
mod app;
mod storage;
use iced::{Theme};
use app::AnimeTimeline;

fn main() -> iced::Result {
    iced::application("Anime Timeline Explorer", AnimeTimeline::update, AnimeTimeline::view)
        .theme(|_| Theme::Dark)
        .run()
}


