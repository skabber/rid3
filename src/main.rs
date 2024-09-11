use yew::prelude::*;
use yewdux::prelude::*;
mod components;
use components::{FileLoader, ID3Tag, MP3Audio};
use web_sys::wasm_bindgen::JsCast;
mod state;
use state::AppState;

use gloo::console::log;
use id3::Version;
use std::io::Cursor;

#[function_component]
fn App() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    let save_clicked = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let tag = state.tag.clone().unwrap();
            let mut b = state.bytes.clone();
            let curs = Cursor::new(&mut b);

            tag.write_to(curs, Version::Id3v23).unwrap();

            let download_url = create_blob_url(b).unwrap();
            force_download(&download_url, &state.name);
        })
    };

    let clear_clicked: Callback<MouseEvent> =
        dispatch.reduce_callback(|_| std::rc::Rc::new(AppState::default()));

    let blob_url = create_blob_url(state.bytes.clone());

    html! {
        <>
            <div class="container">
                <div class="card">
                    <header class="card-header">
                        <p class="card-header-title">{"Upload File"}</p>
                    </header>
                    <div class="card-content">
                        <div class="content">
                            <FileLoader />
                            if state.bytes.len() > 0 {
                                <div>
                                    <strong>{"File Name: "}</strong>
                                    <span>{state.name.clone()}</span>
                                    <button class="button is-info" onclick={save_clicked}>{"Save"}</button>
                                    <button class="button" onclick={clear_clicked.clone()}>{" Clear "}</button>
                                </div>
                            }
                        </div>
                    </div>
                </div>
            </div>

            if blob_url.is_some() {
                <MP3Audio
                    url={blob_url.unwrap()}
                    file_name={state.name.clone()}
                />

                <ID3Tag tag={state.tag.clone()} />
                <div>{ state.url.clone() }</div>
            }
        </>
    }
}

fn _change_location(url: &str) {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window
        .location()
        .set_href(url)
        .expect("location change failed");
}

fn _alert(message: &str) {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window.alert_with_message(message).expect("alert failed");
}

fn main() {
    yew::Renderer::<App>::new().render();
}

/// Forces the download of a file from a given blob URL.
///
/// This function creates a temporary invisible link element, sets its href to the provided
/// blob URL, triggers a click event on it to start the download, and then removes the element
/// from the DOM.
///
/// # Arguments
///
/// * `blob_url` - A string slice that holds the URL of the blob to be downloaded.
/// * `filename` - A string slice that specifies the desired filename for the download.
///
/// # Panics
///
/// This function will panic if:
/// - The global `window` object is not available.
/// - The document object cannot be retrieved from the window.
/// - Creating the temporary `<a>` element fails.
/// - Setting attributes on the `<a>` element fails.
/// - Appending or removing the `<a>` element from the document body fails.
///
/// # Examples
///
/// ```
/// let blob_url = "blob:http://example.com/1234-5678-9012-3456";
/// let filename = "example.mp3";
/// force_download(blob_url, filename);
/// ```
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

/// Creates a blob URL from a vector of bytes.
///
/// This function takes a vector of bytes representing a file (typically an MP3 file)
/// and creates a blob URL that can be used to reference the file in the browser.
///
/// # Arguments
///
/// * `bytes` - A vector of bytes representing the file content.
///
/// # Returns
///
/// * `Option<String>` - Some(String) containing the blob URL if successful, None if the input vector is empty.
///
/// # Panics
///
/// This function may panic if:
/// - Creating the `Uint8Array` fails.
/// - Creating the `Blob` fails.
/// - Creating the object URL fails.
///
/// # Examples
///
/// ```
/// let mp3_bytes = vec![/* ... MP3 file bytes ... */];
/// let blob_url = create_blob_url(mp3_bytes);
/// if let Some(url) = blob_url {
///     println!("Created blob URL: {}", url);
/// } else {
///     println!("Failed to create blob URL");
/// }
/// ```
fn create_blob_url(bytes: Vec<u8>) -> Option<String> {
    // create a blob of the mp3 file bytes
    if bytes.len() > 0 {
        let uint8arr =
            js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&bytes.clone()) }.into());
        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());

        let bpb = web_sys::BlobPropertyBag::new();
        bpb.set_type("audio/mpeg3;audio/x-mpeg-3;video/mpeg;video/x-mpeg;text/xml");
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &bpb).unwrap();
        let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

        log!(format!("{:?}", download_url));
        Some(download_url)
    } else {
        None
    }
}
