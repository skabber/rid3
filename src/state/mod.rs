use gloo::console::log;
use gloo_file::{callbacks::FileReader, File};
use id3::{Frame, Tag, TagLike};
use std::io::Cursor;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct AppState {
    pub mp3: Option<File>,
    pub tag: Option<Tag>,
    pub frames: Vec<Frame>,
    pub reader_tasks: Option<Rc<FileReader>>,
    pub name: String,
    pub bytes: Vec<u8>,
    pub url: String,
}

pub enum AppAction {
    MP3Ready(Vec<u8>),
    AddReader(FileReader),
    TitleChanged(String, String),
    // URLCreated(String),
    ClearClicked,
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: std::rc::Rc<Self>, action: AppAction) -> std::rc::Rc<Self> {
        match action {
            AppAction::AddReader(reader) => {
                log!("add reader");
                std::rc::Rc::new(AppState {
                    mp3: self.mp3.clone(),
                    tag: self.tag.clone(),
                    frames: self.frames.clone(),
                    reader_tasks: Some(Rc::new(reader)),
                    name: self.name.clone(),
                    bytes: self.bytes.clone(),
                    url: self.url.clone(),
                })
            }
            AppAction::MP3Ready(contents) => {
                log!("mp3 ready");
                log!(format!("{:?}", contents.len()).as_str());
                let data = Cursor::new(contents.as_slice());
                let tag = id3::Tag::read_from2(data).unwrap();
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
                    name: self.name.clone(),
                    bytes: contents,
                    url: self.url.clone(),
                })
            }
            AppAction::TitleChanged(att, title) => {
                log!("title changed");
                let mut t = self.tag.clone().unwrap();
                // t.set_album(title.clone());
                t.set_text(att.as_str(), title.clone());
                // t.add_frame(Frame::with_content("TALB", Content::Text(title.clone())));
                log!(format!("{:?}", t).as_str());
                std::rc::Rc::new(AppState {
                    mp3: self.mp3.clone(),
                    tag: Some(t),
                    frames: self.frames.clone(),
                    reader_tasks: self.reader_tasks.clone(),
                    name: title,
                    bytes: self.bytes.clone(),
                    url: self.url.clone(),
                })
            }
            // AppAction::URLCreated(url) => {
            //     log!("title changed");
            //     std::rc::Rc::new(AppState {
            //         mp3: self.mp3.clone(),
            //         tag: self.tag.clone(),
            //         frames: self.frames.clone(),
            //         reader_tasks: self.reader_tasks.clone(),
            //         name: self.name.clone(),
            //         bytes: self.bytes.clone(),
            //         url,
            //     })
            // }
            AppAction::ClearClicked => std::rc::Rc::new(AppState {
                mp3: None,
                tag: None,
                frames: Vec::new(),
                reader_tasks: None,
                name: String::new(),
                bytes: Vec::new(),
                url: String::new(),
            }),
        }
    }
}
