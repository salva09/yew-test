use yew::prelude::*;
use yew_router::prelude::*;

use crate::screens::{counter::Counter, hello::Hello, home::Home, not_found::NotFound};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
    #[at("/counter")]
    Counter,
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Hello => html! { <Hello /> },
        Route::Counter => html! { <Counter /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
