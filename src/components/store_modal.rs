use leptos::*;

use crate::{modal::Modal, modal_state::ModalState};

#[component]
pub fn StoreModal(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<ModalState>>(cx).expect("state to have been provided");


    view! { cx,
        <Modal title="Create store".to_string() description="Add a new store".to_string()>
            Future Create Store Form
        </Modal>
        <button on:click=move |_| {
            log!("hi there");
            state.update(|st| st.is_open = true);
        }>Show the dialog</button>
    }
}
