use web_sys::Event;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileLoaderProps {
    pub on_file_change: Callback<Event>,
}

#[function_component(FileLoader)]
pub fn file_loader(FileLoaderProps { on_file_change }: &FileLoaderProps) -> Html {
    html!(
        <div class="dropzone">
            <input type="file" accept="audio/mp3,audio/*" onchange={on_file_change} multiple=false />
            <span class="dropzone-icon">{"\u{266B}"}</span>
            <div class="dropzone-text">{"Drop an MP3 file here or click to browse"}</div>
            <div class="dropzone-hint">{"Supports MP3 files with ID3v2 tags"}</div>
        </div>
    )
}
