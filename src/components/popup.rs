use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Popup)]
pub fn popup_component(PopupProps { on_close }: &PopupProps) -> Html {
    let on_close_click = {
        let on_close = on_close.clone();
        Callback::from(move |event: MouseEvent| on_close.emit(event))
    };

    html! {
        <div class="notification is-warning">
            <button class="delete" onclick={on_close_click}></button>
            <p>{"This is a popup window!"}</p>
        </div>
    }
}
