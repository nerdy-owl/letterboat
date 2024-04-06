use crate::components::Counter;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos11111!"</h1>
        <Counter />
    }
}
