extern crate yew;

mod app;
mod components;
mod screens;

use app::App;

fn main() {
    yew::start_app::<App>();
}
