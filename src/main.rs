mod app;
mod components;
mod render;
mod services;
mod router;
mod pages;

use app::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}