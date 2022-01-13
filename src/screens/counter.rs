use yew::prelude::*;

use crate::components::button_counter::ButtonCounter;

#[function_component(Counter)]
pub fn counter() -> Html {
    html! {
        <div>
            <h1>{"Click the button to start counting!"}</h1>
            <ButtonCounter />
        </div>
    }
}
