use leptos::{logging::log, *};
use leptos_router::*;

use chrono::{DateTime, Utc};
use leptos_router::use_params_map;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::navbar::Navbar;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Store {
    pub id: Uuid,
    pub name: String,
    pub userid: Uuid,
    pub createdat: DateTime<Utc>,
    pub updatedat: DateTime<Utc>,
}

#[server(GetStore, "/api")]
pub async fn get_store(user_id: Uuid, store_id: String) -> Result<Store, ServerFnError> {
    println!("Get store api has been called");

    use actix_web::web::Data;
    use chrono::Utc;
    use leptos_actix::extract;
    use sqlx::PgPool;
    use uuid::Uuid;

    let store_id = match Uuid::parse_str(&store_id) {
        Ok(id) => id,
        Err(e) => {
            return Err(ServerFnError::Args(
                "Error parsing store_id to Uuid.".to_string(),
            ))
        }
    };

    println!(
        "Get store has been called with userId: {} and store_id: {}",
        user_id, store_id
    );

    let data = extract(move |pool: Data<PgPool>| async move {
        match sqlx::query_as::<_, Store>(&format!(
            r#"
            SELECT id, name, userid, createdat, updatedat
            FROM stores
            WHERE userId = $1 AND id = $2
            "#,
        ))
        .bind(user_id)
        .bind(store_id)
        .fetch_one(pool.get_ref())
        .await
        {
            Ok(row) => {
                println!("store found: {:?}", row);
                Ok(row)
            }
            Err(e) => {
                println!("no store found, error: {}", e.to_string());
                Err(ServerFnError::ServerError(e.to_string()))
            }
        }
    })
    .await;

    data?
}

#[component]
pub fn DashboardLayout() -> impl IntoView {
    let user_id = use_context::<Uuid>();

    println!("In dashboard layout");

    match user_id {
        Some(id) => {
            println!("userid: {}", id);

            let params = use_params_map();

            let store = create_blocking_resource(
                move || {
                    let store_id = params().get("id").cloned().unwrap_or_default();
                    println!("store_id: {}", store_id);
                    store_id
                },
                move |store_id| async move { get_store(id, store_id).await },
            );

            return view! {
                <div>
                    <Transition fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        {move || {
                            store
                                .get()
                                .map(|store| match store {
                                    Err(e) => {
                                        log!("Error: {}", e);
                                        view! { <p>No store found</p> }
                                            .into_view()
                                    }
                                    Ok(store) => {
                                        view! { <p>Store: {store.name} </p>}
                                            .into_view()
                                    }
                                })
                        }}
                    </Transition>
                </div>
            };
        }
        None => {
            let navigate = leptos_router::use_navigate();
            navigate("/sign-in", Default::default());
        }
    }

    view! {<div></div>}
}
