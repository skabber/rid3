use gloo::console::log;
use id3::{frame::Chapter, Tag};
use web_sys::Event;
use yew::prelude::*;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;

#[derive(Properties, PartialEq)]
pub struct ID3TagProps {
    pub tag: Option<Tag>,
    pub on_value_change: Callback<Event>,
    pub on_seek_position_change: Callback<f64>,
}

#[function_component(ID3Tag)]
pub fn tag(
    ID3TagProps {
        tag,
        on_value_change,
        on_seek_position_change,
    }: &ID3TagProps,
) -> Html {
    let mut chaps = Vec::new();
    let mut frames = Vec::new();
    if let Some(tag) = tag {
        for f in tag.frames() {
            log!(format!("{:?}", f.id()));
        }
        frames = tag
            .frames()
            .cloned()
            .filter(|f| f.id() != "CHAP" && f.id() != "APIC")
            .collect();
        chaps = tag.chapters().cloned().collect();
    }

    html! {
        <div class="content-grid">
            <div class="panel">
                <div class="panel-header">{"ID3 Tags"}</div>
                <div class="panel-body">
                    <Frames frames={frames} on_value_change={on_value_change} />
                </div>
            </div>

            if !chaps.is_empty() {
                <div class="panel">
                    <div class="panel-header">{format!("Chapters ({})", chaps.len())}</div>
                    <div class="panel-body">
                        <Chapters chapters={chaps} on_seek_position_change={on_seek_position_change} />
                    </div>
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FramesProps {
    frames: Vec<id3::frame::Frame>,
    on_value_change: Callback<Event>,
}

#[function_component(Frames)]
fn tags(
    FramesProps {
        frames,
        on_value_change,
    }: &FramesProps,
) -> Html {
    html! {
        <table class="tag-table">
            { for frames.iter().map(|f| {
                let name = String::from(f.id());
                let value: String = if name == "USLT" {
                    f.content().lyrics().unwrap().text.to_string()
                } else if name == "COMM" {
                    f.content().comment().unwrap().text.to_string()
                } else if name == "CTOC" {
                    f.content().table_of_contents().unwrap().elements.join(", ")
                } else {
                    String::from(f.content().text().unwrap_or(""))
                };

                html! {
                    <tr>
                        <td class="tag-label">{ name.clone() }</td>
                        <td><input type="text" name={ name } value={ value } onchange={on_value_change} /></td>
                    </tr>
                }
            }) }
        </table>
    }
}

#[derive(Properties, PartialEq)]
struct ChapterArtProps {
    pic: String,
}

#[function_component(ChapterArt)]
fn chapter_art(ChapterArtProps { pic }: &ChapterArtProps) -> Html {
    let modal_open = use_state(|| false);

    let open_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            modal_open.set(true);
        })
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            modal_open.set(false);
        })
    };

    if pic.is_empty() {
        html! {
            <div class="chapter-art">
                <div class="chapter-art-placeholder">{"\u{266B}"}</div>
            </div>
        }
    } else {
        let src = format!("data:image/png;base64,{}", pic);
        html! {
            <>
                <div class="chapter-art" onclick={open_modal}>
                    <img src={src.clone()} alt="Chapter art" />
                </div>
                if *modal_open {
                    <div class="modal-overlay" onclick={close_modal.clone()}>
                        <div class="modal-content">
                            <img src={src} alt="Chapter art full size" />
                        </div>
                        <button class="modal-close" onclick={close_modal}>{"\u{00D7}"}</button>
                    </div>
                }
            </>
        }
    }
}

// Format milliseconds as M:SS
fn format_time(ms: u32) -> String {
    let secs = ms / 1000;
    format!("{}:{:02}", secs / 60, secs % 60)
}

#[derive(Properties, PartialEq)]
struct ChaptersProps {
    chapters: Vec<Chapter>,
    pub on_seek_position_change: Callback<f64>,
}

#[function_component(Chapters)]
fn chapters(
    ChaptersProps {
        chapters,
        on_seek_position_change,
    }: &ChaptersProps,
) -> Html {
    html! {
        <ul class="chapters-list">
            { for chapters.iter().map(|chapter| {
                let start_time = chapter.start_time;
                let end_time = chapter.end_time;
                let mut name = String::new();
                let mut link: Option<String> = None;
                let mut pic = String::new();

                for f in &chapter.frames {
                    match f.id() {
                        "TIT2" => {
                            name = f.content().text().unwrap_or("").to_string();
                        }
                        "APIC" => {
                            if let Some(p) = f.content().picture() {
                                log!(format!("APIC.len == {:?}", p.data.len()));
                                pic = BASE64.encode(&p.data);
                            }
                        }
                        "WXXX" => {
                            link = Some(f.content().extended_link().unwrap().link.to_string());
                        }
                        _ => {}
                    }
                }

                let on_play = on_seek_position_change.reform(move |_| (start_time / 1000) as f64);

                html! {
                    <li class="chapter-item">
                        <ChapterArt pic={pic} />
                        <div class="chapter-info">
                            <div class="chapter-name">
                                if let Some(ref link) = link {
                                    <a href={link.clone()} target="_blank">{ &name }</a>
                                } else {
                                    { &name }
                                }
                            </div>
                            <div class="chapter-time">
                                { format_time(start_time) }{ " \u{2013} " }{ format_time(end_time) }
                            </div>
                        </div>
                        <button class="chapter-play-btn" onclick={on_play} title="Play from here">
                            {"\u{25B6}"}
                        </button>
                    </li>
                }
            }) }
        </ul>
    }
}
