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
    pub on_value_change: Callback<Event>,
    pub save_clicked: Callback<MouseEvent>,
    pub clear_clicked: Callback<MouseEvent>,
}

#[function_component(ID3Tag)]
pub fn tag(
    ID3TagProps {
        tag,
        on_value_change,
        save_clicked,
        clear_clicked,
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
        <div>
            <ID3Image pic={pic.clone()}/>
            <table class="table">
                <thead>
                    <tr>
                        <th>{"ID3 Tag"}</th>
                        <th>{"value"}</th>
                    </tr>
                </thead>
            </table>
            <Frames frames={frames} on_value_change={on_value_change}/>
            <Chapters chapters={chaps}/>
            <button class="button is-info" onclick={save_clicked}>{"Save"}</button>
            <button class="button" onclick={clear_clicked}>{" Clear "}</button>
            //<button class="is-info" onclick={save_clicked}>{"Save"}</button>
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
        let mut value = "".to_string();
        if name == "USLT" {
            value = f.content().lyrics().unwrap().text.to_string();
        } else if name == "COMM" {
            value = f.content().comment().unwrap().text.to_string();
        } else if name == "CTOC" {
            value = f.content().table_of_contents().unwrap().elements.join(", ");
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

#[derive(Properties, PartialEq)]
struct ChaptersProps {
    chapters: Vec<Chapter>,
}

#[function_component(Chapters)]
fn chapters(ChaptersProps { chapters }: &ChaptersProps) -> Html {
    let mut c = Vec::new();
    for chapter in chapters {
        let id = chapter.element_id.clone();
        let start_time = chapter.start_time;
        let mut name = "";
        let mut link = "".to_string();
        chapter.frames.iter().for_each(|f| {
            log!(format!("{:?}", f.id()));
            if f.id() == "WXXX" {
                link = f.content().extended_link().unwrap().link.to_string();
            }
            if let Some(text) = f.content().text() {
                name = text;
            }
        });
        c.push(html! {
            <div class="row">{ id } { ":" } { name } {":"} { start_time } {":"} {link} </div>
        });
    }
    html! {
        <div class="rows">
            { for c }
        </div>
    }
}
