use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::store_modal::StoreModal,
    dark_mode::{DarkModeToggle, ToggleDarkMode},
    modal::Modal,
    modal_provider::ModalProvider,
    modal_state::ModalState,
};

// register server functions if we are in ssr mode
#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = ToggleDarkMode::register();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Store Modal State Management
    let modal_state = create_rw_signal(cx, ModalState::default());
    provide_context(cx, modal_state);

    view! { cx,
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
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1 class="p-6 text-4xl">"Welcome to Leptos and Tailwind"</h1>
        <button class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg" on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
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
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
