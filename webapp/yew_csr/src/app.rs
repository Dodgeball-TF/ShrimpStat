use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // this is a htmx project.
    // Start the page blank with a button that does a htmx-get to /hello

    html! {
        <div>
            <button class="btn btn-primary" hx-get="/hello" hx-target="#hello" >{"Say Hello"}</button>
            <div id="hello"></div>
        </div>
    }
}
