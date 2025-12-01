mod api;
mod app;
use iced::{Settings,Theme};
use app::AnimeTimeline;

fn main() -> iced::Result {
    iced::application("Anime Timeline Explorer", AnimeTimeline::update, AnimeTimeline::view)
        .theme(|_| Theme::Dark)
        .run()
}


