use leptos::*;

mod app;
mod create_poll;
use crate::app::App;

mod poll;

fn main() {
    mount_to_body(|| view! { <App /> })
}
