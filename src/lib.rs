// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

pub mod msg;
pub use msg::Msg;

pub mod model;
pub use model::Model;
pub mod app;
pub use app::*;
mod views;

use seed::prelude::*;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    msg.update(model, orders)
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, views::view);
}
