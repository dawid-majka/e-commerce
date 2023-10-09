pub mod app;
mod components;
mod dark_mode;
mod modal;
mod modal_provider;
mod modal_state;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(move || {
          view! { <App/> }
      });
    }
}
}
