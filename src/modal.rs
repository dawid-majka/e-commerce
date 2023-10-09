use leptos::{html::Dialog, *};

use crate::modal_state::ModalState;

///Renders modal
#[component]
pub fn Modal(children: Children, title: String, description: String) -> impl IntoView {
    let dialog_ref = create_node_ref::<Dialog>();

    let state = use_context::<RwSignal<ModalState>>().expect("state to have been provided");
    let is_open = move || state.get().is_open;

    create_effect(move |_| {
        if let Some(dialog) = dialog_ref.get() {
            if is_open() {
                _ = dialog.show_modal();
            } else {
                dialog.close();
            }
        }
    });

    view! {
            <dialog node_ref=dialog_ref>
            <div class="dialog_content">
                <div class="dialog_header">
                    <div class="dialog_title">{title}</div>
                    <div class="dialog_description">{description}</div>
                </div>
                <div>
                    {children()}
                </div>
            </div>
            </dialog>







    }
}
