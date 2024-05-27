use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Hello, World!"</h1>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Trace);
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
