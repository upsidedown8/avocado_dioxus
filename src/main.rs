#![allow(non_snake_case)]

mod app;
mod board;
mod square;

fn main() {
    dioxus::web::launch_cfg(app::App, |c| c.rootname("main"))
}
