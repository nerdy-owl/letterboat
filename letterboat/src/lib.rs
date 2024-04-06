pub mod components;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::*;
    use pages::app::*;

    console_error_panic_hook::set_once();
    mount_to_body(App);
}
