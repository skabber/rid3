use std::io::Cursor;

use gloo::console::log;
use gloo_file::File;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::state::AppState;

#[function_component(FileLoader)]
pub fn file_loader() -> Html {
    let reader = use_state(|| None);
    let (_, dispatch) = use_store::<AppState>();

    let on_file_change = {
        let reader = reader.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |e: Event| {
            let mut selected_files = Vec::new();
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                selected_files.extend(files);
            }

            for selected_file in selected_files {
                let reader_clone = reader.clone();
                let task = read_file(selected_file, dispatch.clone());
                reader_clone.set(Some(task));
            }
        })
    };

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

fn read_file(file: File, dispatch: Dispatch<AppState>) -> gloo_file::callbacks::FileReader {
    let file_name = file.name();
    gloo_file::callbacks::read_as_bytes(&file, move |bytes| {
        if let Ok(contents) = bytes {
            log!("contents length: ", contents.len());
            let data = Cursor::new(contents.as_slice());
            if let Ok(tag) = id3::Tag::read_from2(data) {
                dispatch.reduce(move |state| {
                    std::rc::Rc::new(AppState {
                        tag: Some(tag),
                        bytes: contents,
                        name: file_name.clone(),
                        url: state.url.clone(),
                        seek_position: state.seek_position.clone(),
                    })
                });
            }
        }
    })
}
