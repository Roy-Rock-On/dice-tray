use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view!{
    <p>
        "Hello Leptos!"
    </p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}


