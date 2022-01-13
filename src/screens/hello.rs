use yew::prelude::*;

use crate::components::hello::HelloWorld;

#[function_component(Hello)]
pub fn hello() -> Html {
    html! {
        <div>
            <HelloWorld />
        </div>
    }
}
