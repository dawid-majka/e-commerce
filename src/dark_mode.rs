use leptos::*;
use leptos_meta::Meta;
use leptos_router::ActionForm;

// Scenario:
// 1. User requests the page
// 2. Server checks for cookies
// 3. Render page with color schema - ssr
// 4. Wasm renders in client - checks for cookies, sets color schema
// 4. When button clicked, switch color schema, send call to server

// Called from frontend - executed on backend
// Be careful, this server functions are not protected
#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(cx: Scope, prefers_dark: bool) -> Result<bool, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

    // Creating response and setting cookie with color mode data
    let response = use_context::<ResponseOptions>(cx).expect("to have ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/"))
            .expect("to create hader value"),
    );
    response_parts.headers = headers;
    response.overwrite(response_parts);

    Ok(prefers_dark)
}

// for client side
// check if cookies present in csr side
#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark(cx: Scope) -> bool {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains("darkmode=true")
}

// for server side
// check if cookies present in ssr side
#[cfg(feature = "ssr")]
fn initial_prefers_dark(cx: Scope) -> bool {
    // leptos_actix gives us access to request
    use_context::<actix_web::HttpRequest>(cx)
        .and_then(|req| {
            req.cookies()
                .map(|cookies| {
                    cookies
                        .iter()
                        .any(|cookie| cookie.name() == "darkmode" && cookie.value() == "true")
                })
                .ok()
        })
        .unwrap_or(false)
}

///Renders dark mode button
#[component]
pub fn DarkModeToggle(cx: Scope) -> impl IntoView {
    let initial = initial_prefers_dark(cx);

    let toggle_dark_mode_action = create_server_action::<ToggleDarkMode>(cx);

    // input is `Some(value)` when pending, and `None` if not pending
    let input = toggle_dark_mode_action.input();
    // value contains most recently-returned value
    let value = toggle_dark_mode_action.value();

    let prefers_dark = move || {
        match (input(), value()) {
            // if there's some current input, use that optimistically
            (Some(submission), _) => submission.prefers_dark,
            // otherwise, if there was a previous value confirmed by server, use that
            (_, Some(Ok(value))) => value,
            // otherwise, use the initial value
            _ => initial,
        }
    };

    let color_schema = move || {
        if prefers_dark() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    view! {cx,
        <Meta name="color-scheme" content=color_schema/>
        <ActionForm action=toggle_dark_mode_action>
            <input type="hidden" name="prefers_dark" value=move|| (!prefers_dark()).to_string()/>
            <input type="submit" value =move|| {
                if prefers_dark() {
                    "Switch to light mode"
                }else {
                    "Switch to dark mode"
                }
            }
            />
        </ActionForm>
    }
}
