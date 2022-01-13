use yew::prelude::*;

#[function_component(ButtonCounter)]
pub fn button_counter() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <button onclick={onclick}>{"Click Me!"}</button>
            <p>{"Count: "} {*counter}</p>
        </div>
    }
}
