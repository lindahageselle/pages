use zoon::*;

mod aloy_page;
mod app;
mod article;
mod cat_page;
mod crab_page;
pub mod fox_page;
mod header;
mod pond_page;
mod router;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("app", app::root);
}
