use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Popup)]
pub fn PopupComponent(PopupProps { on_close }: &PopupProps) -> Html {
    let on_close_click = {
        let on_close = on_close.clone();
        Callback::from(move |event: MouseEvent| on_close.emit(event))
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
