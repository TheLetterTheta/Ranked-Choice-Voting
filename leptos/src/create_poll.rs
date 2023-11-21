use chrono::{DateTime, Days, FixedOffset, Local};
use leptos::*;
use leptos_router::Form;

#[component]
pub fn CreatePollModal() -> impl IntoView {
    let (now, _set_now) = create_signal(Local::now().fixed_offset());

    view! {
        <Form class="font-thin flex-grow flex flex-col gap-2"
            method="post"
            attr:autocomplete="off"
            action="/api/create_poll">
            <h2 class="text-2xl underline font-bold mb-2 capitalize" >"Create a Poll"</h2>
            <label for="create-title">"Title"</label>
            <input class="rounded-md border-2 bg-indigo-50 border-indigo-200 transition duration-75 ease-in focus:border-indigo-400 text-lg p-2"
                required id="create-title" name="title" type="text" />
            <label for="create-description">"Description"</label>
            <textarea class="rounded-md border-2 bg-indigo-50 border-indigo-200 transition duration-75 ease-in focus:border-indigo-400 text-sm p-2"
                rows="2" required id="create-description" name="description" type="text" />
            <fieldset class="border-2 rounded-md p-4 border-indigo-100 flex flex-col gap-2">
                <legend class="px-3">"Expiration"</legend>
                <label for="create-expiration">"Specify Date/Time"</label>
                <input class="rounded-md border-2 bg-indigo-50 border-indigo-200 transition duration-75 ease-in focus:border-indigo-400 p-2"
                        id="create-expiration"
                        min= move || format!("{}", now().format("%Y-%m-%dT%H:%M"))
                        max= move || {
                            let later = now().checked_add_days(Days::new(3))?;
                            Some(format!("{}", later.format("%Y-%m-%dT%H:%M")))
                        }
                        name="expiration"
                        type="datetime-local" />
                <input id="create-timezone"
                    value= move || now().format("%z").to_string() type="hidden" name="timezone" />
                <div class="flex flex-row place-items-center">
                    <hr class="flex-grow" />
                    <p class="mx-2 flex-shrink font-mono uppercase" >"or"</p>
                    <hr class="flex-grow" />
                </div>
                <label for="create-lasts">"Lasts for"</label>
                <select class="p-2 rounded-md border-2 bg-indigo-50 border-indigo-200 transition duration-75 ease-in focus:border-indigo-400 font-bold"
                        id="create-lasts"
                        name="lasts_for">
                    <option selected disabled value>"Select an option"</option>
                    <option value="30">"30 min."</option>
                    <option value="60">"1 hour"</option>
                    <option value="120">"2 hours"</option>
                    <option value="1440">"1 day"</option>
                    <option value="2880">"2 days"</option>
                </select>
            </fieldset>
            <button type="submit" class="text-white rounded-md bg-green-500 p-1" >"Create Poll"</button>
        </Form>
    }
}
