use crate::pages::{home::HomePage, not_found::NotFound, write_message::WriteMessage};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                    <Route path="/write" view=|| view! { <WriteMessage/> }/>
                    <Route path="*" view=|| view! { <NotFound/> }/>
                </Routes>
            </main>
        </Router>
    }
}
