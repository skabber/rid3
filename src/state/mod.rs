use gloo::console::log;
use gloo_file::{callbacks::FileReader, File};
use id3::{Frame, Tag, TagLike};
use std::io::Cursor;
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast;
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
    URLCreated(String),
    ClearClicked,
    SetFileName(String),
    AddNewTag,
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
            AppAction::URLCreated(url) => {
                log!("title changed");
                force_download(url.as_str(), "rid3.mp3");
                std::rc::Rc::new(AppState {
                    mp3: self.mp3.clone(),
                    tag: self.tag.clone(),
                    frames: self.frames.clone(),
                    reader_tasks: self.reader_tasks.clone(),
                    name: self.name.clone(),
                    bytes: self.bytes.clone(),
                    url,
                })
            }
            AppAction::ClearClicked => std::rc::Rc::new(AppState {
                mp3: None,
                tag: None,
                frames: Vec::new(),
                reader_tasks: None,
                name: String::new(),
                bytes: Vec::new(),
                url: String::new(),
            }),
            AppAction::SetFileName(name) => std::rc::Rc::new(AppState {
                mp3: self.mp3.clone(),
                tag: self.tag.clone(),
                frames: self.frames.clone(),
                reader_tasks: self.reader_tasks.clone(),
                name,
                bytes: self.bytes.clone(),
                url: self.url.clone(),
            }),
            AppAction::AddNewTag => {
                let mut new_tag = self.tag.clone().unwrap_or_else(|| Tag::new());
                new_tag.add_frame(Frame::text("TXXX", "New Tag"));
                std::rc::Rc::new(AppState {
                    mp3: self.mp3.clone(),
                    tag: Some(new_tag),
                    frames: self.frames.clone(),
                    reader_tasks: self.reader_tasks.clone(),
                    name: self.name.clone(),
                    bytes: self.bytes.clone(),
                    url: self.url.clone(),
                })
            }
        }
    }
}

fn force_download(blob_url: &str, filename: &str) {
    // Get the window object
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    // Create a temporary <a> element
    let a = document
        .create_element("a")
        .expect("failed to create a element");
    let a: web_sys::HtmlElement = a.dyn_into::<web_sys::HtmlElement>().unwrap();

    // Set the href to the blob URL
    a.set_attribute("href", blob_url)
        .expect("failed to set href");

    // Set the download attribute with the desired filename
    a.set_attribute("download", filename)
        .expect("failed to set download attribute");

    // Make the link invisible
    // a.style()
    //     .set_property("display", "none")
    //     .expect("failed to set style");
    a.set_attribute("style", "display: none")
        .expect("failed to set style attribute");

    // Add the link to the document body
    document
        .body()
        .expect("document should have a body")
        .append_child(&a)
        .expect("failed to append child");

    // Programmatically click the link
    a.click();

    // Remove the link from the document
    document
        .body()
        .expect("document should have a body")
        .remove_child(&a)
        .expect("failed to remove child");
}
