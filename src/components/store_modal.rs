use leptos::{logging::log, *};
use leptos_router::{ActionForm, FromFormData};
use web_sys::SubmitEvent;

use crate::{modal::Modal, modal_state::ModalState};

#[server(CreateStore, "/api")]
pub async fn create_store(name: String) -> Result<(), ServerFnError> {
    println!("Create store api has been called with name: {}", name);

    use actix_web::web::Data;
    use chrono::Utc;
    use leptos_actix::extract;
    use sqlx::PgPool;
    use uuid::Uuid;

    extract(|pool: Data<PgPool>| async move {
        match sqlx::query!(
            r#"
            INSERT INTO stores (id, name, userId, createdAt, updatedAt)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            Uuid::new_v4(),
            name,
            Uuid::new_v4(),
            Utc::now(),
            Utc::now()
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => {
                println!("Store created succesfully");
    Ok(())
}
            Err(e) => {
                println!("Error during store creation");
                Err(ServerFnError::ServerError(e.to_string()))
            }
        }
    })
    .await?
}

#[component]
pub fn StoreModal() -> impl IntoView {
    let state = use_context::<RwSignal<ModalState>>().expect("state to have been provided");

    let create_store = create_server_action::<CreateStore>();
    let input = create_store.input();
    let value = create_store.value();
    let pending = create_store.pending();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let on_submit = move |ev: SubmitEvent| {
        log!("Validation of the form");

        let data = CreateStore::from_event(&ev).expect("to parse form data");

        // TODO: Add validation !!!
        if data.name == " " {
            log!("error");
            ev.prevent_default();
        }
    };

    view! {
        <Modal title="Create store".to_string() description="Add a new store".to_string()>
            <div class="space-y-4 py-2 pb-4">
                <ActionForm action=create_store on:submit=on_submit>
                    <label>
                        <p>"Name"</p>
                        <input disabled=pending type="text" name="name"/>
                    </label>
                    <div class="pt-6 space-x-2 flex items-center justify-end w-full">
                        // <input type="submit" value="Continue"/>
                        <button type="submit" disabled=pending>Continue</button>
                        <button type="button" disabled=pending variant="outline" on:click=move |_ev| { state.update(|state| state.is_open = false)}>Cancel</button>
                    </div>
                </ActionForm>
            </div>
        </Modal>
    }
}
