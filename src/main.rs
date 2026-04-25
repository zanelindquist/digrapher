mod app;
mod components;
mod graph;

use app::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}