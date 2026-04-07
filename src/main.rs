use yew::prelude::*;

mod components;
use components::{FileLoader, ID3Tag, MP3Audio};

mod state;
use state::{AppAction, AppState};

use gloo::console::log;
use gloo_file::File;
use id3::{TagLike, Version};
use std::io::Cursor;
use web_sys::wasm_bindgen::JsCast;
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

            let mut b = state.bytes.clone();
            let curs = Cursor::new(&mut b);
            tag.write_to(curs, Version::Id3v23).unwrap();

            let bytes = b.as_slice();
            let uint8arr =
                js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(bytes) }.into());
            let array = js_sys::Array::new();
            array.push(&uint8arr.buffer());

            let bpb = web_sys::BlobPropertyBag::new();
            bpb.set_type("audio/mpeg3;audio/x-mpeg-3;video/mpeg;video/x-mpeg;text/xml");

            let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &bpb).unwrap();
            let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

            let window: web_sys::Window = web_sys::window().expect("window not available");
            let element = window.document().unwrap().create_element("a").unwrap();
            element
                .set_attribute("href", download_url.as_str())
                .unwrap();

            let download_name = if state.name.is_empty() {
                "download.mp3".to_string()
            } else {
                state.name.clone()
            };
            element
                .set_attribute("download", &download_name)
                .unwrap();

            let body = window.document().unwrap().body().unwrap();
            body.append_child(&element).unwrap();
            let html_el: web_sys::HtmlElement = element.unchecked_into();
            html_el.click();
            let el: web_sys::Element = html_el.unchecked_into();
            body.remove_child(&el).unwrap();
        })
    };

    let clear_clicked = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(AppAction::ClearClicked);
        })
    };

    let mut blob_url: Option<String> = None;

    if !state.bytes.is_empty() {
        let uint8arr = js_sys::Uint8Array::new(
            &unsafe { js_sys::Uint8Array::view(&state.bytes.clone()) }.into(),
        );
        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());

        let bpb = web_sys::BlobPropertyBag::new();
        bpb.set_type("audio/mpeg3;audio/x-mpeg-3;video/mpeg;video/x-mpeg;text/xml");
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &bpb).unwrap();
        let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        blob_url = Some(download_url);
    };

    let on_seek = {
        let seek_position = seek_position.clone();
        Callback::from(move |pos: f64| {
            seek_position.set(Some(pos));
        })
    };

    // Extract album art for hero display
    let album_art = state.tag.as_ref().and_then(|tag| {
        use base64::engine::general_purpose::STANDARD as BASE64;
        use base64::engine::Engine as _;
        tag.frames()
            .find(|f| f.id() == "APIC")
            .and_then(|f| f.content().picture())
            .map(|p| BASE64.encode(&p.data))
    });

    html! {
        <div class="app">
            <header class="app-header">
                <h1>{"rid"}<span>{"3"}</span></h1>
                <p>{"MP3 ID3 Tag Editor"}</p>
            </header>

            if blob_url.is_none() {
                <FileLoader on_file_change={on_file_change} />
            }

            if blob_url.is_some() {
                <div class="hero-section">
                    <div class="album-art-container">
                        if let Some(ref pic) = album_art {
                            <img src={format!("data:image/png;base64,{}", pic)} alt="Album Art" />
                        } else {
                            <div class="album-art-placeholder">
                                {"\u{266B}"}
                            </div>
                        }
                    </div>
                    <div class="hero-info">
                        <div class="file-name">{ &state.name }</div>
                        if let Some(ref tag) = state.tag {
                            <div class="file-meta">
                                if let Some(title) = tag.title() {
                                    <div>{ title }</div>
                                }
                                if let Some(artist) = tag.artist() {
                                    <div>{ artist }</div>
                                }
                                if let Some(album) = tag.album() {
                                    <div>{ album }</div>
                                }
                            </div>
                        }
                        <div class="hero-actions">
                            <button class="btn btn-primary" onclick={save_clicked}>
                                {"\u{2B73} Save"}
                            </button>
                            <button class="btn btn-danger" onclick={clear_clicked}>
                                {"Clear"}
                            </button>
                        </div>
                    </div>
                </div>

                <MP3Audio
                    url={blob_url.unwrap()}
                    seek_position={seek_position}
                    file_name={state.name.clone()}
                />

                <ID3Tag
                    tag={state.tag.clone()}
                    on_value_change={on_title_change}
                    on_seek_position_change={on_seek}
                />
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
