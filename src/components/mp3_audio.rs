use web_sys::wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::{use_media_with_options, UseMediaOptions};
use yewdux::prelude::*;

use crate::state::AppState;

#[derive(Properties, PartialEq)]
pub struct MP3AudioProps {
    pub url: String,
    pub file_name: String,
}

#[function_component(MP3Audio)]
pub fn mp3_audio(MP3AudioProps { url, file_name }: &MP3AudioProps) -> Html {
    let (state, _) = use_store::<AppState>();
    let options = UseMediaOptions {
        ontimeupdate: None,
        ..Default::default()
    };
    let node_audio = use_node_ref();
    let audio = use_media_with_options(node_audio.clone(), url.clone(), options);

    {
        let audio = audio.clone();
        use_effect_with(state.seek_position, move |seek_position| {
            audio.seek(seek_position.clone());
            audio.play();
        });
    }

    let onplay = {
        let audio = audio.clone();
        Callback::from(move |_| {
            audio.play();
        })
    };
    let onpause = {
        let audio = audio.clone();
        Callback::from(move |_| {
            audio.pause();
        })
    };

    let onseek = {
        let audio = audio.clone();
        Callback::from(move |e: MouseEvent| {
            let target: Element = e.target_unchecked_into();
            if let Ok(html_element) = target.dyn_into::<Element>() {
                let rect = html_element.get_bounding_client_rect();
                let click_position = e.client_x() as f64 - rect.left();
                let progress_width = rect.width();
                let seek_percentage = click_position / progress_width;
                let seek_time = seek_percentage * *audio.duration;
                audio.seek(seek_time);
            }
        })
    };

    html! {
        <>
            <div class="container">
                <div class="card">
                    <div class="card-content">
                        <header class="card-header">
                            <p class="card-header-title">{ file_name }</p>
                        </header>
                        <audio ref={node_audio} src={url.clone()} controls=true />
                        <progress
                            class="progress is-primary"
                            value={audio.time.to_string()}
                            max={audio.duration.to_string()}
                            onclick={onseek}
                            style="cursor: pointer;"
                        ></progress>
                        <button class="button" onclick={onplay} disabled={*audio.playing}>{ "Play" }</button>
                        <button class="button" onclick={onpause} disabled={!*audio.playing}>{ "Pause" }</button>
                        <div>{format!("{:02}:{:02}", (*audio.time / 60.0) as i32, (*audio.time % 60.0) as i32)}</div>
                    </div>
                </div>
            </div>
        </>
    }
}
