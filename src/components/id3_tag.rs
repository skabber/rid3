use gloo::console::log;
use id3::{frame::Chapter, Tag};
use web_sys::Event;
use yew::classes;
use yew::prelude::*;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;

#[derive(Properties, PartialEq)]
pub struct ID3TagProps {
    pub tag: Option<Tag>,
    pub on_value_change: Callback<Event>,
    pub save_clicked: Callback<MouseEvent>,
    pub clear_clicked: Callback<MouseEvent>,
    pub on_seek_position_change: Callback<f64>,
}

#[function_component(ID3Tag)]
pub fn tag(
    ID3TagProps {
        tag,
        on_value_change,
        save_clicked,
        clear_clicked,
        on_seek_position_change,
    }: &ID3TagProps,
) -> Html {
    let mut chaps = Vec::new();
    let mut frames = Vec::new();
    let mut pic = String::new();
    if let Some(tag) = tag {
        for f in tag.frames() {
            log!(format!("{:?}", f.id()));
            if f.id() == "APIC" {
                if let Some(p) = f.content().picture() {
                    log!(format!("{:?}", p.mime_type));
                    pic = BASE64.encode(&p.data);
                }
            } else if f.id() != "CHAP" {
                log!(format!("xxx {:?}", f));
            }
        }
        frames = tag
            .frames()
            .cloned()
            .filter(|f| f.id() != "CHAP" && f.id() != "APIC")
            .collect();
        chaps = tag.chapters().cloned().collect();
    }

    html! {
        <div class="container">
            <div class="card">
                <div class="card-header">
                    <p class="card-header-title">{"ID3 Tag"}</p>
                </div>

                <div class="card-content">
                    <div class="columns">
                        <div class="column">
                            <ChapterArt pic={pic.clone()}/>
                        </div>
                        <div class="column">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>{"ID3 Tag"}</th>
                                        <th>{"value"}</th>
                                    </tr>
                                </thead>
                                <Frames frames={frames} on_value_change={on_value_change}/>
                            </table>
                        </div>
                        <div class="column">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>{"Chapters"}</th>
                                        <th>{"Name"}</th>
                                        <th>{"Times"}</th>
                                        <th>{"Link"}</th>
                                        <th>{"Art"}</th>
                                        <th>{"Controls"}</th>
                                    </tr>
                                </thead>
                                <Chapters chapters={chaps} on_seek_position_change={on_seek_position_change}/>
                            </table>
                            <button class="button is-info" onclick={save_clicked}>{"Save"}</button>
                            <button class="button" onclick={clear_clicked}>{" Clear "}</button>
                            //<button class="is-info" onclick={save_clicked}>{"Save"}</button>
                        </div>
                    </div>
                </div>
            </div>
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
    frames.iter().map(|f| {
        let name = String::from(f.id());
        let value: String;
        if name == "USLT" {
            value = f.content().lyrics().unwrap().text.to_string();
        } else if name == "COMM" {
            value = f.content().comment().unwrap().text.to_string();
        } else if name == "CTOC" {
            value = f.content().table_of_contents().unwrap().elements.join(", ")
        } else {
            value = String::from(f.content().text().unwrap_or(""));
        }

        html! {
            <tr>
                <td><span>{ name.clone() }</span></td>
                <td><input type="text" name={ name } value={ value } onchange={on_value_change}/></td>
            </tr>
        }
     }).collect()
}

// ChapterArtProps
#[derive(Properties, PartialEq)]
struct ChapterArtProps {
    pic: String, // base64 encoded image
}

#[function_component(ChapterArt)]
fn chapter_art(ChapterArtProps { pic }: &ChapterArtProps) -> Html {
    let modal_classes = use_state(|| vec!["modal"]);
    let toggle_modal = {
        let classes = modal_classes.clone();
        if classes.contains(&"is-active") {
            Callback::from(move |_: MouseEvent| {
                classes.set(vec!["modal"]);
            })
        } else {
            Callback::from(move |_: MouseEvent| {
                classes.set(vec!["modal", "is-active"]);
            })
        }
    };
    if pic.is_empty() {
        html! {
            <></>
        }
    } else {
        html! {
            <>
                <img src={format!("data:image/png;base64,{}", pic.clone())} width="200" onclick={toggle_modal.clone()} />
                <div class={classes!((*modal_classes).clone())}>
                <div class="modal-background" onclick={toggle_modal.clone()}></div>
                  <div class="modal-content">
                    <p class="image">
                      <img src={format!("data:image/png;base64,{}", pic.clone())} />
                    </p>
                  </div>
                  <button class="modal-close is-large" aria-label="close" onclick={toggle_modal}></button>
                </div>
            </>
        }
    }
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
    let mut c = Vec::new();
    for chapter in chapters {
        let id = chapter.element_id.clone();
        let start_time = chapter.start_time;
        let end_time = chapter.end_time;
        let mut name = "";
        let mut link: Option<String> = None;
        let mut pic: Option<String> = None;
        chapter.frames.iter().for_each(|f| match f.id() {
            "TIT2" => {
                name = f.content().text().unwrap();
            }
            "APIC" => {
                if let Some(p) = f.content().picture() {
                    log!(format!("APIC.len == {:?}", p.data.len()));
                    pic = Some(BASE64.encode(&p.data));
                }
            }
            "WXXX" => {
                link = Some(f.content().extended_link().unwrap().link.to_string());
            }
            _ => {}
        });

        c.push(html! {
            <tr>
                <td>{ id }</td>
                <td>{ name }</td>
                <td>{ start_time/1000 } {"-"} { end_time/1000 }</td>
                <td>
                    if let Some(link) = link {
                        <a href={link.clone()}>{link}</a>
                    }
                </td>
                <td>
                    if pic.is_some() {
                        <ChapterArt pic={pic.clone().unwrap()}/>
                    }
                </td>
                <td>
                    <button class="button is-info" onclick={on_seek_position_change.reform(move |_| (start_time/1000) as f64)}>{">"}</button>
                </td>
            </tr>
        });
    }
    html! {
        { for c }
    }
}
