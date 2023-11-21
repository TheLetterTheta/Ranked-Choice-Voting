use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};

use crate::create_poll::CreatePollModal;
use crate::poll::Poll;

#[component]
pub fn App() -> impl IntoView {
    // state
    let (modal_active, set_modal_active) = create_signal(false);

    let poll_input: NodeRef<Input> = create_node_ref();

    let search_poll = move |ev: SubmitEvent| {
        // Stop reload
        ev.prevent_default();

        let poll_id = poll_input()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value()
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect::<String>();

        let navigate = leptos_router::use_navigate();
        navigate(&format!("/{}", poll_id), Default::default());
    };

    view! {
        <Router>
            <nav class="flex flex-grow flex-row justify-between bg-indigo-900 p-2 content-center">
                <h1 class="text-white text-4xl">"Ranked Choice Voting"</h1>
                <div class="self-center flex flex-row gap-4">
                    <form on:submit=search_poll class="flex gap-2 self-center">
                        <label for="poll-search-input" class="text-white">"Find a poll"</label>
                        <input type="text"
                            id="poll-search-input"
                            name="poll-id"
                            class="rounded-md border-2 bg-indigo-50 border-indigo-200 transition duration-75 ease-in focus:border-indigo-400"
                            node_ref=poll_input />
                        <button type="submit"></button>
                    </form>
                    <button on:click=move |_| set_modal_active.set(true) class="bg-green-600 rounded-md p-2 text-white" type="button">"Create Poll"</button>
                </div>
            </nav>
            <main>
                <Show when=modal_active>
                    <div on:click=move |_| set_modal_active.set(false)
                        class="body-no-scroll overflow-auto bg-slate-800 bg-opacity-50 fixed top-0 left-0 right-0 bottom-0">
                        <div class="flex w-full h-full place-items-center place-content-center">
                            <div on:click=move |e| e.stop_propagation() class="bg-white top-0 sticky container max-w-lg overflow-auto p-3 ring-indigo-200 hover:ring-2 transition duration-75 ease-in rounded-xl">
                                <CreatePollModal />
                            </div>
                        </div>
                    </div>
                </Show>
                <div class="w-100 md:w-48 lg:w-64">
                    <Routes>
                        <Route path="/:id" view=Poll />
                    </Routes>
                </div>
            </main>
        </Router>
    }
}
