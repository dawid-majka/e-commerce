use leptos::*;

use crate::components::store_modal::StoreModal;

#[component]
pub fn ModalProvider() -> impl IntoView {
    view! {
        <StoreModal/>
    }
}
