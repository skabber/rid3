use yew::prelude::*;

mod components;
use components::FileLoader;

mod state;
use state::{AppAction, AppState};

use gloo::console::log;
use gloo_file::{callbacks::FileReader, File};
use web_sys::{Event, HtmlInputElement};

#[function_component]
fn App() -> Html {
    let state = use_reducer(|| AppState {
        mp3: None,
        tag: None,
        frames: Vec::new(),
        reader_tasks: None,
        name: String::new(),
    });

    let _tasks = use_state(Vec::<FileReader>::new);

    let state_closure = state.clone();
    let on_title_change = {
        Callback::from(move |e: Event| {
            let state = state_closure.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            let title = input.value();
            state.dispatch(AppAction::TitleChanged(title));
        })
    };

    let state_closure = state.clone();
    let on_file_change = {
        Callback::from(move |e: Event| {
            let state = state_closure.clone();
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

            for sf in selected_files {
                let state = state.clone();
                {
                    let state = state.clone();
                    let sd = state.clone();
                    let task = gloo_file::callbacks::read_as_bytes(&sf, move |bytes| {
                        let contents = bytes.unwrap();

                        state.dispatch(AppAction::MP3Ready(contents));
                    });

                    sd.dispatch(AppAction::AddReader(task));
                }
            }
        })
    };

    let s = state.clone();
    let save_clicked = Callback::from(move |_: MouseEvent| {
        log!("save clicked");
        log!(format!("{:?}", s.name));
    });

    html! {
        <div>
            <FileLoader tag={state.tag.clone()} on_file_change={on_file_change} on_title_change={on_title_change} save_clicked={save_clicked} name={state.name.clone()}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
