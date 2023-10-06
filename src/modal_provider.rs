use leptos::*;

use crate::components::store_modal::StoreModal;

#[component]
pub fn ModalProvider(cx: Scope) -> impl IntoView {
    view! { cx,
        <StoreModal/>
    }
}
