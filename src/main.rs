use yew::prelude::*;

mod components;
use components::FileLoader;

mod state;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <FileLoader  />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
