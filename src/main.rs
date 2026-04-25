mod app;
mod components;
mod assets;
mod config;

use app::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}