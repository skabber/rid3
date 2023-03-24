use yew::prelude::*;

mod components;
use components::{FileLoader, ID3Tag};

mod state;
use state::{AppAction, AppState};

use gloo::console::log;
use gloo_file::{callbacks::FileReader, File};
use web_sys::{Event, HtmlInputElement};
// use js_sys::Uint8Array;

#[function_component]
fn App() -> Html {
    let state = use_reducer(|| AppState {
        mp3: None,
        tag: None,
        frames: Vec::new(),
        reader_tasks: None,
        name: String::new(),
        bytes: Vec::new(),
        url: String::new(),
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
        let bytes = s.bytes.as_slice();
        let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(bytes) }.into());
        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
            &array,
            web_sys::BlobPropertyBag::new()
                .type_("audio/mpeg3;audio/x-mpeg-3;video/mpeg;video/x-mpeg;text/xml"),
        )
        .unwrap();
        let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap(); // Zero bytes

        // log!(format!("{:?}", download_url));
        // s.dispatch(AppAction::URLCreated(download_url));
        change_location(download_url.as_str());
    });

    let s= state.clone();
    let clear_clicked = Callback::from(move |_: MouseEvent| {
        log!("clear clicked");
        s.dispatch(AppAction::ClearClicked);
    });

    html! {
        <div class="columns">
            <div class="column has-background-link">
                <ID3Tag tag={state.tag.clone()} on_title_change={on_title_change} name={state.name.clone()} save_clicked={save_clicked} clear_clicked={clear_clicked}/>
                <div>{ state.url.clone() }</div>
            </div>
            <div class="column has-background-link-light">
                <FileLoader on_file_change={on_file_change} />
            </div>
        </div>
    }
}

fn change_location(url: &str) {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window
        .location()
        .set_href(url)
        .expect("location change failed");
}

fn alert(message: &str) {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window.alert_with_message(message).expect("alert failed");
}

fn main() {
    yew::Renderer::<App>::new().render();
}
