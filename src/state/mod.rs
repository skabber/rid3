use gloo::console::log;
use gloo_file::{callbacks::FileReader, File};
use id3::{self, Frame, Tag};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct AppState {
    pub mp3: Option<File>,
    pub tag: Option<Tag>,
    pub frames: Vec<Frame>,
    pub reader_tasks: Option<Rc<FileReader>>,
    pub name: String,
}

pub enum AppAction {
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
                    name: self.name.clone(),
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
                    name: self.name.clone(),
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
