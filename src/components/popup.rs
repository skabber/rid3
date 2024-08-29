use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps {
    pub on_close: Callback<()>,
}

#[function_component]
pub fn Popup(PopupProps { on_close }: &PopupProps) -> Html {
    let on_close_click = {
        let on_close = on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    html! {
        <div class="popup">
            <div class="popup-content">
                <span class="close" onclick={on_close_click}>{"×"}</span>
                <p>{"This is a popup window!"}</p>
            </div>
        </div>
    }
}
