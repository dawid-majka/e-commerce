use leptos::{logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;

use crate::{
    dark_mode::DarkModeToggle,
    dashboard_layout::{DashboardLayout, Store},
    modal_provider::ModalProvider,
    modal_state::ModalState,
};

// // register server functions if we are in ssr mode
// #[cfg(feature = "ssr")]
// pub fn register_server_functions() {
//     _ = ToggleDarkMode::register();
// }

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // mocked for now
    let user_id = Uuid::parse_str("2a2a1b23-d850-4e02-94df-71cc0e648d9e")
        .expect("Parsing user id is succesfull");
    provide_context(user_id);

    // Store Modal State Management
    let modal_state = create_rw_signal(ModalState::default());

    provide_context(modal_state);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="E-commerce"/>

        // content for this welcome page
        <Router>
        <main class="my-0 mx-auto max-w-3xl text-center">
                <DarkModeToggle/>
                <ModalProvider/>
                <Routes>
                    <Route path="" view=HomePage/>
                     <Route path="/store/:id" view=DashboardLayout/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let user_id = use_context::<Uuid>();

    let navigate = leptos_router::use_navigate();

    match user_id {
        Some(id) => {
            let store =
                create_blocking_resource(|| (), move |_| async move { get_store(id).await });

            create_effect(move |_| match store.get() {
                Some(store) => match store {
                    Ok(store) => {
                        log!("trying to navigate to /store/:{}", store.id);
                        navigate(&format!("/store/{}", store.id), Default::default());
                    }
                    Err(e) => {
                        println!("Error getting store: {e}");
                        let state = use_context::<RwSignal<ModalState>>()
                            .expect("state to have been provided");
                        state.update(|state| state.is_open = true);
                    }
                },
                None => {
                    let state =
                        use_context::<RwSignal<ModalState>>().expect("state to have been provided");
                    state.update(|state| state.is_open = true);
                }
            });
            return view! {};
        }
        None => {
            navigate("/sign-in", Default::default());
        }
    }

    view! {}
}

#[server(GetStore, "/api")]
pub async fn get_store(user_id: Uuid) -> Result<Store, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use sqlx::PgPool;

    println!("Get store has been called with userId: {}", user_id);

    let data = extract(move |pool: Data<PgPool>| async move {
        match sqlx::query_as::<_, Store>(&format!(
            r#"
            SELECT id, name, userid, createdat, updatedat
            FROM stores
            WHERE userId = $1
            "#,
        ))
        .bind(user_id)
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
