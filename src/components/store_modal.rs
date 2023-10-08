use leptos::*;
use leptos_router::{ActionForm, FromFormData};
use web_sys::SubmitEvent;

use crate::{modal::Modal, modal_state::ModalState};

#[server(CreateStore, "/api")]
pub async fn create_store(name: String) -> Result<(), ServerFnError> {
    println!("Create store api has been called with name: {}", name);
    Ok(())
}

#[component]
pub fn StoreModal(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<ModalState>>(cx).expect("state to have been provided");

    let create_store = create_server_action::<CreateStore>(cx);
    let on_submit = move |ev: SubmitEvent| {
        log!("Validation of the form");

        let data = CreateStore::from_event(&ev).expect("to parse form data");

        // TODO: Add validation !!!
        if data.name == " " {
            log!("error");
            ev.prevent_default();
        }
    };

    view! { cx,
        <Modal title="Create store".to_string() description="Add a new store".to_string()>
            <div class="space-y-4 py-2 pb-4">
                <ActionForm action=create_store on:submit=on_submit>
                    <label>
                        <p>"Name"</p>
                        <input type="text" name="name"/>
                    </label>
                    <div class="pt-6 space-x-2 flex items-center justify-end w-full">
                        // <input type="submit" value="Continue"/>
                        <button type="submit">Continue</button>
                        <button type="button" variant="outline" on:click=move |_ev| { state.update(|state| state.is_open = false)}>Cancel</button>
                    </div>
                </ActionForm>
            </div>
        </Modal>
    }
}
