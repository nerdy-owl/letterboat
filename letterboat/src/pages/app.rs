use crate::pages::{home::HomePage, not_found::NotFound, write_message::WriteMessage, letter_view::LetterView};
//use crate::pages::{home::HomePage, not_found::NotFound, write_message::WriteMessage};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/write" view=|| view! { <WriteMessage/> }/>
                    <Route path="/letterview" view=LetterView/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
