use id3::Tag;
use yewdux::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Store)]
pub struct AppState {
    pub tag: Option<Tag>,
    pub name: String,
    pub bytes: Vec<u8>,
    pub url: String,
    pub seek_position: f64,
}
