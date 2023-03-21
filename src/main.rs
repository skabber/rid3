use gloo::console::log;
use gloo_file::{callbacks::FileReader, File};
use id3::frame::Chapter;
use id3::{self, Frame, Tag};

use std::rc::Rc;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Debug)]
struct AppState {
    mp3: Option<File>,
    tag: Option<Tag>,
    frames: Vec<Frame>,
    reader_tasks: Option<Rc<FileReader>>,
}

enum AppAction {
    MP3Ready(Vec<u8>),
    // AddTag(Tag),
    AddReader(FileReader),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: std::rc::Rc<Self>, action: AppAction) -> std::rc::Rc<Self> {
        match action {
            AppAction::AddReader(reader) => {
                log!("add reader");
                let mut tasks = self.reader_tasks.clone();
                tasks = Some(Rc::new(reader));
                std::rc::Rc::new(AppState {
                    mp3: self.mp3.clone(),
                    tag: self.tag.clone(),
                    frames: self.frames.clone(),
                    reader_tasks: tasks,
                })
            }
            AppAction::MP3Ready(contents) => {
                log!("mp3 ready");

                let tag = id3::Tag::read_from(contents.as_slice()).unwrap();
                // log!(format!("{:?}", tag.version()).as_str());

                // for chapter in tag.chapters() {
                //     log!(format!("{:?}", chapter.element_id).as_str());
                //     for frame in &chapter.frames {
                //         let c = frame.content();
                //         if let Some(text) = c.text() {
                //             log!(text);
                //         }
                //     }
                // }

                std::rc::Rc::new(AppState {
                    mp3: self.mp3.clone(),
                    tag: Some(tag),
                    frames: self.frames.clone(),
                    reader_tasks: self.reader_tasks.clone(),
                })
            } // AppAction::AddTag(tag) => {
              //     log!("add tag");
              //     std::rc::Rc::new(AppState {
              //         mp3: self.mp3.clone(),
              //         tag: self.tag.clone(),
              //         frames: self.frames.clone(),
              //         reader_tasks: self.reader_tasks.clone(),
              //     })
              // } // AppAction::MP3Ready(contents: Vec<u8>) => {}

              // AppAction::Add(item) => {
              //     log!("add item");
              //     let mut items = self.items.clone();
              //     items.push(Asset {
              //         url: item.url,
              //         size: item.size,
              //         browser_download_url: item.browser_download_url,
              //         name: item.name,
              //     });
              //     std::rc::Rc::new(AppState { items })
              // }
        }
    }
}

#[function_component(FileLoader)]
fn file_loader() -> Html {
    let state = use_reducer(|| AppState {
        mp3: None,
        tag: None,
        frames: Vec::new(),
        reader_tasks: None,
    });
    let state_view = state.clone();

    // let tasks = use_state(Vec::<FileReader>::new);

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
            // <input type="file" accept="image/png, image/gif" onchange={on_change} multiple=false/>
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

    html! {
        <div>
            <img src={format!("data:image/png;base64,{}", pic)} width="500" />
            <h1>{ name }</h1>
            <h2>{ tpe1 }</h2>
            <h2>{ tit2 }</h2>
            <h2>{ comm }</h2>
            <h2>{ uslt }</h2>
            <Chapters chapters={chaps}/>
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

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <FileLoader  />
        </div>
    }
}

fn main() {
    log!("main");
    yew::Renderer::<App>::new().render();
}
