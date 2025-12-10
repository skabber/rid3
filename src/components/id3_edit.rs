use gloo::console::log;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[function_component(SelectForm)]
pub fn select_form() -> Html {
    let edit_form = use_state(|| html! {<div></div>});
    let options = vec!["TALB", "TPE1"];

    let edit_form_clone = edit_form.clone();
    let onchange = Callback::from(move |e: Event| {
        let value = e.target_unchecked_into::<HtmlSelectElement>().value();
        log!("Selected value: ", value.clone());
        edit_form_clone.set(html! {
            <div>
                <input type="text" value={value} />
            </div>
        });
    });

    html! {
      <div class={"columns"}>
        <div class={"column"}>
          <select onchange={onchange}>
              { options.into_iter().map(|option| {
                let value = option.to_owned();
                  html! {
                    <option value={value.clone()} >{option}</option>
                  }
              }).collect::<Html>() }
          </select>
        </div>
        <div class={"column"}>
          {<VNode as Clone>::clone(&*(edit_form))}
        </div>
      </div>
    }
}
