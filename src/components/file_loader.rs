use crate::state::{AppAction, AppState};
use gloo::console::log;
use gloo_file::{callbacks::FileReader, File};
use id3::{frame::Chapter, Tag};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[function_component(FileLoader)]
pub fn file_loader() -> Html {
    let state = use_reducer(|| AppState {
        mp3: None,
        tag: None,
        frames: Vec::new(),
        reader_tasks: None,
        name: String::new(),
    });
    let state_view = state.clone();

    let tasks = use_state(Vec::<FileReader>::new);

    let on_change = {
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
                    let task = gloo_file::callbacks::read_as_bytes(&sf, move |bytes| {
                        let contents = bytes.unwrap();

                        state.dispatch(AppAction::MP3Ready(contents));
                    });

                    sd.dispatch(AppAction::AddReader(task));
                }
                // tasks.set(vec![task]); // leaks memory
            }
        })
    };

    let mut t = None;

    if let Some(tag) = &state_view.tag {
        t = Some(tag.clone());
    }

    html!(
        <div>
            <input type="file" accept="audio/mp3,audio/*" onchange={on_change} multiple=false/>
            <ID3Tag tag={t} />
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct ID3TagProps {
    tag: Option<Tag>,
}

#[function_component(ID3Tag)]
fn tag(ID3TagProps { tag }: &ID3TagProps) -> Html {
    let mut name = "";
    let mut tpe1 = "";
    let mut tit2 = "";
    let mut uslt = "";
    let mut comm = "";
    let mut chaps = Vec::new();
    let mut pic = String::new();
    if let Some(tag) = tag {
        for f in tag.frames() {
            if f.id() == "TALB" {
                name = f.content().text().unwrap();
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
                    log!(format!("{:?}", p.mime_type));
                    pic = base64::encode(&p.data);
                }
            } else if f.id() != "CHAP" {
                log!(format!("{:?}", f));
            }
        }
        chaps = tag.chapters().cloned().collect();
    }

    let title = String::from(name);

    let save_clicked = Callback::from(move |_: MouseEvent| {
        log!("save clicked");
    });

    html! {
        <div>
            <img src={format!("data:image/png;base64,{}", pic)} width="500" />
            <h1>{"Title:"} <input type="text" name="tile" value={ title }/></h1>
            <h2>{ tpe1 }</h2>
            <h2>{ tit2 }</h2>
            <h2>{ comm }</h2>
            <h2>{ uslt }</h2>
            <Chapters chapters={chaps}/>
            <button onclick={save_clicked}>{"Save"}</button>
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
            <div>{ id } { ":" } { name } </div>
        });
    }
    html! {
        <div>
            { for c }
        </div>
    }
}
