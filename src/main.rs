use yew::prelude::*;

mod components;
use components::{FileLoader, ID3Tag, MP3Audio};

mod state;
use state::{AppAction, AppState};

use gloo::console::log;
use gloo_file::File;
use id3::Version;
use std::io::Cursor;
use web_sys::{Event, HtmlInputElement};

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

    let seek_position = use_state(|| None);

    let on_title_change = {
        log!("on_title_change");
        let state = state.clone();
        Callback::from(move |e: Event| {
            let state = state.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            let title = input.value();
            let att = input.get_attribute("name").unwrap();
            state.dispatch(AppAction::TitleChanged(att, title));
        })
    };

    let on_file_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let state = state.clone();
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
                    let file_name = sf.name();
                    let task = gloo_file::callbacks::read_as_bytes(&sf, move |bytes| {
                        let contents = bytes.unwrap();
                        state.dispatch(AppAction::MP3Ready(contents));
                        state.dispatch(AppAction::SetFileName(file_name.clone()));
                    });

                    sd.dispatch(AppAction::AddReader(task));
                }
            }
        })
    };

    let save_clicked = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            log!("save clicked");
            let tag = state.tag.clone().unwrap();
            log!(format!("1 {:?}", tag));

            let b = state.bytes.clone();
            log!(format!("2 {:?}", b.len()));

            let mut curs = Cursor::new(b);

            // tag.write_to(curs, Version::Id3v23).unwrap();
            match tag.write_to_file(&mut curs, Version::Id3v23) {
                Ok(_) => log!("Successfully wrote tag to file"),
                Err(e) => log!(format!("Error writing tag to file: {:?}", e)),
            };
            let uint8arr = js_sys::Uint8Array::new(
                &unsafe { js_sys::Uint8Array::view(curs.get_ref()) }.into(),
            );
            let array = js_sys::Array::new();
            array.push(&uint8arr.buffer());

            let bpb = web_sys::BlobPropertyBag::new();
            bpb.set_type("audio/mpeg");

            let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &bpb).unwrap();
            let blob_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

            log!(format!("Blob URL: {:?}", blob_url));

            state.dispatch(AppAction::URLCreated(blob_url));
        })
    };

    let clear_clicked = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            log!("clear clicked");
            state.dispatch(AppAction::ClearClicked);
        })
    };

    let mut blob_url: Option<String> = None;

    // create a blob of the mp3 file bytes
    if state.bytes.len() > 0 {
        let uint8arr = js_sys::Uint8Array::new(
            &unsafe { js_sys::Uint8Array::view(&state.bytes.clone()) }.into(),
        );
        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());

        let bpb = web_sys::BlobPropertyBag::new();
        bpb.set_type("audio/mpeg3;audio/x-mpeg-3;video/mpeg;video/x-mpeg;text/xml");
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &bpb).unwrap();
        let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        // Zero bytes
        log!(format!("{:?}", download_url));
        blob_url = Some(download_url);
    };

    let on_seek = {
        let seek_position = seek_position.clone();
        Callback::from(move |pos: f64| {
            seek_position.set(Some(pos));
        })
    };

    let add_new_tag = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(AppAction::AddNewTag);
        })
    };

    html! {
        <>
            <div class="container">
                <div class="card">
                    <header class="card-header">
                        <p class="card-header-title">{"Upload File"}</p>
                        // <h1 class="title">{"Upload File"}</h1>
                    </header>
                    <div class="card-content">
                        <div class="content">
                            <FileLoader on_file_change={on_file_change} />
                        </div>
                    </div>
                </div>
            </div>

            if blob_url.is_some() {
                <MP3Audio
                    url={blob_url.unwrap()}
                    seek_position={seek_position}
                    file_name={state.name.clone()}
                />
                // <a href={blob_url.clone().unwrap()} download="test.mp3">{"Download"}</a>

                <ID3Tag add_new_tag={add_new_tag} tag={state.tag.clone()} on_value_change={on_title_change} save_clicked={save_clicked} clear_clicked={clear_clicked} on_seek_position_change={on_seek} />
                <div>{ state.url.clone() }</div>
            }
        </>
    }
}

fn _change_location(url: &str) {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window
        .location()
        .set_href(url)
        .expect("location change failed");
}

fn _alert(message: &str) {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window.alert_with_message(message).expect("alert failed");
}

fn main() {
    yew::Renderer::<App>::new().render();
}
