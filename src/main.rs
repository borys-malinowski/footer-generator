use log::Level;
#[path = "components/app/app.rs"]
mod app;
use app::App;

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialize Log!");
    yew::Renderer::<App>::new().render();
}
