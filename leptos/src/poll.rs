use leptos::*;
use leptos_router::{use_params, IntoParam, Params};

#[derive(Params, PartialEq, Clone)]
struct PollParams {
    pub id: String,
}

#[component]
pub fn Poll() -> impl IntoView {
    let params = use_params::<PollParams>();

    let id = move || params.with(|p| p.clone().map(|i| i.id).unwrap_or_default());
    view! {
        <p> "So - you're viewing a poll? " {id} </p>
    }
}
