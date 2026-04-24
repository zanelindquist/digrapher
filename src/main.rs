mod app;

use app::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}