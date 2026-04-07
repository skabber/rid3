use gloo::console::log;
use web_sys::wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::{use_media_with_options, UseMediaOptions};

#[derive(Properties, PartialEq)]
pub struct MP3AudioProps {
    pub url: String,
    pub seek_position: UseStateHandle<Option<f64>>,
    pub file_name: String,
}

fn format_time(seconds: f64) -> String {
    let s = seconds as i32;
    format!("{}:{:02}", s / 60, s % 60)
}

#[function_component(MP3Audio)]
pub fn mp3_audio(
    MP3AudioProps {
        url,
        seek_position,
        file_name: _,
    }: &MP3AudioProps,
) -> Html {
    let options = UseMediaOptions {
        ontimeupdate: None,
        ..Default::default()
    };
    let node_audio = use_node_ref();
    let audio = use_media_with_options(node_audio.clone(), url.clone(), options);

    {
        let audio = audio.clone();
        let seek_position = seek_position.clone();
        use_effect_with(seek_position, move |seek_position| {
            if let Some(position) = seek_position.as_ref() {
                log!("Seeking to {:?}", *position);
                audio.seek(*position);
                audio.play();
            }
        });
    }

    let toggle_play = {
        let audio = audio.clone();
        Callback::from(move |_| {
            if *audio.playing {
                audio.pause();
            } else {
                audio.play();
            }
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

    let progress_pct = if *audio.duration > 0.0 {
        (*audio.time / *audio.duration) * 100.0
    } else {
        0.0
    };

    html! {
        <div class="audio-player">
            <audio ref={node_audio} src={url.clone()} />
            <div class="player-controls">
                <button class="player-btn" onclick={toggle_play}>
                    if *audio.playing { {"\u{23F8}"} } else { {"\u{25B6}"} }
                </button>
                <div class="player-progress-wrap">
                    <span class="player-time">{ format_time(*audio.time) }</span>
                    <div class="progress-bar" onclick={onseek}>
                        <div class="progress-fill" style={format!("width: {}%", progress_pct)}></div>
                    </div>
                    <span class="player-time right">{ format_time(*audio.duration) }</span>
                </div>
            </div>
        </div>
    }
}
