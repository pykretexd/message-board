mod components;
use components::app::App;
use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App /> })
}
