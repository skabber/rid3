use yew::prelude::*;

// ModalProps
#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub children: Html,
}

#[allow(non_snake_case)]
#[function_component]
pub fn Modal(ModalProps { title, children }: &ModalProps) -> Html {
    let modal_classes = use_state(|| vec!["modal"]);
    let toggle_modal = {
        let classes = modal_classes.clone();
        if classes.contains(&"is-active") {
            Callback::from(move |_: MouseEvent| {
                classes.set(vec!["modal"]);
            })
        } else {
            Callback::from(move |_: MouseEvent| {
                classes.set(vec!["modal", "is-active"]);
            })
        }
    };

    html! {
        <>
          <button class="button is-primary" onclick={toggle_modal.clone()}>{title.clone()}</button>
          <div class={classes!((*modal_classes).clone())}>
          <div class="modal-background" onclick={toggle_modal.clone()}></div>
          <div class="modal-card">
                <header class="modal-card-head">
                  <p class="modal-card-title">{title}</p>
                  <button class="delete" aria-label="close" onclick={toggle_modal.clone()}></button>
                </header>
                <section class="modal-card-body">
                  {children.clone()}
                </section>
                <footer class="modal-card-foot">
                  <div class="buttons">
                    <button class="button is-success">{"Save changes"}</button>
                    <button class="button" onclick={toggle_modal.clone()}>{"Cancel"}</button>
                  </div>
                </footer>
              </div>
          </div>
        </>
    }
}
