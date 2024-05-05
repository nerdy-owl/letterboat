use crate::pages::{
    home::HomePage, letter_view::LetterView, not_found::NotFound, write_message::WriteMessage,
};
//use crate::pages::{home::HomePage, not_found::NotFound, write_message::WriteMessage};
use crate::components::counter::Counter;
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
