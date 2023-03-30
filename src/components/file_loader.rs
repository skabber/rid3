use gloo::console::log;
use id3::{frame::Chapter, Tag};
use web_sys::Event;
use yew::prelude::*;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;

#[derive(Properties, PartialEq)]
pub struct FileLoaderProps {
    pub on_file_change: Callback<Event>,
}

#[function_component(FileLoader)]
pub fn file_loader(FileLoaderProps { on_file_change }: &FileLoaderProps) -> Html {
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

#[derive(Properties, PartialEq)]
struct ID3Props {
    pic: String,
}

#[function_component(ID3Image)]
fn id3_image(ID3Props { pic }: &ID3Props) -> Html {
    if pic.is_empty() {
        html! {
            <div class="has-background-light">{" Upload File"}</div>
        }
    } else {
        html! {
            <div>
                <img src={format!("data:image/png;base64,{}", pic)} width="200" />
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ID3TagProps {
    pub tag: Option<Tag>,
    pub on_title_change: Callback<Event>,
    pub name: String,
    pub save_clicked: Callback<MouseEvent>,
    pub clear_clicked: Callback<MouseEvent>,
}

#[function_component(ID3Tag)]
pub fn tag(
    ID3TagProps {
        tag,
        on_title_change,
        name,
        save_clicked,
        clear_clicked,
    }: &ID3TagProps,
) -> Html {
    let mut tpe1 = "";
    let mut tit2 = "";
    let mut uslt = "";
    let mut comm = "";
    let mut chaps = Vec::new();
    let mut pic = String::new();
    if let Some(tag) = tag {
        for f in tag.frames() {
            if f.id() == "TALB" {
                // name = f.content().text().unwrap();
            } else if f.id() == "TPE1" {
                tpe1 = f.content().text().unwrap();
            } else if f.id() == "TIT2" {
                tit2 = f.content().text().unwrap();
            } else if f.id() == "USLT" {
                if let Some(l) = f.content().lyrics() {
                    uslt = l.text.as_str();
                }
            } else if f.id() == "COMM" {
                if let Some(c) = f.content().comment() {
                    comm = c.text.as_str();
                }
            } else if f.id() == "APIC" {
                if let Some(p) = f.content().picture() {
                    // log!(format!("{:?}", p.mime_type));
                    pic = BASE64.encode(&p.data);
                }
            } else if f.id() != "CHAP" {
                log!(format!("{:?}", f));
            }
        }
        chaps = tag.chapters().cloned().collect();
    }

    html! {
        <div>
            <ID3Image pic={pic.clone()}/>
            <table class="table">
                <thead>
                    <tr>
                        <th>{"ID3 Tag"}</th>
                        <th>{"value"}</th>
                    </tr>
                </thead>
                <tr>
                    <td><span class="tag">{"Title:"}</span></td>
                    <td><input type="text" name="tile" value={ name.clone() } onchange={on_title_change}/></td>
                </tr>
                <tr><td><span class="tag">{"TPE1:"}</span></td><td>{ tpe1 }</td></tr>
                <tr><td><span class="tag">{"TIT2"}</span></td><td>{ tit2 }</td></tr>
                <tr><td><span class="tag">{"COMM"}</span></td><td>{ comm }</td></tr>
                <tr><td><span class="tag">{"USLT"}</span></td><td>{ uslt }</td></tr>
            </table>
            <Chapters chapters={chaps}/>
            <button class="button is-info" onclick={save_clicked}>{"Save"}</button>
            <button class="button" onclick={clear_clicked}>{" Clear "}</button>
            //<button class="is-info" onclick={save_clicked}>{"Save"}</button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ChaptersProps {
    chapters: Vec<Chapter>,
}

#[function_component(Chapters)]
fn chapters(ChaptersProps { chapters }: &ChaptersProps) -> Html {
    let mut c = Vec::new();
    for chapter in chapters {
        let id = chapter.element_id.clone();
        let mut name = "";
        chapter.frames.iter().for_each(|f| {
            if let Some(text) = f.content().text() {
                name = text;
            }
        });
        c.push(html! {
            <div class="row">{ id } { ":" } { name } </div>
        });
    }
    html! {
        <div class="rows">
            { for c }
        </div>
    }
}
