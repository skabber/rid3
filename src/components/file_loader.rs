use web_sys::Event;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileLoaderProps {
    pub on_file_change: Callback<Event>,
}

#[function_component(FileLoader)]
pub fn file_loader(FileLoaderProps { on_file_change }: &FileLoaderProps) -> Html {
    html!(
        <div class="file">
            <label class="file-label">
              <input class="file-input" type="file" name="resume" accept="audio/mp3,audio/*" onchange={on_file_change} multiple=false/>
              <span class="file-cta">
                <span class="file-icon">
                  <i class="fas fa-upload"></i>
                </span>
                <span class="file-label">
                  {"Choose a file…"}
                </span>
              </span>
            </label>
          </div>
    )
}
