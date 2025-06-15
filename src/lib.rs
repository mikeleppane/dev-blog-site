pub mod api;
pub mod app;
pub mod components;
pub mod constants;
pub mod models;
pub mod pages;
pub mod services;
pub mod test_blog_code;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
