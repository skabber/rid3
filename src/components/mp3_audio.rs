// use _MP3AudioProps::seek_position;
use gloo::console::log;

use web_sys::{Element, HtmlElement};
use yew::prelude::*;
use yew_hooks::{use_media_with_options, UseMediaOptions};
//MP3AudioProps
#[derive(Properties, PartialEq)]
pub struct MP3AudioProps {
    pub url: String,
    pub seek_position: UseStateHandle<Option<f64>>,
}

#[function_component(MP3Audio)]
pub fn mp3_audio(MP3AudioProps { url, seek_position }: &MP3AudioProps) -> Html {
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
            if let Ok(html_element) = target.dyn_into::<HtmlElement>() {
                if let Some(rect) = html_element.get_bounding_client_rect() {
                    let click_position = e.client_x() as f64 - rect.left();
                    let progress_width = rect.width();
                    let seek_percentage = click_position / progress_width;
                    let seek_time = seek_percentage * *audio.duration;
                    audio.seek(seek_time);
                }
            }
        })
    };

    html! {
        <>
            <div class="container">
                <div class="card">
                    <div class="card-content">
                        <header class="card-header">
                            <p class="card-header-title">{ "MP3 Audio" }</p>
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
                        <div>{*audio.time}</div>
                    </div>
                </div>
            </div>
        </>
    }
}
