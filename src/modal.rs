use leptos::{html::Dialog, *};

///Renders modal
#[component]
pub fn Modal(cx: Scope) -> impl IntoView {
    let dialog_ref = create_node_ref::<Dialog>(cx);

    let (dialog_open, set_dialog_open) = create_signal(cx, false);

    create_effect(cx, move |_| {
        if let Some(dialog) = dialog_ref.get() {
            if dialog_open() {
                _ = dialog.show_modal();
            } else {
                dialog.close();
            }
        }
    });

    let dialog_view = move || match dialog_open() {
        true => view! {cx, <p>This modal is working</p>},
        false => view! {cx, <p></p>},
    };

    view! {cx,
            <dialog node_ref=dialog_ref>
                <button autofocus on:click=move|_| {
                    log!("closing");
                    set_dialog_open.update(|dialog_status| *dialog_status = false);
                }>
                    Close
                </button>
                {dialog_view}
            </dialog>
            <button on:click=move |_| {
                log!("hi there");
                set_dialog_open.update(|dialog_status| *dialog_status = true);
            }>
                Show the dialog
            </button>
    }
}
