mod app;
mod rummy;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
