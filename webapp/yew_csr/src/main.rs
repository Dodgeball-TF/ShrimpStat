#![deny(
    unsafe_code,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
