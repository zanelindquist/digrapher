mod app;
mod components;
mod render;
mod logic;
mod router;
mod pages;

use app::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}