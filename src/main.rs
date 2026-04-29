mod app;
mod assets;
mod components;
mod render;
mod logic;

use app::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}